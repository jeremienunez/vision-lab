use std::collections::BTreeMap;

use perception_domain::{AnnotationId, DatasetId, NormalizedBbox};

use crate::{
    AnnotationDraft, AnnotationRepository, DatasetDraft, DatasetRepository, SampleDraft,
    SampleRepository, UseCaseError, YoloAnnotationImportFile, YoloAnnotationImportResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportYoloAnnotationsCommand {
    pub dataset_id: DatasetId,
    pub files: Vec<YoloAnnotationImportFile>,
}

pub struct ImportYoloAnnotationsUseCase<'repository> {
    dataset_repository: &'repository dyn DatasetRepository,
    sample_repository: &'repository dyn SampleRepository,
    annotation_repository: &'repository dyn AnnotationRepository,
}

impl<'repository> ImportYoloAnnotationsUseCase<'repository> {
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
        command: ImportYoloAnnotationsCommand,
    ) -> Result<YoloAnnotationImportResult, UseCaseError> {
        let dataset = self
            .dataset_repository
            .get(command.dataset_id)
            .await?
            .ok_or(UseCaseError::NotFound("dataset not found"))?;
        let samples_by_filename = self.samples_by_filename(command.dataset_id).await?;
        let annotations =
            annotations_from_yolo_files(&dataset, &samples_by_filename, command.files)?;
        let imported_count = annotations.len();

        for annotation in annotations {
            self.annotation_repository.create(annotation).await?;
        }

        Ok(YoloAnnotationImportResult {
            dataset_id: command.dataset_id,
            imported_count,
        })
    }

    async fn samples_by_filename(
        &self,
        dataset_id: DatasetId,
    ) -> Result<BTreeMap<String, SampleDraft>, UseCaseError> {
        Ok(self
            .sample_repository
            .list_by_dataset(dataset_id)
            .await?
            .into_iter()
            .map(|sample| (sample.filename.clone(), sample))
            .collect())
    }
}

fn annotations_from_yolo_files(
    dataset: &DatasetDraft,
    samples_by_filename: &BTreeMap<String, SampleDraft>,
    files: Vec<YoloAnnotationImportFile>,
) -> Result<Vec<AnnotationDraft>, UseCaseError> {
    let mut annotations = Vec::new();

    for file in files {
        let sample = samples_by_filename
            .get(&file.sample_filename)
            .ok_or(UseCaseError::NotFound("sample not found"))?;

        for line in file
            .content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            annotations.push(annotation_from_yolo_line(dataset, sample, line)?);
        }
    }

    Ok(annotations)
}

fn annotation_from_yolo_line(
    dataset: &DatasetDraft,
    sample: &SampleDraft,
    line: &str,
) -> Result<AnnotationDraft, UseCaseError> {
    let mut parts = line.split_whitespace();
    let class_id = parse_class_id(parts.next())?;
    let x_center = parse_coordinate(parts.next())?;
    let y_center = parse_coordinate(parts.next())?;
    let width = parse_coordinate(parts.next())?;
    let height = parse_coordinate(parts.next())?;

    if parts.next().is_some() {
        return Err(UseCaseError::Validation("invalid yolo annotation line"));
    }

    let class_name = dataset
        .classes
        .get(class_id as usize)
        .ok_or(UseCaseError::Validation("unknown yolo class id"))?;
    let bbox = NormalizedBbox::new(
        x_center - width / 2.0,
        y_center - height / 2.0,
        width,
        height,
    )
    .map_err(|_| UseCaseError::Validation("invalid normalized bbox"))?;

    Ok(AnnotationDraft {
        id: AnnotationId::new(),
        sample_id: sample.id,
        dataset_id: dataset.id,
        class_name: class_name.clone(),
        class_id,
        bbox_x: bbox.x,
        bbox_y: bbox.y,
        bbox_width: bbox.width,
        bbox_height: bbox.height,
        format: "normalized_xywh".to_owned(),
        confidence: None,
        source: "yolo_import".to_owned(),
    })
}

fn parse_class_id(value: Option<&str>) -> Result<u32, UseCaseError> {
    value
        .ok_or(UseCaseError::Validation("invalid yolo annotation line"))?
        .parse()
        .map_err(|_| UseCaseError::Validation("invalid yolo class id"))
}

fn parse_coordinate(value: Option<&str>) -> Result<f32, UseCaseError> {
    value
        .ok_or(UseCaseError::Validation("invalid yolo annotation line"))?
        .parse()
        .map_err(|_| UseCaseError::Validation("invalid yolo numeric value"))
}
