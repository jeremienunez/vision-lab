use async_trait::async_trait;
use perception_domain::TrainingJobId;

use crate::{TrainingJobDraft, UseCaseError};

#[async_trait]
pub trait TrainingJobRepository: Send + Sync {
    async fn create(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError>;

    async fn list(&self) -> Result<Vec<TrainingJobDraft>, UseCaseError>;

    async fn get(&self, job_id: TrainingJobId) -> Result<Option<TrainingJobDraft>, UseCaseError>;

    async fn update(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError>;
}
