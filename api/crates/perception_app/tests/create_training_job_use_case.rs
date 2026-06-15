use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    CreateTrainingJobCommand, CreateTrainingJobUseCase, DatasetVersionDraft,
    DatasetVersionRepository, TrainingJobDraft, TrainingJobRepository, UseCaseError,
};
use perception_domain::{DatasetId, DatasetVersionId, TrainingJobId, TrainingJobStatus};

#[derive(Default)]
struct InMemoryDatasetVersionRepository {
    versions: Mutex<Vec<DatasetVersionDraft>>,
}

#[async_trait]
impl DatasetVersionRepository for InMemoryDatasetVersionRepository {
    async fn create(
        &self,
        version: DatasetVersionDraft,
    ) -> Result<DatasetVersionDraft, UseCaseError> {
        self.versions
            .lock()
            .expect("repository mutex is available")
            .push(version.clone());
        Ok(version)
    }

    async fn get(
        &self,
        version_id: DatasetVersionId,
    ) -> Result<Option<DatasetVersionDraft>, UseCaseError> {
        Ok(self
            .versions
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|version| version.id == version_id)
            .cloned())
    }
}

#[derive(Default)]
struct InMemoryTrainingJobRepository {
    jobs: Mutex<Vec<TrainingJobDraft>>,
}

#[async_trait]
impl TrainingJobRepository for InMemoryTrainingJobRepository {
    async fn create(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        self.jobs
            .lock()
            .expect("repository mutex is available")
            .push(job.clone());
        Ok(job)
    }

    async fn get(&self, job_id: TrainingJobId) -> Result<Option<TrainingJobDraft>, UseCaseError> {
        Ok(self
            .jobs
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|job| job.id == job_id)
            .cloned())
    }
}

fn version_fixture() -> DatasetVersionDraft {
    DatasetVersionDraft {
        id: DatasetVersionId::new(),
        dataset_id: DatasetId::new(),
        version_name: "v1".to_owned(),
        sample_count: 1,
        annotation_count: 1,
        classes_snapshot: vec!["cup".to_owned()],
        split_config: BTreeMap::new(),
        created_by: "local-user".to_owned(),
    }
}

#[tokio::test]
async fn create_training_job_queues_job_for_existing_dataset_version() {
    let versions = InMemoryDatasetVersionRepository::default();
    let jobs = InMemoryTrainingJobRepository::default();
    let version = versions
        .create(version_fixture())
        .await
        .expect("version is created");

    let job = CreateTrainingJobUseCase::new(&versions, &jobs)
        .execute(CreateTrainingJobCommand {
            dataset_version_id: version.id,
            model_family: "yolo".to_owned(),
            base_model: Some("yolo11n".to_owned()),
            epochs: 5,
            batch_size: 2,
            image_size: 640,
            learning_rate: 0.001,
        })
        .await
        .expect("job is created");

    assert_eq!(job.dataset_version_id, version.id);
    assert_eq!(job.model_family, "yolo");
    assert_eq!(job.base_model, Some("yolo11n".to_owned()));
    assert_eq!(job.status, TrainingJobStatus::Queued);
    assert_eq!(job.hyperparameters.epochs, 5);
}

#[tokio::test]
async fn create_training_job_rejects_missing_dataset_version() {
    let versions = InMemoryDatasetVersionRepository::default();
    let jobs = InMemoryTrainingJobRepository::default();

    let result = CreateTrainingJobUseCase::new(&versions, &jobs)
        .execute(CreateTrainingJobCommand {
            dataset_version_id: DatasetVersionId::new(),
            model_family: "yolo".to_owned(),
            base_model: Some("yolo11n".to_owned()),
            epochs: 5,
            batch_size: 2,
            image_size: 640,
            learning_rate: 0.001,
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::NotFound("dataset version not found"))
    );
}
