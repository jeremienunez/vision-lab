use async_trait::async_trait;
use perception_app::{
    DetectionDraft, InferenceEngine, InferenceRequest, InferenceResult, UseCaseError,
};
use perception_domain::{InferenceRunId, NormalizedBbox};

pub struct FakeInferenceEngine;

#[async_trait]
impl InferenceEngine for FakeInferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResult, UseCaseError> {
        Ok(InferenceResult {
            run_id: InferenceRunId::new(),
            model_id: request.model.id,
            latency_ms: 1,
            detections: vec![DetectionDraft {
                class_id: 0,
                class_name: "object".to_owned(),
                confidence: 0.91,
                bbox: NormalizedBbox::new(0.25, 0.25, 0.5, 0.5)
                    .map_err(|_| UseCaseError::Repository("fake inference bbox invalid"))?,
                distance_m: None,
            }],
        })
    }
}
