use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    routing::get,
};
use perception_app::{DatasetDraft, DatasetRepository, UseCaseError};
use perception_domain::DatasetId;
use serde_json::Value;
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

fn protected_router() -> Router {
    let router = Router::new().route("/protected", get(|| async { "ok" }));

    perception_http::auth::with_optional_api_key_auth(router, Some("secret-key".to_owned()))
}

fn app_with_api_key(key: &str) -> Router {
    let app =
        perception_http::router_with_dataset_repository(Arc::new(AuthDatasetRepository));

    perception_http::with_optional_api_key_auth(
        app,
        perception_http::ApiKeyAuthConfig::required(key),
    )
}

#[tokio::test]
async fn health_route_remains_public_when_api_key_is_configured() {
    let response = perception_http::auth::with_optional_api_key_auth(
        perception_http::router(),
        Some("secret-key".to_owned()),
    )
    .oneshot(
        Request::builder()
            .uri("/health")
            .body(Body::empty())
            .expect("request is valid"),
    )
    .await
    .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn protected_route_rejects_missing_api_key() {
    let response = protected_router()
        .oneshot(
            Request::builder()
                .uri("/protected")
                .body(Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = to_bytes(response.into_body(), 1024)
        .await
        .expect("body is readable");
    let json: Value = serde_json::from_slice(&body).expect("body is JSON");

    assert_eq!(json["error"]["code"], "missing_api_key");
}

#[tokio::test]
async fn protected_route_rejects_wrong_api_key() {
    let response = protected_router()
        .oneshot(
            Request::builder()
                .uri("/protected")
                .header("x-api-key", "wrong-key")
                .body(Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    let body = to_bytes(response.into_body(), 1024)
        .await
        .expect("body is readable");
    let json: Value = serde_json::from_slice(&body).expect("body is JSON");

    assert_eq!(json["error"]["code"], "invalid_api_key");
}

#[tokio::test]
async fn protected_route_accepts_matching_api_key() {
    let response = protected_router()
        .oneshot(
            Request::builder()
                .uri("/protected")
                .header("x-api-key", "secret-key")
                .body(Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn configured_api_key_auth_allows_matching_key_for_dataset_routes() {
    let response = app_with_api_key("local-secret")
        .oneshot(
            Request::builder()
                .uri("/datasets")
                .header("x-api-key", "local-secret")
                .body(Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn disabled_api_key_auth_allows_protected_routes_without_key() {
    let app =
        perception_http::router_with_dataset_repository(Arc::new(AuthDatasetRepository));
    let app = perception_http::with_optional_api_key_auth(
        app,
        perception_http::ApiKeyAuthConfig::disabled(),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/datasets")
                .body(Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn blank_api_key_configuration_leaves_routes_unprotected() {
    let router = Router::new().route("/protected", get(|| async { "ok" }));
    let response = perception_http::auth::with_optional_api_key_auth(router, Some("  ".to_owned()))
        .oneshot(
            Request::builder()
                .uri("/protected")
                .body(Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
}
