use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{SampleDraft, SampleRepository, UseCaseError};
use perception_domain::{DatasetId, SampleId};

#[derive(Default)]
pub struct TransientSampleRepository {
    samples: RwLock<Vec<SampleDraft>>,
}

#[async_trait]
impl SampleRepository for TransientSampleRepository {
    async fn create(&self, sample: SampleDraft) -> Result<SampleDraft, UseCaseError> {
        self.samples
            .write()
            .map_err(|_| UseCaseError::Repository("sample repository lock poisoned"))?
            .push(sample.clone());

        Ok(sample)
    }

    async fn get(&self, sample_id: SampleId) -> Result<Option<SampleDraft>, UseCaseError> {
        self.samples
            .read()
            .map(|samples| {
                samples
                    .iter()
                    .find(|sample| sample.id == sample_id)
                    .cloned()
            })
            .map_err(|_| UseCaseError::Repository("sample repository lock poisoned"))
    }

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<SampleDraft>, UseCaseError> {
        self.samples
            .read()
            .map(|samples| {
                samples
                    .iter()
                    .filter(|sample| sample.dataset_id == dataset_id)
                    .cloned()
                    .collect()
            })
            .map_err(|_| UseCaseError::Repository("sample repository lock poisoned"))
    }
}
