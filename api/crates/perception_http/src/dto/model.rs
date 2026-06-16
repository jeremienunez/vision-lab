use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
pub struct CompareModelsRequest {
    pub model_ids: Vec<String>,
    pub metric_name: String,
}

#[derive(Debug, Serialize)]
pub struct ModelComparisonResponse {
    pub metric_name: String,
    pub direction: &'static str,
    pub best_model_id: String,
    pub models: Vec<ModelComparisonEntryResponse>,
}

#[derive(Debug, Serialize)]
pub struct ModelComparisonEntryResponse {
    pub rank: u32,
    pub model_id: String,
    pub name: String,
    pub version: String,
    pub metric_value: f64,
    pub metrics_summary: BTreeMap<String, String>,
}
