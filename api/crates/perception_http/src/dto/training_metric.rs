use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TrainingMetricResponse {
    pub id: String,
    pub training_job_id: String,
    pub split_name: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub step: Option<u32>,
    pub epoch: Option<u32>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct ListTrainingMetricsResponse {
    pub metrics: Vec<TrainingMetricResponse>,
}
