use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{
    TrainingJobQueue, TrainingJobQueueEntry, TrainingJobQueueStatus, UseCaseError,
};

#[derive(Default)]
pub struct TransientTrainingJobQueue {
    entries: RwLock<Vec<TrainingJobQueueEntry>>,
}

#[async_trait]
impl TrainingJobQueue for TransientTrainingJobQueue {
    async fn enqueue(
        &self,
        entry: TrainingJobQueueEntry,
    ) -> Result<TrainingJobQueueEntry, UseCaseError> {
        self.entries
            .write()
            .map_err(|_| UseCaseError::Repository("training job queue lock poisoned"))?
            .push(entry.clone());

        Ok(entry)
    }

    async fn lease_next(
        &self,
        worker_id: String,
    ) -> Result<Option<TrainingJobQueueEntry>, UseCaseError> {
        let mut entries = self
            .entries
            .write()
            .map_err(|_| UseCaseError::Repository("training job queue lock poisoned"))?;
        let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.status == TrainingJobQueueStatus::Queued)
        else {
            return Ok(None);
        };

        entry.status = TrainingJobQueueStatus::Leased;
        entry.locked_by = Some(worker_id);
        entry.attempts += 1;

        Ok(Some(entry.clone()))
    }
}
