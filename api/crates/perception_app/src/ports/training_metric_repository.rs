use async_trait::async_trait;
use perception_domain::TrainingJobId;

use crate::{TrainingMetricDraft, UseCaseError};

#[async_trait]
pub trait TrainingMetricRepository: Send + Sync {
    async fn create(
        &self,
        metric: TrainingMetricDraft,
    ) -> Result<TrainingMetricDraft, UseCaseError>;

    async fn list_by_training_job(
        &self,
        training_job_id: TrainingJobId,
    ) -> Result<Vec<TrainingMetricDraft>, UseCaseError>;
}
