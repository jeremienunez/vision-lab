use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{TrainingMetricDraft, TrainingMetricRepository, UseCaseError};
use perception_domain::TrainingJobId;

#[derive(Default)]
pub struct TransientTrainingMetricRepository {
    metrics: RwLock<Vec<TrainingMetricDraft>>,
}

#[async_trait]
impl TrainingMetricRepository for TransientTrainingMetricRepository {
    async fn create(
        &self,
        metric: TrainingMetricDraft,
    ) -> Result<TrainingMetricDraft, UseCaseError> {
        self.metrics
            .write()
            .map_err(|_| UseCaseError::Repository("training metric repository lock poisoned"))?
            .push(metric.clone());

        Ok(metric)
    }

    async fn list_by_training_job(
        &self,
        training_job_id: TrainingJobId,
    ) -> Result<Vec<TrainingMetricDraft>, UseCaseError> {
        let mut metrics = self
            .metrics
            .read()
            .map_err(|_| UseCaseError::Repository("training metric repository lock poisoned"))?
            .iter()
            .filter(|metric| metric.training_job_id == training_job_id)
            .cloned()
            .collect::<Vec<_>>();
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
