use async_trait::async_trait;
use perception_domain::ModelId;

use crate::{ModelDraft, UseCaseError};

#[async_trait]
pub trait ModelRepository: Send + Sync {
    async fn create(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError>;

    async fn list(&self) -> Result<Vec<ModelDraft>, UseCaseError>;

    async fn get(&self, model_id: ModelId) -> Result<Option<ModelDraft>, UseCaseError>;

    async fn update(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError>;
}
