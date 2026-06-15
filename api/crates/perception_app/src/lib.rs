#![forbid(unsafe_code)]
//! Application use cases and ports for PerceptionLab.

mod error;
mod models;
pub mod ports;
pub mod use_cases;

pub use error::UseCaseError;
pub use models::{AnnotationDraft, DatasetDraft, DatasetStats, SampleDraft, TaskType};
pub use ports::{
    AnnotationRepository, DatasetRepository, SampleRepository, SampleStorage, SampleStorageCommand,
    StoredSample,
};
pub use use_cases::{
    AddAnnotationCommand, AddAnnotationUseCase, CreateDatasetCommand, CreateDatasetUseCase,
    DatasetStatsUseCase, ListDatasetsUseCase, ListSampleAnnotationsUseCase, UploadSampleCommand,
    UploadSampleUseCase,
};

pub const CRATE_NAME: &str = "perception_app";
