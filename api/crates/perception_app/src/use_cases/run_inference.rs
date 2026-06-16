use perception_domain::{InferenceRunId, ModelId, ModelStatus};

use crate::{
    InferenceEngine, InferenceRequest, InferenceResult, InferenceRunDraft, InferenceRunRepository,
    ModelRepository, UseCaseError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RunInferenceCommand {
    pub model_id: ModelId,
    pub filename: String,
    pub mime_type: String,
    pub image_bytes: Vec<u8>,
    pub confidence_threshold: f32,
}

pub struct RunInferenceUseCase<'repository> {
    model_repository: &'repository dyn ModelRepository,
    inference_run_repository: &'repository dyn InferenceRunRepository,
    inference_engine: &'repository dyn InferenceEngine,
}

impl<'repository> RunInferenceUseCase<'repository> {
    pub fn new(
        model_repository: &'repository dyn ModelRepository,
        inference_run_repository: &'repository dyn InferenceRunRepository,
        inference_engine: &'repository dyn InferenceEngine,
    ) -> Self {
        Self {
            model_repository,
            inference_run_repository,
            inference_engine,
        }
    }

    pub async fn execute(
        &self,
        command: RunInferenceCommand,
    ) -> Result<InferenceResult, UseCaseError> {
        validate_inference_contract(&command)?;

        let model = self
            .model_repository
            .get(command.model_id)
            .await?
            .ok_or(UseCaseError::NotFound("model not found"))?;

        if model.status == ModelStatus::Archived {
            return Err(UseCaseError::Validation(
                "archived model cannot run inference",
            ));
        }

        let filename = command.filename;
        let mime_type = command.mime_type;
        let run_id = InferenceRunId::new();
        let mut result = self
            .inference_engine
            .infer(InferenceRequest {
                model,
                filename: filename.clone(),
                mime_type: mime_type.clone(),
                image_bytes: command.image_bytes,
                confidence_threshold: command.confidence_threshold,
            })
            .await?;
        result.run_id = run_id;
        result
            .detections
            .retain(|detection| detection.confidence >= command.confidence_threshold);

        self.inference_run_repository
            .create(InferenceRunDraft {
                id: run_id,
                model_id: result.model_id,
                filename,
                mime_type,
                latency_ms: result.latency_ms,
                detections: result.detections.clone(),
            })
            .await?;

        Ok(result)
    }
}

fn validate_inference_contract(command: &RunInferenceCommand) -> Result<(), UseCaseError> {
    if command.filename.trim().is_empty() || command.image_bytes.is_empty() {
        return Err(UseCaseError::Validation("invalid inference image"));
    }

    if !command.mime_type.starts_with("image/") {
        return Err(UseCaseError::Validation("unsupported image mime type"));
    }

    if !(0.0..=1.0).contains(&command.confidence_threshold) {
        return Err(UseCaseError::Validation("invalid confidence threshold"));
    }

    Ok(())
}
