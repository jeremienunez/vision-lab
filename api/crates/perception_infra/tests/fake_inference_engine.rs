use std::collections::BTreeMap;

use perception_app::{InferenceEngine, InferenceRequest, ModelDraft};
use perception_domain::{DatasetVersionId, ModelId, ModelStatus, TrainingJobId};
use perception_infra::FakeInferenceEngine;

fn model_fixture() -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: "desk-objects".to_owned(),
        version: "v1".to_owned(),
        training_job_id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "tiny_torch".to_owned(),
        artifact_uri: "file:///tmp/model.pt".to_owned(),
        metrics_summary: BTreeMap::new(),
        status: ModelStatus::Candidate,
    }
}

#[tokio::test]
async fn fake_inference_engine_returns_deterministic_detection_for_model() {
    let model = model_fixture();
    let result = FakeInferenceEngine
        .infer(InferenceRequest {
            model: model.clone(),
            filename: "cup.jpg".to_owned(),
            mime_type: "image/jpeg".to_owned(),
            image_bytes: vec![1, 2, 3],
        })
        .await
        .expect("inference succeeds");

    assert_eq!(result.model_id, model.id);
    assert_eq!(result.latency_ms, 1);
    assert_eq!(result.detections.len(), 1);
    assert_eq!(result.detections[0].class_name, "object");
    assert_eq!(result.detections[0].confidence, 0.91);
}

#[tokio::test]
async fn fake_inference_engine_returns_model_declared_classes_for_demo_models() {
    let mut model = model_fixture();
    model.metrics_summary = BTreeMap::from([("classes".to_owned(), "cup,book,phone".to_owned())]);

    let result = FakeInferenceEngine
        .infer(InferenceRequest {
            model,
            filename: "desk-objects.png".to_owned(),
            mime_type: "image/png".to_owned(),
            image_bytes: vec![1, 2, 3],
        })
        .await
        .expect("inference succeeds");

    assert_eq!(result.detections.len(), 3);
    assert_eq!(result.detections[0].class_name, "cup");
    assert_eq!(result.detections[1].class_name, "book");
    assert_eq!(result.detections[2].class_name, "phone");
}
