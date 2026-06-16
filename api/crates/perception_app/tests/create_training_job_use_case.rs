use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    CreateTrainingJobCommand, CreateTrainingJobUseCase, DatasetVersionDraft,
    DatasetVersionRepository, ListTrainingJobsUseCase, TrainingJobDraft, TrainingJobQueue,
    TrainingJobQueueEntry, TrainingJobQueueStatus, TrainingJobRepository,
    TransitionTrainingJobCommand, TransitionTrainingJobUseCase, UseCaseError,
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

#[derive(Default)]
struct InMemoryTrainingJobQueue {
    entries: Mutex<Vec<TrainingJobQueueEntry>>,
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

    async fn list(&self) -> Result<Vec<TrainingJobDraft>, UseCaseError> {
        Ok(self
            .jobs
            .lock()
            .expect("repository mutex is available")
            .clone())
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

    async fn update(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        let mut jobs = self.jobs.lock().expect("repository mutex is available");
        let stored = jobs
            .iter_mut()
            .find(|stored_job| stored_job.id == job.id)
            .ok_or(UseCaseError::NotFound("training job not found"))?;
        *stored = job.clone();
        Ok(job)
    }
}

#[async_trait]
impl TrainingJobQueue for InMemoryTrainingJobQueue {
    async fn enqueue(
        &self,
        entry: TrainingJobQueueEntry,
    ) -> Result<TrainingJobQueueEntry, UseCaseError> {
        self.entries
            .lock()
            .expect("queue mutex is available")
            .push(entry.clone());
        Ok(entry)
    }

    async fn lease_next(
        &self,
        worker_id: String,
    ) -> Result<Option<TrainingJobQueueEntry>, UseCaseError> {
        let mut entries = self.entries.lock().expect("queue mutex is available");
        let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.status == TrainingJobQueueStatus::Queued)
        else {
            return Ok(None);
        };

        entry.status = TrainingJobQueueStatus::Leased;
        entry.locked_by = Some(worker_id);
        entry.attempts += 1;
        Ok(Some(entry.clone()))
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
    let queue = InMemoryTrainingJobQueue::default();
    let version = versions
        .create(version_fixture())
        .await
        .expect("version is created");

    let job = CreateTrainingJobUseCase::new_with_queue(&versions, &jobs, &queue)
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

    let leased = queue
        .lease_next("worker-1".to_owned())
        .await
        .expect("queue lease succeeds")
        .expect("queue has job");

    assert_eq!(leased.training_job_id, job.id);
    assert_eq!(leased.status, TrainingJobQueueStatus::Leased);
    assert_eq!(leased.locked_by, Some("worker-1".to_owned()));
    assert_eq!(leased.attempts, 1);
}

#[tokio::test]
async fn list_training_jobs_returns_repository_jobs() {
    let versions = InMemoryDatasetVersionRepository::default();
    let jobs = InMemoryTrainingJobRepository::default();
    let version = versions
        .create(version_fixture())
        .await
        .expect("version is created");
    let created = CreateTrainingJobUseCase::new(&versions, &jobs)
        .execute(CreateTrainingJobCommand {
            dataset_version_id: version.id,
            model_family: "tiny_torch".to_owned(),
            base_model: None,
            epochs: 2,
            batch_size: 1,
            image_size: 64,
            learning_rate: 0.01,
        })
        .await
        .expect("job is created");

    let listed = ListTrainingJobsUseCase::new(&jobs)
        .execute()
        .await
        .expect("jobs list succeeds");

    assert_eq!(listed, vec![created]);
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

#[tokio::test]
async fn transition_training_job_persists_valid_status_progression() {
    let versions = InMemoryDatasetVersionRepository::default();
    let jobs = InMemoryTrainingJobRepository::default();
    let version = versions
        .create(version_fixture())
        .await
        .expect("version is created");
    let created = CreateTrainingJobUseCase::new(&versions, &jobs)
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

    let running = TransitionTrainingJobUseCase::new(&jobs)
        .execute(TransitionTrainingJobCommand {
            job_id: created.id,
            next_status: TrainingJobStatus::Running,
            error_message: None,
        })
        .await
        .expect("job moves to running");
    let failed = TransitionTrainingJobUseCase::new(&jobs)
        .execute(TransitionTrainingJobCommand {
            job_id: running.id,
            next_status: TrainingJobStatus::Failed,
            error_message: Some("training crashed".to_owned()),
        })
        .await
        .expect("job moves to failed");

    assert_eq!(failed.status, TrainingJobStatus::Failed);
    assert_eq!(failed.error_message, Some("training crashed".to_owned()));
    assert_eq!(
        jobs.get(created.id)
            .await
            .expect("job lookup succeeds")
            .expect("job exists")
            .status,
        TrainingJobStatus::Failed
    );
}

#[tokio::test]
async fn transition_training_job_rejects_invalid_status_progression() {
    let versions = InMemoryDatasetVersionRepository::default();
    let jobs = InMemoryTrainingJobRepository::default();
    let version = versions
        .create(version_fixture())
        .await
        .expect("version is created");
    let created = CreateTrainingJobUseCase::new(&versions, &jobs)
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

    let result = TransitionTrainingJobUseCase::new(&jobs)
        .execute(TransitionTrainingJobCommand {
            job_id: created.id,
            next_status: TrainingJobStatus::Succeeded,
            error_message: None,
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation(
            "invalid training job status transition"
        ))
    );
}
