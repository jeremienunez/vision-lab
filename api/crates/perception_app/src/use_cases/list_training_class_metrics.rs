use perception_domain::TrainingJobId;

use crate::{TrainingClassMetric, TrainingJobRepository, TrainingMetricRepository, UseCaseError};

pub struct ListTrainingClassMetricsUseCase<'repository> {
    training_job_repository: &'repository dyn TrainingJobRepository,
    training_metric_repository: &'repository dyn TrainingMetricRepository,
}

impl<'repository> ListTrainingClassMetricsUseCase<'repository> {
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
    ) -> Result<Vec<TrainingClassMetric>, UseCaseError> {
        self.training_job_repository
            .get(training_job_id)
            .await?
            .ok_or(UseCaseError::NotFound("training job not found"))?;

        let mut class_metrics = self
            .training_metric_repository
            .list_by_training_job(training_job_id)
            .await?
            .into_iter()
            .filter_map(|metric| {
                let class_name = metric.metadata.get("class_name")?.trim().to_owned();

                if class_name.is_empty() {
                    return None;
                }

                Some(TrainingClassMetric {
                    training_job_id: metric.training_job_id,
                    class_name,
                    split_name: metric.split_name,
                    metric_name: metric.metric_name,
                    metric_value: metric.metric_value,
                    step: metric.step,
                    epoch: metric.epoch,
                })
            })
            .collect::<Vec<_>>();

        class_metrics.sort_by_key(|metric| {
            (
                metric.class_name.clone(),
                metric.epoch.unwrap_or(u32::MAX),
                metric.step.unwrap_or(u32::MAX),
                metric.metric_name.clone(),
            )
        });

        Ok(class_metrics)
    }
}
