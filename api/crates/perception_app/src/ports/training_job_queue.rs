use async_trait::async_trait;

use crate::{TrainingJobQueueEntry, UseCaseError};

#[async_trait]
pub trait TrainingJobQueue: Send + Sync {
    async fn enqueue(
        &self,
        entry: TrainingJobQueueEntry,
    ) -> Result<TrainingJobQueueEntry, UseCaseError>;

    async fn lease_next(
        &self,
        worker_id: String,
    ) -> Result<Option<TrainingJobQueueEntry>, UseCaseError>;
}
