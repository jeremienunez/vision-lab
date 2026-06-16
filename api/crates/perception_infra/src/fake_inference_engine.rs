use async_trait::async_trait;
use perception_app::{
    DetectionDraft, InferenceEngine, InferenceRequest, InferenceResult, UseCaseError,
};
use perception_domain::{InferenceRunId, NormalizedBbox};

pub struct FakeInferenceEngine;

#[async_trait]
impl InferenceEngine for FakeInferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResult, UseCaseError> {
        let detections = demo_detections(&request)?;

        Ok(InferenceResult {
            run_id: InferenceRunId::new(),
            model_id: request.model.id,
            latency_ms: 1,
            detections,
        })
    }
}

fn demo_detections(request: &InferenceRequest) -> Result<Vec<DetectionDraft>, UseCaseError> {
    let classes = request
        .model
        .metrics_summary
        .get("classes")
        .map(|classes| {
            classes
                .split(',')
                .map(str::trim)
                .filter(|class_name| !class_name.is_empty())
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .filter(|classes| !classes.is_empty())
        .unwrap_or_else(|| vec!["object".to_owned()]);
    let boxes = [
        (0.12, 0.34, 0.18, 0.35),
        (0.43, 0.54, 0.32, 0.22),
        (0.68, 0.40, 0.16, 0.28),
    ];

    classes
        .into_iter()
        .enumerate()
        .map(|(index, class_name)| {
            let (x, y, width, height) = boxes[index % boxes.len()];
            Ok(DetectionDraft {
                class_id: index as u32,
                class_name,
                confidence: (0.91 - index as f32 * 0.03).max(0.50),
                bbox: NormalizedBbox::new(x, y, width, height)
                    .map_err(|_| UseCaseError::Repository("fake inference bbox invalid"))?,
                distance_m: None,
            })
        })
        .collect()
}
