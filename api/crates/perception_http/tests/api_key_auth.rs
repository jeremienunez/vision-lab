use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use perception_app::{DatasetDraft, DatasetRepository, UseCaseError};
use perception_domain::DatasetId;
use tower::ServiceExt;

#[derive(Default)]
struct AuthDatasetRepository;

#[async_trait]
impl DatasetRepository for AuthDatasetRepository {
    async fn create(&self, dataset: DatasetDraft) -> Result<DatasetDraft, UseCaseError> {
        Ok(dataset)
    }

    async fn get(&self, _dataset_id: DatasetId) -> Result<Option<DatasetDraft>, UseCaseError> {
        Ok(None)
    }

    async fn list(&self) -> Result<Vec<DatasetDraft>, UseCaseError> {
        Ok(Vec::new())
    }
}

fn app_with_api_key(key: &str) -> axum::Router {
    let app =
        perception_http::router_with_dataset_repository(Arc::new(AuthDatasetRepository::default()));

    perception_http::with_optional_api_key_auth(
        app,
        perception_http::ApiKeyAuthConfig::required(key),
    )
}

#[tokio::test]
async fn configured_api_key_auth_keeps_health_public() {
    let response = app_with_api_key("local-secret")
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn configured_api_key_auth_rejects_missing_key_for_protected_routes() {
    let response = app_with_api_key("local-secret")
        .oneshot(
            Request::builder()
                .uri("/datasets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn configured_api_key_auth_rejects_wrong_key_for_protected_routes() {
    let response = app_with_api_key("local-secret")
        .oneshot(
            Request::builder()
                .uri("/datasets")
                .header("x-api-key", "wrong-secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn configured_api_key_auth_allows_correct_key_for_protected_routes() {
    let response = app_with_api_key("local-secret")
        .oneshot(
            Request::builder()
                .uri("/datasets")
                .header("x-api-key", "local-secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn disabled_api_key_auth_allows_protected_routes_without_key() {
    let app =
        perception_http::router_with_dataset_repository(Arc::new(AuthDatasetRepository::default()));
    let app = perception_http::with_optional_api_key_auth(
        app,
        perception_http::ApiKeyAuthConfig::disabled(),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/datasets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
