use async_trait::async_trait;

use crate::{DatasetDraft, UseCaseError};

#[async_trait]
pub trait DatasetRepository: Send + Sync {
    async fn create(&self, dataset: DatasetDraft) -> Result<DatasetDraft, UseCaseError>;
}
