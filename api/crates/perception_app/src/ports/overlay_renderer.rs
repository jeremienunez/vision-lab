use async_trait::async_trait;

use crate::{InferenceRunDraft, OverlayArtifact, UseCaseError};

#[async_trait]
pub trait OverlayRenderer: Send + Sync {
    async fn render(&self, run: InferenceRunDraft) -> Result<OverlayArtifact, UseCaseError>;
}
