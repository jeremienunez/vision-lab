use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{InferenceRunDraft, InferenceRunRepository, UseCaseError};
use perception_domain::InferenceRunId;

#[derive(Default)]
pub struct TransientInferenceRunRepository {
    runs: RwLock<Vec<InferenceRunDraft>>,
}

#[async_trait]
impl InferenceRunRepository for TransientInferenceRunRepository {
    async fn create(&self, run: InferenceRunDraft) -> Result<InferenceRunDraft, UseCaseError> {
        self.runs
            .write()
            .map_err(|_| UseCaseError::Repository("inference run repository lock poisoned"))?
            .push(run.clone());

        Ok(run)
    }

    async fn get(&self, run_id: InferenceRunId) -> Result<Option<InferenceRunDraft>, UseCaseError> {
        self.runs
            .read()
            .map(|runs| runs.iter().find(|run| run.id == run_id).cloned())
            .map_err(|_| UseCaseError::Repository("inference run repository lock poisoned"))
    }
}
