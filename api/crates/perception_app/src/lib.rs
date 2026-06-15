#![forbid(unsafe_code)]
//! Application use cases and ports for PerceptionLab.

mod error;
mod models;
pub mod ports;
pub mod use_cases;

pub use error::UseCaseError;
pub use models::{DatasetDraft, TaskType};
pub use ports::DatasetRepository;
pub use use_cases::{CreateDatasetCommand, CreateDatasetUseCase, ListDatasetsUseCase};

pub const CRATE_NAME: &str = "perception_app";
