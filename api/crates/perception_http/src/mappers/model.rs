use perception_app::ModelDraft;
use perception_domain::ModelStatus;

use crate::dto::model::ModelResponse;

pub fn model_response(model: ModelDraft) -> ModelResponse {
    ModelResponse {
        id: model.id.to_string(),
        name: model.name,
        version: model.version,
        training_job_id: model.training_job_id.to_string(),
        dataset_version_id: model.dataset_version_id.to_string(),
        model_family: model.model_family,
        artifact_uri: model.artifact_uri,
        metrics_summary: model.metrics_summary,
        status: model_status_name(model.status),
    }
}

fn model_status_name(status: ModelStatus) -> &'static str {
    match status {
        ModelStatus::Candidate => "candidate",
        ModelStatus::Validated => "validated",
        ModelStatus::Promoted => "promoted",
        ModelStatus::Archived => "archived",
    }
}
