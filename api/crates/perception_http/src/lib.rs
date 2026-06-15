#![forbid(unsafe_code)]
//! HTTP routes, DTOs, mappers, and public API errors for PerceptionLab.

use axum::Router;

pub mod dto;
pub mod routes;

pub const CRATE_NAME: &str = "perception_http";

pub fn router() -> Router {
    routes::health::routes()
}
