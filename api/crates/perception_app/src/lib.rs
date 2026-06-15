#![forbid(unsafe_code)]
//! Application use cases and ports for PerceptionLab.

mod error;
mod models;
pub mod ports;
pub mod use_cases;

pub use error::UseCaseError;
pub use models::{
    AnnotationDraft, DatasetDraft, DatasetStats, DatasetVersionDraft, SampleDraft, TaskType,
    TrainingJobDraft,
};
pub use ports::{
    AnnotationRepository, DatasetRepository, DatasetVersionRepository, SampleRepository,
    SampleStorage, SampleStorageCommand, StoredSample, TrainingJobRepository,
};
pub use use_cases::{
    AddAnnotationCommand, AddAnnotationUseCase, CreateDatasetCommand, CreateDatasetUseCase,
    CreateDatasetVersionCommand, CreateDatasetVersionUseCase, CreateTrainingJobCommand,
    CreateTrainingJobUseCase, DatasetStatsUseCase, ListDatasetsUseCase,
    ListSampleAnnotationsUseCase, UploadSampleCommand, UploadSampleUseCase,
};

pub const CRATE_NAME: &str = "perception_app";
