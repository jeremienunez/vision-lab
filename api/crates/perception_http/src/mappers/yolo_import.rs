use perception_app::{
    ImportYoloAnnotationsCommand, YoloAnnotationImportFile, YoloAnnotationImportResult,
};
use perception_domain::DatasetId;

use crate::dto::yolo_import::{YoloAnnotationImportRequest, YoloAnnotationImportResponse};

pub fn yolo_annotation_import_command(
    dataset_id: DatasetId,
    request: YoloAnnotationImportRequest,
) -> ImportYoloAnnotationsCommand {
    ImportYoloAnnotationsCommand {
        dataset_id,
        files: request
            .files
            .into_iter()
            .map(|file| YoloAnnotationImportFile {
                sample_filename: file.sample_filename,
                content: file.content,
            })
            .collect(),
    }
}

pub fn yolo_annotation_import_response(
    result: YoloAnnotationImportResult,
) -> YoloAnnotationImportResponse {
    YoloAnnotationImportResponse {
        dataset_id: result.dataset_id.to_string(),
        imported_count: result.imported_count,
    }
}
