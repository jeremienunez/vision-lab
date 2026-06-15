use std::collections::BTreeMap;

use perception_domain::{TrainingJobId, TrainingMetricId};

use crate::{TrainingJobRepository, TrainingMetricDraft, TrainingMetricRepository, UseCaseError};

#[derive(Debug, Clone, PartialEq)]
pub struct RecordTrainingMetricCommand {
    pub training_job_id: TrainingJobId,
    pub split_name: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub step: Option<u32>,
    pub epoch: Option<u32>,
    pub metadata: BTreeMap<String, String>,
}

pub struct RecordTrainingMetricUseCase<'repository> {
    training_job_repository: &'repository dyn TrainingJobRepository,
    training_metric_repository: &'repository dyn TrainingMetricRepository,
}

impl<'repository> RecordTrainingMetricUseCase<'repository> {
    pub fn new(
        training_job_repository: &'repository dyn TrainingJobRepository,
        training_metric_repository: &'repository dyn TrainingMetricRepository,
    ) -> Self {
        Self {
            training_job_repository,
            training_metric_repository,
        }
    }

    pub async fn execute(
        &self,
        command: RecordTrainingMetricCommand,
    ) -> Result<TrainingMetricDraft, UseCaseError> {
        if !is_valid_metric_contract(&command) {
            return Err(UseCaseError::Validation("invalid training metric"));
        }

        self.training_job_repository
            .get(command.training_job_id)
            .await?
            .ok_or(UseCaseError::NotFound("training job not found"))?;

        self.training_metric_repository
            .create(TrainingMetricDraft {
                id: TrainingMetricId::new(),
                training_job_id: command.training_job_id,
                split_name: command.split_name,
                metric_name: command.metric_name.trim().to_owned(),
                metric_value: command.metric_value,
                step: command.step,
                epoch: command.epoch,
                metadata: command.metadata,
            })
            .await
    }
}

fn is_valid_metric_contract(command: &RecordTrainingMetricCommand) -> bool {
    matches!(command.split_name.as_str(), "train" | "validation" | "test")
        && !command.metric_name.trim().is_empty()
        && command.metric_value.is_finite()
}
