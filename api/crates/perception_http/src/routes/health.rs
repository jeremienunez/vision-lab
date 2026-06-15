use axum::{Json, Router, routing::get};

use crate::dto::health::HealthResponse;

pub fn routes() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse::healthy_without_configured_dependencies())
}
