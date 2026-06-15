use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{
    DetectionDraft, InferenceEngine, InferenceRequest, InferenceResult, ModelDraft,
    ModelRepository, UseCaseError,
};
use perception_domain::{DatasetVersionId, ModelId, ModelStatus, NormalizedBbox, TrainingJobId};
use serde_json::Value;
use tower::ServiceExt;

#[derive(Default)]
struct RouteModelRepository {
    models: Mutex<Vec<ModelDraft>>,
}

struct RouteInferenceEngine;

#[async_trait]
impl ModelRepository for RouteModelRepository {
    async fn create(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        self.models
            .lock()
            .expect("repository mutex is available")
            .push(model.clone());
        Ok(model)
    }

    async fn list(&self) -> Result<Vec<ModelDraft>, UseCaseError> {
        Ok(self
            .models
            .lock()
            .expect("repository mutex is available")
            .clone())
    }

    async fn get(&self, model_id: ModelId) -> Result<Option<ModelDraft>, UseCaseError> {
        Ok(self
            .models
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|model| model.id == model_id)
            .cloned())
    }
}

#[async_trait]
impl InferenceEngine for RouteInferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResult, UseCaseError> {
        Ok(InferenceResult {
            model_id: request.model.id,
            latency_ms: 9,
            detections: vec![
                DetectionDraft {
                    class_id: 0,
                    class_name: "cup".to_owned(),
                    confidence: 0.95,
                    bbox: NormalizedBbox::new(0.1, 0.2, 0.3, 0.4).expect("bbox is valid"),
                    distance_m: Some(0.4),
                },
                DetectionDraft {
                    class_id: 1,
                    class_name: "book".to_owned(),
                    confidence: 0.42,
                    bbox: NormalizedBbox::new(0.2, 0.3, 0.2, 0.2).expect("bbox is valid"),
                    distance_m: None,
                },
            ],
        })
    }
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), 4096)
        .await
        .expect("body is readable");
    serde_json::from_slice(&body).expect("body is JSON")
}

fn model_fixture() -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: "desk-objects".to_owned(),
        version: "v1".to_owned(),
        training_job_id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "tiny_torch".to_owned(),
        artifact_uri: "file:///tmp/model.pt".to_owned(),
        metrics_summary: BTreeMap::from([("train_loss".to_owned(), "0.32".to_owned())]),
        status: ModelStatus::Candidate,
    }
}

fn inference_multipart_body(mime_type: &str, confidence_threshold: &str) -> (String, String) {
    let boundary = "perceptionlab-infer-boundary";
    let body = format!(
        "--{boundary}\r\n\
Content-Disposition: form-data; name=\"confidence_threshold\"\r\n\r\n\
{confidence_threshold}\r\n\
--{boundary}\r\n\
Content-Disposition: form-data; name=\"image\"; filename=\"cup.jpg\"\r\n\
Content-Type: {mime_type}\r\n\r\n\
fake-jpeg-bytes\r\n\
--{boundary}--\r\n"
    );

    (boundary.to_owned(), body)
}

#[tokio::test]
async fn list_models_route_returns_registered_models() {
    let models = Arc::new(RouteModelRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(models, Arc::new(RouteInferenceEngine));

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/models")
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(
        body["models"]
            .as_array()
            .expect("models are an array")
            .len(),
        1
    );
    assert_eq!(body["models"][0]["id"], model.id.to_string());
    assert_eq!(body["models"][0]["status"], "candidate");
}

#[tokio::test]
async fn get_model_route_returns_model_detail() {
    let models = Arc::new(RouteModelRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(models, Arc::new(RouteInferenceEngine));

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/models/{}", model.id))
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["id"], model.id.to_string());
    assert_eq!(body["training_job_id"], model.training_job_id.to_string());
    assert_eq!(
        body["dataset_version_id"],
        model.dataset_version_id.to_string()
    );
    assert_eq!(body["model_family"], "tiny_torch");
    assert_eq!(body["metrics_summary"]["train_loss"], "0.32");
}

#[tokio::test]
async fn infer_model_route_returns_detections_filtered_by_confidence() {
    let models = Arc::new(RouteModelRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(models, Arc::new(RouteInferenceEngine));
    let (boundary, body) = inference_multipart_body("image/jpeg", "0.90");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/models/{}/infer", model.id))
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(axum::body::Body::from(body))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["model_id"], model.id.to_string());
    assert_eq!(body["latency_ms"], 9);
    assert_eq!(
        body["detections"]
            .as_array()
            .expect("detections are an array")
            .len(),
        1
    );
    assert_eq!(body["detections"][0]["class_name"], "cup");
    assert!(
        body["detections"][0]["confidence"]
            .as_f64()
            .expect("confidence is numeric")
            >= 0.90
    );
}

#[tokio::test]
async fn infer_model_route_rejects_invalid_image_file() {
    let models = Arc::new(RouteModelRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(models, Arc::new(RouteInferenceEngine));
    let (boundary, body) = inference_multipart_body("text/plain", "0.25");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/models/{}/infer", model.id))
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(axum::body::Body::from(body))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}
