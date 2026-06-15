use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{SampleDraft, SampleRepository, UseCaseError};

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
}
