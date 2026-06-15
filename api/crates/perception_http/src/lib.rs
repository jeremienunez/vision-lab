#![forbid(unsafe_code)]
//! HTTP routes, DTOs, mappers, and public API errors for PerceptionLab.

use axum::Router;
use std::sync::Arc;

use perception_app::DatasetRepository;

pub mod dto;
pub mod mappers;
pub mod routes;
pub mod state;

pub const CRATE_NAME: &str = "perception_http";

pub fn router() -> Router {
    routes::health::routes()
}

pub fn router_with_dataset_repository(dataset_repository: Arc<dyn DatasetRepository>) -> Router {
    routes::health::routes().merge(routes::datasets::routes(state::HttpState::new(
        dataset_repository,
    )))
}
