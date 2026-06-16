use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    CompareModelsCommand, CompareModelsUseCase, ModelDraft, ModelRepository, UseCaseError,
};
use perception_domain::{DatasetVersionId, ModelId, ModelStatus, TrainingJobId};

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

    async fn update(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        let mut models = self.models.lock().expect("repository mutex is available");
        let stored = models
            .iter_mut()
            .find(|stored_model| stored_model.id == model.id)
            .ok_or(UseCaseError::NotFound("model not found"))?;
        *stored = model.clone();
        Ok(model)
    }
}

fn model_fixture(name: &str, metric_name: &str, metric_value: &str) -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: name.to_owned(),
        version: "v1".to_owned(),
        training_job_id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "tiny_torch".to_owned(),
        artifact_uri: format!("file:///tmp/{name}.pt"),
        metrics_summary: BTreeMap::from([(metric_name.to_owned(), metric_value.to_owned())]),
        status: ModelStatus::Candidate,
    }
}

#[tokio::test]
async fn compare_models_ranks_by_highest_metric_value() {
    let models = InMemoryModelRepository::default();
    let first = models
        .create(model_fixture("baseline", "mAP50", "0.73"))
        .await
        .expect("model is created");
    let second = models
        .create(model_fixture("challenger", "mAP50", "0.81"))
        .await
        .expect("model is created");

    let comparison = CompareModelsUseCase::new(&models)
        .execute(CompareModelsCommand {
            model_ids: vec![first.id, second.id],
            metric_name: "mAP50".to_owned(),
        })
        .await
        .expect("models are compared");

    assert_eq!(comparison.metric_name, "mAP50");
    assert_eq!(comparison.direction, "higher_is_better");
    assert_eq!(comparison.best_model_id, second.id);
    assert_eq!(comparison.models[0].model_id, second.id);
    assert_eq!(comparison.models[0].rank, 1);
    assert_eq!(comparison.models[0].metric_value, 0.81);
    assert_eq!(comparison.models[1].model_id, first.id);
    assert_eq!(comparison.models[1].rank, 2);
}

#[tokio::test]
async fn compare_models_rejects_single_model() {
    let models = InMemoryModelRepository::default();
    let model = models
        .create(model_fixture("baseline", "mAP50", "0.73"))
        .await
        .expect("model is created");

    let result = CompareModelsUseCase::new(&models)
        .execute(CompareModelsCommand {
            model_ids: vec![model.id],
            metric_name: "mAP50".to_owned(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation(
            "model comparison requires at least two models"
        ))
    );
}

#[tokio::test]
async fn compare_models_rejects_non_comparable_metrics() {
    let models = InMemoryModelRepository::default();
    let first = models
        .create(model_fixture("baseline", "mAP50", "0.73"))
        .await
        .expect("model is created");
    let second = models
        .create(model_fixture("challenger", "train_loss", "0.21"))
        .await
        .expect("model is created");

    let result = CompareModelsUseCase::new(&models)
        .execute(CompareModelsCommand {
            model_ids: vec![first.id, second.id],
            metric_name: "mAP50".to_owned(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation(
            "models must have comparable metric"
        ))
    );
}
