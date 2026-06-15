use async_trait::async_trait;
use perception_domain::InferenceRunId;

use crate::{InferenceRunDraft, UseCaseError};

#[async_trait]
pub trait InferenceRunRepository: Send + Sync {
    async fn create(&self, run: InferenceRunDraft) -> Result<InferenceRunDraft, UseCaseError>;

    async fn get(&self, run_id: InferenceRunId) -> Result<Option<InferenceRunDraft>, UseCaseError>;
}
