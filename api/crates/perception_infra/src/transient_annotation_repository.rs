use std::sync::RwLock;

use async_trait::async_trait;
use perception_app::{AnnotationDraft, AnnotationRepository, UseCaseError};
use perception_domain::{DatasetId, SampleId};

#[derive(Default)]
pub struct TransientAnnotationRepository {
    annotations: RwLock<Vec<AnnotationDraft>>,
}

#[async_trait]
impl AnnotationRepository for TransientAnnotationRepository {
    async fn create(&self, annotation: AnnotationDraft) -> Result<AnnotationDraft, UseCaseError> {
        self.annotations
            .write()
            .map_err(|_| UseCaseError::Repository("annotation repository lock poisoned"))?
            .push(annotation.clone());

        Ok(annotation)
    }

    async fn list_by_sample(
        &self,
        sample_id: SampleId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError> {
        self.annotations
            .read()
            .map(|annotations| {
                annotations
                    .iter()
                    .filter(|annotation| annotation.sample_id == sample_id)
                    .cloned()
                    .collect()
            })
            .map_err(|_| UseCaseError::Repository("annotation repository lock poisoned"))
    }

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError> {
        self.annotations
            .read()
            .map(|annotations| {
                annotations
                    .iter()
                    .filter(|annotation| annotation.dataset_id == dataset_id)
                    .cloned()
                    .collect()
            })
            .map_err(|_| UseCaseError::Repository("annotation repository lock poisoned"))
    }
}
