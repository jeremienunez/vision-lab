use perception_app::{YoloAnnotationExport, YoloAnnotationFile};

use crate::dto::yolo_export::{YoloAnnotationExportResponse, YoloAnnotationFileResponse};

pub fn yolo_annotation_export_response(
    export: YoloAnnotationExport,
) -> YoloAnnotationExportResponse {
    YoloAnnotationExportResponse {
        dataset_id: export.dataset_id.to_string(),
        classes_txt: export.classes_txt,
        files: export
            .files
            .into_iter()
            .map(yolo_annotation_file_response)
            .collect(),
    }
}

fn yolo_annotation_file_response(file: YoloAnnotationFile) -> YoloAnnotationFileResponse {
    YoloAnnotationFileResponse {
        path: file.path,
        content: file.content,
    }
}
