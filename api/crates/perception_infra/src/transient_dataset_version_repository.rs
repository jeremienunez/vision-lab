use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{DatasetVersionDraft, DatasetVersionRepository, UseCaseError};
use perception_domain::DatasetVersionId;

#[derive(Default)]
pub struct TransientDatasetVersionRepository {
    versions: RwLock<Vec<DatasetVersionDraft>>,
}

#[async_trait]
impl DatasetVersionRepository for TransientDatasetVersionRepository {
    async fn create(
        &self,
        version: DatasetVersionDraft,
    ) -> Result<DatasetVersionDraft, UseCaseError> {
        self.versions
            .write()
            .map_err(|_| UseCaseError::Repository("dataset version repository lock poisoned"))?
            .push(version.clone());

        Ok(version)
    }

    async fn get(
        &self,
        version_id: DatasetVersionId,
    ) -> Result<Option<DatasetVersionDraft>, UseCaseError> {
        self.versions
            .read()
            .map(|versions| {
                versions
                    .iter()
                    .find(|version| version.id == version_id)
                    .cloned()
            })
            .map_err(|_| UseCaseError::Repository("dataset version repository lock poisoned"))
    }
}
