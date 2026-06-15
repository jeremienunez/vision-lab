use perception_domain::{TrainingJobId, TrainingJobStatus};

use crate::{TrainingJobDraft, TrainingJobRepository, UseCaseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionTrainingJobCommand {
    pub job_id: TrainingJobId,
    pub next_status: TrainingJobStatus,
    pub error_message: Option<String>,
}

pub struct TransitionTrainingJobUseCase<'repository> {
    training_job_repository: &'repository dyn TrainingJobRepository,
}

impl<'repository> TransitionTrainingJobUseCase<'repository> {
    pub fn new(training_job_repository: &'repository dyn TrainingJobRepository) -> Self {
        Self {
            training_job_repository,
        }
    }

    pub async fn execute(
        &self,
        command: TransitionTrainingJobCommand,
    ) -> Result<TrainingJobDraft, UseCaseError> {
        let mut job = self
            .training_job_repository
            .get(command.job_id)
            .await?
            .ok_or(UseCaseError::NotFound("training job not found"))?;

        job.status = job
            .status
            .transition_to(command.next_status)
            .map_err(|_| UseCaseError::Validation("invalid training job status transition"))?;

        if job.status == TrainingJobStatus::Failed {
            let error_message = command
                .error_message
                .filter(|message| !message.trim().is_empty())
                .ok_or(UseCaseError::Validation(
                    "failed training job requires error message",
                ))?;
            job.error_message = Some(error_message);
        }

        self.training_job_repository.update(job).await
    }
}
