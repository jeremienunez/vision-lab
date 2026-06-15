use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{DatasetDraft, DatasetRepository, UseCaseError};
use perception_domain::DatasetId;

#[derive(Default)]
pub struct TransientDatasetRepository {
    datasets: RwLock<Vec<DatasetDraft>>,
}

#[async_trait]
impl DatasetRepository for TransientDatasetRepository {
    async fn create(&self, dataset: DatasetDraft) -> Result<DatasetDraft, UseCaseError> {
        self.datasets
            .write()
            .map_err(|_| UseCaseError::Repository("dataset repository lock poisoned"))?
            .push(dataset.clone());

        Ok(dataset)
    }

    async fn get(&self, dataset_id: DatasetId) -> Result<Option<DatasetDraft>, UseCaseError> {
        self.datasets
            .read()
            .map(|datasets| {
                datasets
                    .iter()
                    .find(|dataset| dataset.id == dataset_id)
                    .cloned()
            })
            .map_err(|_| UseCaseError::Repository("dataset repository lock poisoned"))
    }

    async fn list(&self) -> Result<Vec<DatasetDraft>, UseCaseError> {
        self.datasets
            .read()
            .map(|datasets| datasets.clone())
            .map_err(|_| UseCaseError::Repository("dataset repository lock poisoned"))
    }
}
