#![forbid(unsafe_code)]
//! Application use cases and ports for PerceptionLab.

mod error;
mod models;
pub mod ports;
pub mod use_cases;

pub use error::UseCaseError;
pub use models::{DatasetDraft, SampleDraft, TaskType};
pub use ports::{
    DatasetRepository, SampleRepository, SampleStorage, SampleStorageCommand, StoredSample,
};
pub use use_cases::{
    CreateDatasetCommand, CreateDatasetUseCase, ListDatasetsUseCase, UploadSampleCommand,
    UploadSampleUseCase,
};

pub const CRATE_NAME: &str = "perception_app";
