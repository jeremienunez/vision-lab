use crate::{TrainingJobDraft, TrainingJobRepository, UseCaseError};

pub struct ListTrainingJobsUseCase<'repository> {
    training_job_repository: &'repository dyn TrainingJobRepository,
}

impl<'repository> ListTrainingJobsUseCase<'repository> {
    pub fn new(training_job_repository: &'repository dyn TrainingJobRepository) -> Self {
        Self {
            training_job_repository,
        }
    }

    pub async fn execute(&self) -> Result<Vec<TrainingJobDraft>, UseCaseError> {
        self.training_job_repository.list().await
    }
}
