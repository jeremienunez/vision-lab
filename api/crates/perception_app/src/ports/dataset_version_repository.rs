use async_trait::async_trait;
use perception_domain::{DatasetId, DatasetVersionId};

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

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<DatasetVersionDraft>, UseCaseError>;
}
