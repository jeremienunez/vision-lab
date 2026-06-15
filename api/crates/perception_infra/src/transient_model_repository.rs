use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{ModelDraft, ModelRepository, UseCaseError};
use perception_domain::ModelId;

#[derive(Default)]
pub struct TransientModelRepository {
    models: RwLock<Vec<ModelDraft>>,
}

#[async_trait]
impl ModelRepository for TransientModelRepository {
    async fn create(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        self.models
            .write()
            .map_err(|_| UseCaseError::Repository("model repository lock poisoned"))?
            .push(model.clone());

        Ok(model)
    }

    async fn list(&self) -> Result<Vec<ModelDraft>, UseCaseError> {
        self.models
            .read()
            .map(|models| models.clone())
            .map_err(|_| UseCaseError::Repository("model repository lock poisoned"))
    }

    async fn get(&self, model_id: ModelId) -> Result<Option<ModelDraft>, UseCaseError> {
        self.models
            .read()
            .map(|models| models.iter().find(|model| model.id == model_id).cloned())
            .map_err(|_| UseCaseError::Repository("model repository lock poisoned"))
    }
}
