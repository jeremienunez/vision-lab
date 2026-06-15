use async_trait::async_trait;
use perception_domain::ModelId;

use crate::{ModelExportDraft, UseCaseError};

#[async_trait]
pub trait ModelExportRepository: Send + Sync {
    async fn create(&self, export: ModelExportDraft) -> Result<ModelExportDraft, UseCaseError>;

    async fn list_by_model(&self, model_id: ModelId)
    -> Result<Vec<ModelExportDraft>, UseCaseError>;
}
