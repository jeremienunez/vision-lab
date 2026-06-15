use perception_domain::TrainingJobId;

use crate::{TrainingJobRepository, TrainingMetricDraft, TrainingMetricRepository, UseCaseError};

pub struct ListTrainingMetricsUseCase<'repository> {
    training_job_repository: &'repository dyn TrainingJobRepository,
    training_metric_repository: &'repository dyn TrainingMetricRepository,
}

impl<'repository> ListTrainingMetricsUseCase<'repository> {
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
        training_job_id: TrainingJobId,
    ) -> Result<Vec<TrainingMetricDraft>, UseCaseError> {
        self.training_job_repository
            .get(training_job_id)
            .await?
            .ok_or(UseCaseError::NotFound("training job not found"))?;

        let mut metrics = self
            .training_metric_repository
            .list_by_training_job(training_job_id)
            .await?;
        metrics.sort_by_key(|metric| {
            (
                metric.epoch.unwrap_or(u32::MAX),
                metric.step.unwrap_or(u32::MAX),
                metric.metric_name.clone(),
            )
        });
        Ok(metrics)
    }
}
