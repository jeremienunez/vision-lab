use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{ModelDraft, ModelRepository, UseCaseError};
use perception_domain::{DatasetVersionId, ModelId, ModelStatus, TrainingJobId};
use serde_json::Value;
use tower::ServiceExt;

#[derive(Default)]
struct RouteModelRepository {
    models: Mutex<Vec<ModelDraft>>,
}

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

#[tokio::test]
async fn list_models_route_returns_registered_models() {
    let models = Arc::new(RouteModelRepository::default());
    let model = models
        .create(model_fixture())
        .await
        .expect("model is created");
    let app = perception_http::router_with_model_repository(models);

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
    let app = perception_http::router_with_model_repository(models);

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
