use perception_app::{DetectionDraft, InferenceRunDraft, InferenceRunRepository};
use perception_domain::{InferenceRunId, ModelId, NormalizedBbox};

fn run_fixture(model_id: ModelId) -> InferenceRunDraft {
    InferenceRunDraft {
        id: InferenceRunId::new(),
        model_id,
        filename: "cup.jpg".to_owned(),
        mime_type: "image/jpeg".to_owned(),
        latency_ms: 12,
        detections: vec![DetectionDraft {
            class_id: 0,
            class_name: "cup".to_owned(),
            confidence: 0.91,
            bbox: NormalizedBbox::new(0.1, 0.2, 0.3, 0.4).expect("bbox is valid"),
            distance_m: None,
        }],
    }
}

#[tokio::test]
async fn transient_inference_run_repository_creates_and_gets_runs() {
    let repository = perception_infra::TransientInferenceRunRepository::default();
    let run = repository
        .create(run_fixture(ModelId::new()))
        .await
        .expect("run is stored");

    let fetched = repository
        .get(run.id)
        .await
        .expect("run lookup succeeds")
        .expect("run exists");

    assert_eq!(fetched, run);
}
