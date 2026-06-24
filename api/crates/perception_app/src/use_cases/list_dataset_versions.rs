use perception_domain::DatasetId;

use crate::{DatasetRepository, DatasetVersionDraft, DatasetVersionRepository, UseCaseError};

pub struct ListDatasetVersionsUseCase<'repository> {
    dataset_repository: &'repository dyn DatasetRepository,
    dataset_version_repository: &'repository dyn DatasetVersionRepository,
}

impl<'repository> ListDatasetVersionsUseCase<'repository> {
    pub fn new(
        dataset_repository: &'repository dyn DatasetRepository,
        dataset_version_repository: &'repository dyn DatasetVersionRepository,
    ) -> Self {
        Self {
            dataset_repository,
            dataset_version_repository,
        }
    }

    pub async fn execute(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<DatasetVersionDraft>, UseCaseError> {
        self.dataset_repository
            .get(dataset_id)
            .await?
            .ok_or(UseCaseError::NotFound("dataset not found"))?;

        self.dataset_version_repository
            .list_by_dataset(dataset_id)
            .await
    }
}
