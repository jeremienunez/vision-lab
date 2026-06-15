use std::collections::BTreeMap;

use perception_domain::{ModelId, ModelStatus, TrainingJobId, TrainingJobStatus};

use crate::{ModelDraft, ModelRepository, TrainingJobRepository, UseCaseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterModelCommand {
    pub training_job_id: TrainingJobId,
    pub name: String,
    pub version: String,
    pub artifact_uri: String,
    pub metrics_summary: BTreeMap<String, String>,
}

pub struct RegisterModelUseCase<'repository> {
    training_job_repository: &'repository dyn TrainingJobRepository,
    model_repository: &'repository dyn ModelRepository,
}

impl<'repository> RegisterModelUseCase<'repository> {
    pub fn new(
        training_job_repository: &'repository dyn TrainingJobRepository,
        model_repository: &'repository dyn ModelRepository,
    ) -> Self {
        Self {
            training_job_repository,
            model_repository,
        }
    }

    pub async fn execute(&self, command: RegisterModelCommand) -> Result<ModelDraft, UseCaseError> {
        if command.name.trim().is_empty()
            || command.version.trim().is_empty()
            || command.artifact_uri.trim().is_empty()
        {
            return Err(UseCaseError::Validation("invalid model registration"));
        }

        let training_job = self
            .training_job_repository
            .get(command.training_job_id)
            .await?
            .ok_or(UseCaseError::NotFound("training job not found"))?;

        if training_job.status != TrainingJobStatus::Succeeded {
            return Err(UseCaseError::Validation(
                "model requires a succeeded training job",
            ));
        }

        self.model_repository
            .create(ModelDraft {
                id: ModelId::new(),
                name: command.name.trim().to_owned(),
                version: command.version.trim().to_owned(),
                training_job_id: training_job.id,
                dataset_version_id: training_job.dataset_version_id,
                model_family: training_job.model_family,
                artifact_uri: command.artifact_uri.trim().to_owned(),
                metrics_summary: command.metrics_summary,
                status: ModelStatus::Candidate,
            })
            .await
    }
}
