use perception_app::ModelExportDraft;
use perception_domain::ExportStatus;

use crate::dto::model_export::ModelExportResponse;

pub fn model_export_response(export: ModelExportDraft) -> ModelExportResponse {
    ModelExportResponse {
        id: export.id.to_string(),
        model_id: export.model_id.to_string(),
        format: export.format,
        artifact_uri: export.artifact_uri,
        status: export_status_name(export.status),
        error_message: export.error_message,
    }
}

fn export_status_name(status: ExportStatus) -> &'static str {
    match status {
        ExportStatus::Queued => "queued",
        ExportStatus::Running => "running",
        ExportStatus::Succeeded => "succeeded",
        ExportStatus::Failed => "failed",
    }
}
