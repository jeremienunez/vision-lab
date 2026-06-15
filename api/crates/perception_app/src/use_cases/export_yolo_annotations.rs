use perception_domain::DatasetId;

use crate::{
    AnnotationDraft, AnnotationRepository, DatasetRepository, SampleDraft, SampleRepository,
    UseCaseError, YoloAnnotationExport, YoloAnnotationFile,
};

pub struct ExportYoloAnnotationsUseCase<'repository> {
    dataset_repository: &'repository dyn DatasetRepository,
    sample_repository: &'repository dyn SampleRepository,
    annotation_repository: &'repository dyn AnnotationRepository,
}

impl<'repository> ExportYoloAnnotationsUseCase<'repository> {
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
        dataset_id: DatasetId,
    ) -> Result<YoloAnnotationExport, UseCaseError> {
        let dataset = self
            .dataset_repository
            .get(dataset_id)
            .await?
            .ok_or(UseCaseError::NotFound("dataset not found"))?;
        let mut samples = self.sample_repository.list_by_dataset(dataset_id).await?;
        let annotations = self
            .annotation_repository
            .list_by_dataset(dataset_id)
            .await?;

        samples.sort_by(|left, right| left.filename.cmp(&right.filename));

        Ok(YoloAnnotationExport {
            dataset_id,
            classes_txt: format!("{}\n", dataset.classes.join("\n")),
            files: samples
                .into_iter()
                .map(|sample| yolo_file_for_sample(sample, &annotations))
                .collect(),
        })
    }
}

fn yolo_file_for_sample(
    sample: SampleDraft,
    annotations: &[AnnotationDraft],
) -> YoloAnnotationFile {
    let mut sample_annotations = annotations
        .iter()
        .filter(|annotation| annotation.sample_id == sample.id)
        .cloned()
        .collect::<Vec<_>>();
    sample_annotations.sort_by_key(|annotation| (annotation.class_id, annotation.id.to_string()));

    YoloAnnotationFile {
        path: format!("labels/{}.txt", label_stem(&sample.filename)),
        content: sample_annotations
            .iter()
            .map(yolo_line)
            .collect::<Vec<_>>()
            .join(""),
    }
}

fn yolo_line(annotation: &AnnotationDraft) -> String {
    let x_center = annotation.bbox_x + annotation.bbox_width / 2.0;
    let y_center = annotation.bbox_y + annotation.bbox_height / 2.0;

    format!(
        "{} {:.6} {:.6} {:.6} {:.6}\n",
        annotation.class_id, x_center, y_center, annotation.bbox_width, annotation.bbox_height,
    )
}

fn label_stem(filename: &str) -> String {
    filename
        .rsplit_once('.')
        .map_or(filename, |(stem, _)| stem)
        .to_owned()
}
