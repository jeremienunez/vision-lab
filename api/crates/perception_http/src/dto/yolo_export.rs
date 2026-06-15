use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct YoloAnnotationExportResponse {
    pub dataset_id: String,
    pub classes_txt: String,
    pub files: Vec<YoloAnnotationFileResponse>,
}

#[derive(Debug, Serialize)]
pub struct YoloAnnotationFileResponse {
    pub path: String,
    pub content: String,
}
