use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    ModelDraft, ModelRepository, PromoteModelCommand, PromoteModelUseCase, UseCaseError,
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

fn model_fixture(
    name: &str,
    dataset_version_id: DatasetVersionId,
    model_family: &str,
    status: ModelStatus,
) -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: name.to_owned(),
        version: "v1".to_owned(),
        training_job_id: TrainingJobId::new(),
        dataset_version_id,
        model_family: model_family.to_owned(),
        artifact_uri: format!("file:///tmp/{name}.pt"),
        metrics_summary: BTreeMap::new(),
        status,
    }
}

#[tokio::test]
async fn promote_model_is_exclusive_for_dataset_version_and_family() {
    let models = InMemoryModelRepository::default();
    let dataset_version_id = DatasetVersionId::new();
    let baseline = models
        .create(model_fixture(
            "baseline",
            dataset_version_id,
            "tiny_torch",
            ModelStatus::Promoted,
        ))
        .await
        .expect("baseline is created");
    let challenger = models
        .create(model_fixture(
            "challenger",
            dataset_version_id,
            "tiny_torch",
            ModelStatus::Validated,
        ))
        .await
        .expect("challenger is created");
    let other_family = models
        .create(model_fixture(
            "mobile",
            dataset_version_id,
            "mobile_net",
            ModelStatus::Promoted,
        ))
        .await
        .expect("other family model is created");

    let promoted = PromoteModelUseCase::new(&models)
        .execute(PromoteModelCommand {
            model_id: challenger.id,
        })
        .await
        .expect("model is promoted");

    assert_eq!(promoted.id, challenger.id);
    assert_eq!(promoted.status, ModelStatus::Promoted);

    let stored_models = models.list().await.expect("models are listed");
    assert_eq!(
        stored_models
            .iter()
            .find(|model| model.id == baseline.id)
            .expect("baseline is stored")
            .status,
        ModelStatus::Validated
    );
    assert_eq!(
        stored_models
            .iter()
            .find(|model| model.id == challenger.id)
            .expect("challenger is stored")
            .status,
        ModelStatus::Promoted
    );
    assert_eq!(
        stored_models
            .iter()
            .find(|model| model.id == other_family.id)
            .expect("other family model is stored")
            .status,
        ModelStatus::Promoted
    );
}

#[tokio::test]
async fn promote_model_rejects_archived_model() {
    let models = InMemoryModelRepository::default();
    let model = models
        .create(model_fixture(
            "archived",
            DatasetVersionId::new(),
            "tiny_torch",
            ModelStatus::Archived,
        ))
        .await
        .expect("model is created");

    let result = PromoteModelUseCase::new(&models)
        .execute(PromoteModelCommand { model_id: model.id })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation(
            "archived model cannot be promoted"
        ))
    );
}
