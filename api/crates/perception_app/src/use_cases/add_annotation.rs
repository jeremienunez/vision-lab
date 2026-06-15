use perception_domain::{AnnotationId, NormalizedBbox, SampleId};

use crate::{
    AnnotationDraft, AnnotationRepository, DatasetRepository, SampleRepository, UseCaseError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AddAnnotationCommand {
    pub sample_id: SampleId,
    pub class_name: String,
    pub bbox_x: f32,
    pub bbox_y: f32,
    pub bbox_width: f32,
    pub bbox_height: f32,
    pub confidence: Option<f32>,
}

pub struct AddAnnotationUseCase<'repository> {
    dataset_repository: &'repository dyn DatasetRepository,
    sample_repository: &'repository dyn SampleRepository,
    annotation_repository: &'repository dyn AnnotationRepository,
}

impl<'repository> AddAnnotationUseCase<'repository> {
    pub fn new(
        dataset_repository: &'repository dyn DatasetRepository,
        sample_repository: &'repository dyn SampleRepository,
        annotation_repository: &'repository dyn AnnotationRepository,
    ) -> Self {
        Self {
            dataset_repository,
            sample_repository,
            annotation_repository,
        }
    }

    pub async fn execute(
        &self,
        command: AddAnnotationCommand,
    ) -> Result<AnnotationDraft, UseCaseError> {
        let sample = self
            .sample_repository
            .get(command.sample_id)
            .await?
            .ok_or(UseCaseError::NotFound("sample not found"))?;
        let dataset = self
            .dataset_repository
            .get(sample.dataset_id)
            .await?
            .ok_or(UseCaseError::NotFound("dataset not found"))?;
        let class_id = dataset
            .classes
            .iter()
            .position(|class_name| class_name == &command.class_name)
            .ok_or(UseCaseError::Validation("unknown dataset class"))?
            as u32;
        let bbox = NormalizedBbox::new(
            command.bbox_x,
            command.bbox_y,
            command.bbox_width,
            command.bbox_height,
        )
        .map_err(|_| UseCaseError::Validation("invalid normalized bbox"))?;

        if let Some(confidence) = command.confidence
            && !(0.0..=1.0).contains(&confidence)
        {
            return Err(UseCaseError::Validation("invalid annotation confidence"));
        }

        self.annotation_repository
            .create(AnnotationDraft {
                id: AnnotationId::new(),
                sample_id: sample.id,
                dataset_id: sample.dataset_id,
                class_name: command.class_name,
                class_id,
                bbox_x: bbox.x,
                bbox_y: bbox.y,
                bbox_width: bbox.width,
                bbox_height: bbox.height,
                format: "normalized_xywh".to_owned(),
                confidence: command.confidence,
                source: "manual".to_owned(),
            })
            .await
    }
}
