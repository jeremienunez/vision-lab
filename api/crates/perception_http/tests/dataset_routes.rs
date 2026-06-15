use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{DatasetDraft, DatasetRepository, UseCaseError};
use serde_json::{Value, json};
use tower::ServiceExt;

#[derive(Default)]
struct RouteDatasetRepository {
    datasets: Mutex<Vec<DatasetDraft>>,
}

#[async_trait]
impl DatasetRepository for RouteDatasetRepository {
    async fn create(&self, dataset: DatasetDraft) -> Result<DatasetDraft, UseCaseError> {
        self.datasets
            .lock()
            .expect("repository mutex is available")
            .push(dataset.clone());
        Ok(dataset)
    }

    async fn list(&self) -> Result<Vec<DatasetDraft>, UseCaseError> {
        Ok(self
            .datasets
            .lock()
            .expect("repository mutex is available")
            .clone())
    }
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), 4096)
        .await
        .expect("body is readable");
    serde_json::from_slice(&body).expect("body is JSON")
}

#[tokio::test]
async fn create_dataset_route_persists_a_draft_dataset_and_list_route_returns_it() {
    let app = perception_http::router_with_dataset_repository(Arc::new(
        RouteDatasetRepository::default(),
    ));

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/datasets")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    json!({
                        "name": " desk-objects-v1 ",
                        "description": "Desk object detection dataset",
                        "task_type": "object_detection",
                        "classes": ["cup", "book"]
                    })
                    .to_string(),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(create_response.status(), StatusCode::CREATED);
    let created = json_body(create_response).await;
    assert!(created["id"].as_str().expect("id is present").len() > 20);
    assert_eq!(created["name"], "desk-objects-v1");
    assert_eq!(created["description"], "Desk object detection dataset");
    assert_eq!(created["task_type"], "object_detection");
    assert_eq!(created["classes"], json!(["cup", "book"]));
    assert_eq!(created["status"], "draft");

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/datasets")
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(list_response.status(), StatusCode::OK);
    let listed = json_body(list_response).await;
    assert_eq!(listed["datasets"].as_array().expect("datasets is an array").len(), 1);
    assert_eq!(listed["datasets"][0]["name"], "desk-objects-v1");
    assert_eq!(listed["datasets"][0]["status"], "draft");
}

#[tokio::test]
async fn create_dataset_route_maps_validation_failures_to_bad_request() {
    let app = perception_http::router_with_dataset_repository(Arc::new(
        RouteDatasetRepository::default(),
    ));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/datasets")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    json!({
                        "name": " ",
                        "description": null,
                        "task_type": "object_detection",
                        "classes": ["cup"]
                    })
                    .to_string(),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error = json_body(response).await;
    assert_eq!(error["error"]["code"], "validation_failed");
    assert_eq!(error["error"]["message"], "dataset name is required");
}
