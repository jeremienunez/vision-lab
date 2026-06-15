use std::fs;

use perception_app::{DetectionDraft, InferenceRunDraft, OverlayRenderer};
use perception_domain::{InferenceRunId, ModelId, NormalizedBbox};

fn run_fixture() -> InferenceRunDraft {
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
async fn svg_overlay_renderer_writes_svg_with_detection_labels() {
    let root =
        std::env::temp_dir().join(format!("perceptionlab-overlay-{}", InferenceRunId::new()));
    let renderer = perception_infra::SvgOverlayRenderer::new(&root);
    let run = run_fixture();

    let overlay = renderer.render(run.clone()).await.expect("overlay renders");

    assert_eq!(overlay.inference_run_id, run.id);
    assert!(overlay.artifact_uri.starts_with("file://"));
    assert_eq!(overlay.labels, vec!["cup 89%"]);

    let artifact_path = overlay
        .artifact_uri
        .strip_prefix("file://")
        .expect("artifact uri is file uri");
    let svg = fs::read_to_string(artifact_path).expect("overlay artifact exists");

    assert!(svg.contains("cup 89%"));
    assert!(svg.contains("<rect"));

    fs::remove_dir_all(root).expect("test overlay root is removed");
}
