use perception_app::TrainingJobDraft;
use perception_domain::TrainingJobStatus;

use crate::dto::training_job::TrainingJobResponse;

pub fn training_job_response(job: TrainingJobDraft) -> TrainingJobResponse {
    TrainingJobResponse {
        id: job.id.to_string(),
        dataset_version_id: job.dataset_version_id.to_string(),
        model_family: job.model_family,
        base_model: job.base_model,
        status: training_job_status_name(job.status),
    }
}

fn training_job_status_name(status: TrainingJobStatus) -> &'static str {
    match status {
        TrainingJobStatus::Queued => "queued",
        TrainingJobStatus::Running => "running",
        TrainingJobStatus::Succeeded => "succeeded",
        TrainingJobStatus::Failed => "failed",
        TrainingJobStatus::Cancelled => "cancelled",
    }
}
