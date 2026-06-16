use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{
    DetectionDraft, InferenceEngine, InferenceRequest, InferenceResult, InferenceRunDraft,
    InferenceRunRepository, ModelDraft, ModelExportDraft, ModelExportRepository, ModelRepository,
    OverlayArtifact, OverlayRenderer, TrainingJobDraft, TrainingJobRepository, UseCaseError,
};
use perception_domain::{
    DatasetVersionId, InferenceRunId, ModelId, ModelStatus, NormalizedBbox,
    TrainingHyperparameters, TrainingJobId, TrainingJobStatus,
};
use serde_json::Value;
use tower::ServiceExt;

#[derive(Default)]
struct RouteModelRepository {
    models: Mutex<Vec<ModelDraft>>,
}

#[derive(Default)]
struct RouteModelExportRepository {
    exports: Mutex<Vec<ModelExportDraft>>,
}

#[derive(Default)]
struct RouteInferenceRunRepository {
    runs: Mutex<Vec<InferenceRunDraft>>,
}

#[derive(Default)]
struct RouteTrainingJobRepository {
    jobs: Mutex<Vec<TrainingJobDraft>>,
}

struct RouteInferenceEngine;

struct RouteOverlayRenderer;

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

    async fn update(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        let mut models = self.models.lock().expect("repository mutex is available");
        let stored = models
            .iter_mut()
            .find(|stored_model| stored_model.id == model.id)
            .ok_or(UseCaseError::NotFound("model not found"))?;
        *stored = model.clone();
        Ok(model)
    }
}

#[async_trait]
impl TrainingJobRepository for RouteTrainingJobRepository {
    async fn create(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        self.jobs
            .lock()
            .expect("repository mutex is available")
            .push(job.clone());
        Ok(job)
    }

    async fn get(&self, job_id: TrainingJobId) -> Result<Option<TrainingJobDraft>, UseCaseError> {
        Ok(self
            .jobs
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|job| job.id == job_id)
            .cloned())
    }

    async fn update(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        let mut jobs = self.jobs.lock().expect("repository mutex is available");
        let stored = jobs
            .iter_mut()
            .find(|stored_job| stored_job.id == job.id)
            .ok_or(UseCaseError::NotFound("training job not found"))?;
        *stored = job.clone();
        Ok(job)
    }
}

#[async_trait]
impl InferenceEngine for RouteInferenceEngine {
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResult, UseCaseError> {
        Ok(InferenceResult {
            run_id: InferenceRunId::new(),
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

#[async_trait]
impl InferenceRunRepository for RouteInferenceRunRepository {
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

#[async_trait]
impl ModelExportRepository for RouteModelExportRepository {
    async fn create(&self, export: ModelExportDraft) -> Result<ModelExportDraft, UseCaseError> {
        self.exports
            .lock()
            .expect("repository mutex is available")
            .push(export.clone());
        Ok(export)
    }

    async fn list_by_model(
        &self,
        model_id: ModelId,
    ) -> Result<Vec<ModelExportDraft>, UseCaseError> {
        Ok(self
            .exports
            .lock()
            .expect("repository mutex is available")
            .iter()
            .filter(|export| export.model_id == model_id)
            .cloned()
            .collect())
    }
}

#[async_trait]
impl OverlayRenderer for RouteOverlayRenderer {
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

fn succeeded_training_job_fixture() -> TrainingJobDraft {
    TrainingJobDraft {
        id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "tiny_torch".to_owned(),
        base_model: None,
        status: TrainingJobStatus::Succeeded,
        hyperparameters: TrainingHyperparameters::new(2, 1, 64, 0.01)
            .expect("hyperparameters are valid"),
        error_message: None,
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
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(
        models,
        exports,
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );

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
async fn register_model_route_creates_candidate_model_from_succeeded_job() {
    let jobs = Arc::new(RouteTrainingJobRepository::default());
    let models = Arc::new(RouteModelRepository::default());
    let job = jobs
        .create(succeeded_training_job_fixture())
        .await
        .expect("job is created");
    let app = perception_http::router_with_model_registration_ports(jobs, models.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/models")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "training_job_id": job.id.to_string(),
                        "name": "desk-objects-demo",
                        "version": "v1",
                        "artifact_uri": "file:///tmp/perceptionlab/demo-model.pt",
                        "metrics_summary": {
                            "mAP50": "0.91",
                            "classes": "cup,book,phone"
                        }
                    }))
                    .expect("request JSON is encoded"),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::CREATED);
    let model = json_body(response).await;
    assert_eq!(model["training_job_id"], job.id.to_string());
    assert_eq!(
        model["dataset_version_id"],
        job.dataset_version_id.to_string()
    );
    assert_eq!(model["model_family"], "tiny_torch");
    assert_eq!(model["status"], "candidate");
    assert_eq!(model["metrics_summary"]["classes"], "cup,book,phone");
    assert_eq!(models.list().await.expect("models are listed").len(), 1);
}

#[tokio::test]
async fn get_model_route_returns_model_detail() {
    let models = Arc::new(RouteModelRepository::default());
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(
        models,
        exports,
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );

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
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(
        models,
        exports,
        runs.clone(),
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );
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
    let run_id = InferenceRunId::parse(body["run_id"].as_str().expect("run id is a string"))
        .expect("run id parses");
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

    let stored = runs
        .get(run_id)
        .await
        .expect("run lookup succeeds")
        .expect("run is stored");

    assert_eq!(stored.model_id, model.id);
    assert_eq!(stored.detections.len(), 1);
    assert_eq!(stored.detections[0].class_name, "cup");
}

#[tokio::test]
async fn infer_model_route_rejects_invalid_image_file() {
    let models = Arc::new(RouteModelRepository::default());
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(
        models,
        exports,
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );
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

#[tokio::test]
async fn export_model_route_creates_and_lists_onnx_exports() {
    let models = Arc::new(RouteModelRepository::default());
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(
        models,
        exports.clone(),
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/models/{}/exports", model.id))
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    serde_json::to_vec(&serde_json::json!({ "format": "onnx" }))
                        .expect("request JSON is encoded"),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::CREATED);
    let export = json_body(response).await;
    assert_eq!(export["model_id"], model.id.to_string());
    assert_eq!(export["format"], "onnx");
    assert_eq!(export["artifact_uri"], "file:///tmp/model.onnx");
    assert_eq!(export["status"], "succeeded");
    assert_eq!(export["error_message"], Value::Null);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/models/{}/exports", model.id))
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(
        body["exports"]
            .as_array()
            .expect("exports are an array")
            .len(),
        1
    );
    assert_eq!(body["exports"][0]["artifact_uri"], "file:///tmp/model.onnx");

    let listed = exports
        .list_by_model(model.id)
        .await
        .expect("exports are listed from repository");
    assert_eq!(listed.len(), 1);
}

#[tokio::test]
async fn export_model_route_creates_coreml_export() {
    let models = Arc::new(RouteModelRepository::default());
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_ports(
        models,
        exports,
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/models/{}/exports", model.id))
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    serde_json::to_vec(&serde_json::json!({ "format": "coreml" }))
                        .expect("request JSON is encoded"),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::CREATED);
    let export = json_body(response).await;
    assert_eq!(export["model_id"], model.id.to_string());
    assert_eq!(export["format"], "coreml");
    assert_eq!(export["artifact_uri"], "file:///tmp/model.mlpackage");
    assert_eq!(export["status"], "succeeded");
    assert_eq!(export["error_message"], Value::Null);
}

#[tokio::test]
async fn compare_models_route_ranks_models_by_metric() {
    let models = Arc::new(RouteModelRepository::default());
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let baseline = models
        .create(ModelDraft {
            metrics_summary: BTreeMap::from([("mAP50".to_owned(), "0.73".to_owned())]),
            ..model_fixture()
        })
        .await
        .expect("baseline is created");
    let challenger = models
        .create(ModelDraft {
            name: "challenger".to_owned(),
            metrics_summary: BTreeMap::from([("mAP50".to_owned(), "0.81".to_owned())]),
            ..model_fixture()
        })
        .await
        .expect("challenger is created");
    let app = perception_http::router_with_model_ports(
        models,
        exports,
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/models/compare")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "model_ids": [baseline.id.to_string(), challenger.id.to_string()],
                        "metric_name": "mAP50"
                    }))
                    .expect("request JSON is encoded"),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
    let comparison = json_body(response).await;
    assert_eq!(comparison["metric_name"], "mAP50");
    assert_eq!(comparison["direction"], "higher_is_better");
    assert_eq!(comparison["best_model_id"], challenger.id.to_string());
    assert_eq!(
        comparison["models"][0]["model_id"],
        challenger.id.to_string()
    );
    assert_eq!(comparison["models"][0]["rank"], 1);
    assert_eq!(comparison["models"][0]["metric_value"], 0.81);
    assert_eq!(comparison["models"][1]["model_id"], baseline.id.to_string());
}

#[tokio::test]
async fn promote_model_route_promotes_target_and_demotes_competitor() {
    let models = Arc::new(RouteModelRepository::default());
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let dataset_version_id = DatasetVersionId::new();
    let baseline = models
        .create(ModelDraft {
            dataset_version_id,
            status: ModelStatus::Promoted,
            ..model_fixture()
        })
        .await
        .expect("baseline is created");
    let challenger = models
        .create(ModelDraft {
            name: "challenger".to_owned(),
            dataset_version_id,
            status: ModelStatus::Validated,
            ..model_fixture()
        })
        .await
        .expect("challenger is created");
    let app = perception_http::router_with_model_ports(
        models.clone(),
        exports,
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/models/{}/promote", challenger.id))
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
    let promoted = json_body(response).await;
    assert_eq!(promoted["id"], challenger.id.to_string());
    assert_eq!(promoted["status"], "promoted");

    let stored_models = models.list().await.expect("models are listed");
    assert_eq!(
        stored_models
            .iter()
            .find(|model| model.id == baseline.id)
            .expect("baseline is stored")
            .status,
        ModelStatus::Validated
    );
    assert_eq!(
        stored_models
            .iter()
            .find(|model| model.id == challenger.id)
            .expect("challenger is stored")
            .status,
        ModelStatus::Promoted
    );
}

#[tokio::test]
async fn generate_overlay_route_returns_artifact_uri_and_labels() {
    let models = Arc::new(RouteModelRepository::default());
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let run = runs
        .create(InferenceRunDraft {
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
        })
        .await
        .expect("run is stored");
    let app = perception_http::router_with_model_ports(
        models,
        exports,
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/inference-runs/{}/overlay", run.id))
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = json_body(response).await;
    assert_eq!(body["inference_run_id"], run.id.to_string());
    assert_eq!(
        body["artifact_uri"],
        format!("artifact://overlays/{}.svg", run.id)
    );
    assert_eq!(body["labels"][0], "cup 89%");
}

#[tokio::test]
async fn generate_overlay_route_rejects_unknown_inference_run() {
    let models = Arc::new(RouteModelRepository::default());
    let exports = Arc::new(RouteModelExportRepository::default());
    let runs = Arc::new(RouteInferenceRunRepository::default());
    let app = perception_http::router_with_model_ports(
        models,
        exports,
        runs,
        Arc::new(RouteOverlayRenderer),
        Arc::new(RouteInferenceEngine),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/inference-runs/{}/overlay", InferenceRunId::new()))
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
