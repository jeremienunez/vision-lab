use std::sync::Mutex;

use async_trait::async_trait;
use perception_app::{
    DetectionDraft, GenerateOverlayUseCase, InferenceRunDraft, InferenceRunRepository,
    OverlayArtifact, OverlayRenderer, UseCaseError,
};
use perception_domain::{InferenceRunId, ModelId, NormalizedBbox};

#[derive(Default)]
struct InMemoryInferenceRunRepository {
    runs: Mutex<Vec<InferenceRunDraft>>,
}

#[async_trait]
impl InferenceRunRepository for InMemoryInferenceRunRepository {
    async fn create(&self, run: InferenceRunDraft) -> Result<InferenceRunDraft, UseCaseError> {
        self.runs
            .lock()
            .expect("repository mutex is available")
            .push(run.clone());
        Ok(run)
    }

    async fn get(&self, run_id: InferenceRunId) -> Result<Option<InferenceRunDraft>, UseCaseError> {
        Ok(self
            .runs
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|run| run.id == run_id)
            .cloned())
    }
}

struct TestOverlayRenderer;

#[async_trait]
impl OverlayRenderer for TestOverlayRenderer {
    async fn render(&self, run: InferenceRunDraft) -> Result<OverlayArtifact, UseCaseError> {
        Ok(OverlayArtifact {
            inference_run_id: run.id,
            artifact_uri: format!("artifact://overlays/{}.svg", run.id),
            labels: run
                .detections
                .iter()
                .map(|detection| {
                    format!(
                        "{} {:.0}%",
                        detection.class_name,
                        detection.confidence * 100.0
                    )
                })
                .collect(),
        })
    }
}

fn inference_run_fixture() -> InferenceRunDraft {
    InferenceRunDraft {
        id: InferenceRunId::new(),
        model_id: ModelId::new(),
        filename: "cup.jpg".to_owned(),
        mime_type: "image/jpeg".to_owned(),
        latency_ms: 12,
        detections: vec![DetectionDraft {
            class_id: 0,
            class_name: "cup".to_owned(),
            confidence: 0.89,
            bbox: NormalizedBbox::new(0.1, 0.2, 0.3, 0.4).expect("bbox is valid"),
            distance_m: None,
        }],
    }
}

#[tokio::test]
async fn generate_overlay_returns_artifact_uri_and_detection_labels() {
    let runs = InMemoryInferenceRunRepository::default();
    let run = runs
        .create(inference_run_fixture())
        .await
        .expect("run is stored");

    let overlay = GenerateOverlayUseCase::new(&runs, &TestOverlayRenderer)
        .execute(run.id)
        .await
        .expect("overlay is generated");

    assert_eq!(overlay.inference_run_id, run.id);
    assert_eq!(
        overlay.artifact_uri,
        format!("artifact://overlays/{}.svg", run.id)
    );
    assert_eq!(overlay.labels, vec!["cup 89%"]);
}

#[tokio::test]
async fn generate_overlay_rejects_unknown_inference_run() {
    let runs = InMemoryInferenceRunRepository::default();

    let result = GenerateOverlayUseCase::new(&runs, &TestOverlayRenderer)
        .execute(InferenceRunId::new())
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::NotFound("inference run not found"))
    );
}
