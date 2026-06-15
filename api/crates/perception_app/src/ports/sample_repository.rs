use async_trait::async_trait;
use perception_domain::SampleId;

use crate::{SampleDraft, UseCaseError};

#[async_trait]
pub trait SampleRepository: Send + Sync {
    async fn create(&self, sample: SampleDraft) -> Result<SampleDraft, UseCaseError>;

    async fn get(&self, sample_id: SampleId) -> Result<Option<SampleDraft>, UseCaseError>;
}
