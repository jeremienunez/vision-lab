use async_trait::async_trait;

use crate::{DatasetDraft, UseCaseError};

#[async_trait]
pub trait DatasetRepository: Send + Sync {
    async fn create(&self, dataset: DatasetDraft) -> Result<DatasetDraft, UseCaseError>;

    async fn list(&self) -> Result<Vec<DatasetDraft>, UseCaseError>;
}
