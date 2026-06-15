use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InferenceResponse {
    pub model_id: String,
    pub latency_ms: u32,
    pub detections: Vec<DetectionResponse>,
}

#[derive(Debug, Serialize)]
pub struct DetectionResponse {
    pub class_id: u32,
    pub class_name: String,
    pub confidence: f32,
    pub bbox: DetectionBboxResponse,
    pub distance_m: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct DetectionBboxResponse {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
