use async_trait::async_trait;

use crate::{InferenceRequest, InferenceResult, UseCaseError};

#[async_trait]
pub trait InferenceEngine: Send + Sync {
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResult, UseCaseError>;
}
