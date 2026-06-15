use perception_domain::{
    DatasetVersionId, TrainingHyperparameters, TrainingJobId, TrainingJobStatus,
};

use crate::{
    DatasetVersionRepository, TrainingJobDraft, TrainingJobQueue, TrainingJobQueueEntry,
    TrainingJobRepository, UseCaseError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTrainingJobCommand {
    pub dataset_version_id: DatasetVersionId,
    pub model_family: String,
    pub base_model: Option<String>,
    pub epochs: u16,
    pub batch_size: u16,
    pub image_size: u16,
    pub learning_rate: f32,
}

pub struct CreateTrainingJobUseCase<'repository> {
    dataset_version_repository: &'repository dyn DatasetVersionRepository,
    training_job_repository: &'repository dyn TrainingJobRepository,
    training_job_queue: Option<&'repository dyn TrainingJobQueue>,
}

impl<'repository> CreateTrainingJobUseCase<'repository> {
    pub fn new(
        dataset_version_repository: &'repository dyn DatasetVersionRepository,
        training_job_repository: &'repository dyn TrainingJobRepository,
    ) -> Self {
        Self {
            dataset_version_repository,
            training_job_repository,
            training_job_queue: None,
        }
    }

    pub fn new_with_queue(
        dataset_version_repository: &'repository dyn DatasetVersionRepository,
        training_job_repository: &'repository dyn TrainingJobRepository,
        training_job_queue: &'repository dyn TrainingJobQueue,
    ) -> Self {
        Self {
            dataset_version_repository,
            training_job_repository,
            training_job_queue: Some(training_job_queue),
        }
    }

    pub async fn execute(
        &self,
        command: CreateTrainingJobCommand,
    ) -> Result<TrainingJobDraft, UseCaseError> {
        if self
            .dataset_version_repository
            .get(command.dataset_version_id)
            .await?
            .is_none()
        {
            return Err(UseCaseError::NotFound("dataset version not found"));
        }

        if command.model_family.trim().is_empty() {
            return Err(UseCaseError::Validation("model family is required"));
        }

        let hyperparameters = TrainingHyperparameters::new(
            command.epochs,
            command.batch_size,
            command.image_size,
            command.learning_rate,
        )
        .map_err(|_| UseCaseError::Validation("invalid training hyperparameters"))?;

        let job = self
            .training_job_repository
            .create(TrainingJobDraft {
                id: TrainingJobId::new(),
                dataset_version_id: command.dataset_version_id,
                model_family: command.model_family.trim().to_owned(),
                base_model: command
                    .base_model
                    .map(|base_model| base_model.trim().to_owned())
                    .filter(|base_model| !base_model.is_empty()),
                status: TrainingJobStatus::Queued,
                hyperparameters,
                error_message: None,
            })
            .await?;

        if let Some(training_job_queue) = self.training_job_queue {
            training_job_queue
                .enqueue(TrainingJobQueueEntry::queued(job.id))
                .await?;
        }

        Ok(job)
    }
}
