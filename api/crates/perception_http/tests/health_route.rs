use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn health_route_returns_platform_status_and_dependencies() {
    let response = perception_http::router()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1024)
        .await
        .expect("body is readable");
    let json: Value = serde_json::from_slice(&body).expect("body is JSON");

    assert_eq!(json["status"], "healthy");
    assert_eq!(json["dependencies"]["database"], "not_configured");
    assert_eq!(json["dependencies"]["storage"], "not_configured");
    assert_eq!(json["dependencies"]["queue"], "not_configured");
}
