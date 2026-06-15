use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OverlayResponse {
    pub inference_run_id: String,
    pub artifact_uri: String,
    pub labels: Vec<String>,
}
