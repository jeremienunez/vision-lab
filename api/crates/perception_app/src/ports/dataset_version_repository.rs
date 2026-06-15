use async_trait::async_trait;
use perception_domain::DatasetVersionId;

use crate::{DatasetVersionDraft, UseCaseError};

#[async_trait]
pub trait DatasetVersionRepository: Send + Sync {
    async fn create(
        &self,
        version: DatasetVersionDraft,
    ) -> Result<DatasetVersionDraft, UseCaseError>;

    async fn get(
        &self,
        version_id: DatasetVersionId,
    ) -> Result<Option<DatasetVersionDraft>, UseCaseError>;
}
