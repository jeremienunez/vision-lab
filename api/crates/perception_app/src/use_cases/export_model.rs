use perception_domain::{ExportStatus, ModelExportId, ModelId, ModelStatus};

use crate::{ModelExportDraft, ModelExportRepository, ModelRepository, UseCaseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportModelCommand {
    pub model_id: ModelId,
    pub format: String,
}

pub struct ExportModelUseCase<'repository> {
    model_repository: &'repository dyn ModelRepository,
    export_repository: &'repository dyn ModelExportRepository,
}

impl<'repository> ExportModelUseCase<'repository> {
    pub fn new(
        model_repository: &'repository dyn ModelRepository,
        export_repository: &'repository dyn ModelExportRepository,
    ) -> Self {
        Self {
            model_repository,
            export_repository,
        }
    }

    pub async fn execute(
        &self,
        command: ExportModelCommand,
    ) -> Result<ModelExportDraft, UseCaseError> {
        let format = normalized_format(&command.format)?;
        let model = self
            .model_repository
            .get(command.model_id)
            .await?
            .ok_or(UseCaseError::NotFound("model not found"))?;

        if model.status == ModelStatus::Archived {
            return Err(UseCaseError::Validation(
                "archived model cannot be exported",
            ));
        }

        self.export_repository
            .create(ModelExportDraft {
                id: ModelExportId::new(),
                model_id: model.id,
                format,
                artifact_uri: Some(onnx_artifact_uri(&model.artifact_uri)),
                status: ExportStatus::Succeeded,
                error_message: None,
            })
            .await
    }
}

fn normalized_format(format: &str) -> Result<String, UseCaseError> {
    let normalized = format.trim().to_ascii_lowercase();

    if normalized == "onnx" {
        Ok(normalized)
    } else {
        Err(UseCaseError::Validation("unsupported model export format"))
    }
}

fn onnx_artifact_uri(model_artifact_uri: &str) -> String {
    let model_artifact_uri = model_artifact_uri.trim();

    if let Some(stem) = model_artifact_uri.strip_suffix(".pt") {
        format!("{stem}.onnx")
    } else if let Some(stem) = model_artifact_uri.strip_suffix(".pth") {
        format!("{stem}.onnx")
    } else {
        format!("{model_artifact_uri}.onnx")
    }
}
