use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTrainingJobRequest {
    pub dataset_version_id: String,
    pub model_family: String,
    pub base_model: Option<String>,
    pub hyperparameters: TrainingHyperparametersRequest,
}

#[derive(Debug, Deserialize)]
pub struct TrainingHyperparametersRequest {
    pub epochs: u16,
    pub batch_size: u16,
    pub image_size: u16,
    pub learning_rate: f32,
}

#[derive(Debug, Deserialize)]
pub struct TransitionTrainingJobRequest {
    pub next_status: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TrainingJobResponse {
    pub id: String,
    pub dataset_version_id: String,
    pub model_family: String,
    pub base_model: Option<String>,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ListTrainingJobsResponse {
    pub training_jobs: Vec<TrainingJobResponse>,
}
