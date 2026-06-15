use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    GetModelUseCase, ListModelsUseCase, ModelDraft, ModelRepository, RegisterModelCommand,
    RegisterModelUseCase, TrainingJobDraft, TrainingJobRepository, UseCaseError,
};
use perception_domain::{
    DatasetVersionId, ModelId, ModelStatus, TrainingHyperparameters, TrainingJobId,
    TrainingJobStatus,
};

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

#[derive(Default)]
struct InMemoryModelRepository {
    models: Mutex<Vec<ModelDraft>>,
}

#[async_trait]
impl ModelRepository for InMemoryModelRepository {
    async fn create(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        self.models
            .lock()
            .expect("repository mutex is available")
            .push(model.clone());
        Ok(model)
    }

    async fn list(&self) -> Result<Vec<ModelDraft>, UseCaseError> {
        Ok(self
            .models
            .lock()
            .expect("repository mutex is available")
            .clone())
    }

    async fn get(&self, model_id: ModelId) -> Result<Option<ModelDraft>, UseCaseError> {
        Ok(self
            .models
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|model| model.id == model_id)
            .cloned())
    }
}

fn training_job_fixture(status: TrainingJobStatus) -> TrainingJobDraft {
    TrainingJobDraft {
        id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "tiny_torch".to_owned(),
        base_model: None,
        status,
        hyperparameters: TrainingHyperparameters::new(2, 1, 64, 0.01)
            .expect("hyperparameters are valid"),
        error_message: None,
    }
}

#[tokio::test]
async fn register_model_creates_candidate_model_for_succeeded_training_job() {
    let jobs = InMemoryTrainingJobRepository::default();
    let models = InMemoryModelRepository::default();
    let job = jobs
        .create(training_job_fixture(TrainingJobStatus::Succeeded))
        .await
        .expect("job is created");

    let model = RegisterModelUseCase::new(&jobs, &models)
        .execute(RegisterModelCommand {
            training_job_id: job.id,
            name: "desk-objects".to_owned(),
            version: "v1".to_owned(),
            artifact_uri: "file:///tmp/model.pt".to_owned(),
            metrics_summary: BTreeMap::from([("train_loss".to_owned(), "0.32".to_owned())]),
        })
        .await
        .expect("model is registered");

    assert_eq!(model.training_job_id, job.id);
    assert_eq!(model.dataset_version_id, job.dataset_version_id);
    assert_eq!(model.model_family, "tiny_torch");
    assert_eq!(model.artifact_uri, "file:///tmp/model.pt");
    assert_eq!(model.status, ModelStatus::Candidate);
}

#[tokio::test]
async fn register_model_rejects_non_succeeded_training_job() {
    let jobs = InMemoryTrainingJobRepository::default();
    let models = InMemoryModelRepository::default();
    let job = jobs
        .create(training_job_fixture(TrainingJobStatus::Failed))
        .await
        .expect("job is created");

    let result = RegisterModelUseCase::new(&jobs, &models)
        .execute(RegisterModelCommand {
            training_job_id: job.id,
            name: "desk-objects".to_owned(),
            version: "v1".to_owned(),
            artifact_uri: "file:///tmp/model.pt".to_owned(),
            metrics_summary: BTreeMap::new(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation(
            "model requires a succeeded training job"
        ))
    );
    assert!(
        ListModelsUseCase::new(&models)
            .execute()
            .await
            .expect("models are listed")
            .is_empty()
    );
}

#[tokio::test]
async fn list_and_get_models_return_registered_models() {
    let jobs = InMemoryTrainingJobRepository::default();
    let models = InMemoryModelRepository::default();
    let job = jobs
        .create(training_job_fixture(TrainingJobStatus::Succeeded))
        .await
        .expect("job is created");
    let registered = RegisterModelUseCase::new(&jobs, &models)
        .execute(RegisterModelCommand {
            training_job_id: job.id,
            name: "desk-objects".to_owned(),
            version: "v1".to_owned(),
            artifact_uri: "file:///tmp/model.pt".to_owned(),
            metrics_summary: BTreeMap::new(),
        })
        .await
        .expect("model is registered");

    let listed = ListModelsUseCase::new(&models)
        .execute()
        .await
        .expect("models are listed");
    let fetched = GetModelUseCase::new(&models)
        .execute(registered.id)
        .await
        .expect("model is fetched");

    assert_eq!(listed, vec![registered.clone()]);
    assert_eq!(fetched, registered);
}
