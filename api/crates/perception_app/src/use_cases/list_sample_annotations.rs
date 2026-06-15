use perception_domain::SampleId;

use crate::{AnnotationDraft, AnnotationRepository, SampleRepository, UseCaseError};

pub struct ListSampleAnnotationsUseCase<'repository> {
    sample_repository: &'repository dyn SampleRepository,
    annotation_repository: &'repository dyn AnnotationRepository,
}

impl<'repository> ListSampleAnnotationsUseCase<'repository> {
    pub fn new(
        sample_repository: &'repository dyn SampleRepository,
        annotation_repository: &'repository dyn AnnotationRepository,
    ) -> Self {
        Self {
            sample_repository,
            annotation_repository,
        }
    }

    pub async fn execute(
        &self,
        sample_id: SampleId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError> {
        if self.sample_repository.get(sample_id).await?.is_none() {
            return Err(UseCaseError::NotFound("sample not found"));
        }

        self.annotation_repository.list_by_sample(sample_id).await
    }
}
