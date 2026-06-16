use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{TrainingJobDraft, TrainingJobRepository, UseCaseError};
use perception_domain::TrainingJobId;

#[derive(Default)]
pub struct TransientTrainingJobRepository {
    jobs: RwLock<Vec<TrainingJobDraft>>,
}

#[async_trait]
impl TrainingJobRepository for TransientTrainingJobRepository {
    async fn create(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        self.jobs
            .write()
            .map_err(|_| UseCaseError::Repository("training job repository lock poisoned"))?
            .push(job.clone());

        Ok(job)
    }

    async fn list(&self) -> Result<Vec<TrainingJobDraft>, UseCaseError> {
        self.jobs
            .read()
            .map(|jobs| jobs.clone())
            .map_err(|_| UseCaseError::Repository("training job repository lock poisoned"))
    }

    async fn get(&self, job_id: TrainingJobId) -> Result<Option<TrainingJobDraft>, UseCaseError> {
        self.jobs
            .read()
            .map(|jobs| jobs.iter().find(|job| job.id == job_id).cloned())
            .map_err(|_| UseCaseError::Repository("training job repository lock poisoned"))
    }

    async fn update(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        let mut jobs = self
            .jobs
            .write()
            .map_err(|_| UseCaseError::Repository("training job repository lock poisoned"))?;
        let stored = jobs
            .iter_mut()
            .find(|stored_job| stored_job.id == job.id)
            .ok_or(UseCaseError::NotFound("training job not found"))?;

        *stored = job.clone();
        Ok(job)
    }
}
