use async_trait::async_trait;
use perception_domain::{DatasetId, SampleId};

use crate::{AnnotationDraft, UseCaseError};

#[async_trait]
pub trait AnnotationRepository: Send + Sync {
    async fn create(&self, annotation: AnnotationDraft) -> Result<AnnotationDraft, UseCaseError>;

    async fn list_by_sample(
        &self,
        sample_id: SampleId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError>;

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError>;
}
