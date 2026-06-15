use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct YoloAnnotationImportRequest {
    pub files: Vec<YoloAnnotationImportFileRequest>,
}

#[derive(Debug, Deserialize)]
pub struct YoloAnnotationImportFileRequest {
    pub sample_filename: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct YoloAnnotationImportResponse {
    pub dataset_id: String,
    pub imported_count: usize,
}
