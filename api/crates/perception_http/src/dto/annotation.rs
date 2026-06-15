use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddAnnotationRequest {
    pub class_name: String,
    pub bbox: BboxRequest,
    pub confidence: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct BboxRequest {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize)]
pub struct AnnotationResponse {
    pub id: String,
    pub sample_id: String,
    pub dataset_id: String,
    pub class_name: String,
    pub class_id: u32,
    pub bbox: BboxResponse,
    pub format: String,
    pub confidence: Option<f32>,
    pub source: String,
}

#[derive(Debug, Serialize)]
pub struct BboxResponse {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize)]
pub struct ListAnnotationsResponse {
    pub annotations: Vec<AnnotationResponse>,
}
