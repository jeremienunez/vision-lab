use async_trait::async_trait;

use perception_domain::DatasetId;

use crate::{DatasetDraft, UseCaseError};

#[async_trait]
pub trait DatasetRepository: Send + Sync {
    async fn create(&self, dataset: DatasetDraft) -> Result<DatasetDraft, UseCaseError>;

    async fn get(&self, dataset_id: DatasetId) -> Result<Option<DatasetDraft>, UseCaseError>;

    async fn list(&self) -> Result<Vec<DatasetDraft>, UseCaseError>;
}
