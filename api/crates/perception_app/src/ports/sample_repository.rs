use async_trait::async_trait;

use crate::{SampleDraft, UseCaseError};

#[async_trait]
pub trait SampleRepository: Send + Sync {
    async fn create(&self, sample: SampleDraft) -> Result<SampleDraft, UseCaseError>;
}
