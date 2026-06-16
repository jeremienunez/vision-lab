use perception_app::{ModelComparison, ModelComparisonEntry, ModelDraft};
use perception_domain::ModelStatus;

use crate::dto::model::{ModelComparisonEntryResponse, ModelComparisonResponse, ModelResponse};

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

pub fn model_comparison_response(comparison: ModelComparison) -> ModelComparisonResponse {
    ModelComparisonResponse {
        metric_name: comparison.metric_name,
        direction: comparison.direction,
        best_model_id: comparison.best_model_id.to_string(),
        models: comparison
            .models
            .into_iter()
            .map(model_comparison_entry_response)
            .collect(),
    }
}

fn model_comparison_entry_response(entry: ModelComparisonEntry) -> ModelComparisonEntryResponse {
    ModelComparisonEntryResponse {
        rank: entry.rank,
        model_id: entry.model_id.to_string(),
        name: entry.name,
        version: entry.version,
        metric_value: entry.metric_value,
        metrics_summary: entry.metrics_summary,
    }
}
