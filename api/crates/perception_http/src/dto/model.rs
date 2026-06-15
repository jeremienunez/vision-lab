use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ModelResponse {
    pub id: String,
    pub name: String,
    pub version: String,
    pub training_job_id: String,
    pub dataset_version_id: String,
    pub model_family: String,
    pub artifact_uri: String,
    pub metrics_summary: BTreeMap<String, String>,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ListModelsResponse {
    pub models: Vec<ModelResponse>,
}
