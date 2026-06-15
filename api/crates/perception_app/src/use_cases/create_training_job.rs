use perception_domain::{
    DatasetVersionId, TrainingHyperparameters, TrainingJobId, TrainingJobStatus,
};

use crate::{DatasetVersionRepository, TrainingJobDraft, TrainingJobRepository, UseCaseError};

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
}

impl<'repository> CreateTrainingJobUseCase<'repository> {
    pub fn new(
        dataset_version_repository: &'repository dyn DatasetVersionRepository,
        training_job_repository: &'repository dyn TrainingJobRepository,
    ) -> Self {
        Self {
            dataset_version_repository,
            training_job_repository,
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

        self.training_job_repository
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
            .await
    }
}
