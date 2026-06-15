use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateModelExportRequest {
    pub format: String,
}

#[derive(Debug, Serialize)]
pub struct ModelExportResponse {
    pub id: String,
    pub model_id: String,
    pub format: String,
    pub artifact_uri: Option<String>,
    pub status: &'static str,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListModelExportsResponse {
    pub exports: Vec<ModelExportResponse>,
}
