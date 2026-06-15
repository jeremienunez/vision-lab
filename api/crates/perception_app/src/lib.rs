#![forbid(unsafe_code)]
//! Application use cases and ports for PerceptionLab.

mod error;
mod models;
pub mod ports;
pub mod use_cases;

pub use error::UseCaseError;
pub use models::{
    AnnotationDraft, DatasetDraft, DatasetStats, DatasetVersionDraft, SampleDraft, TaskType,
    TrainingJobDraft, TrainingJobQueueEntry, TrainingJobQueueStatus, TrainingMetricDraft,
};
pub use ports::{
    AnnotationRepository, DatasetRepository, DatasetVersionRepository, SampleRepository,
    SampleStorage, SampleStorageCommand, StoredSample, TrainingJobQueue, TrainingJobRepository,
    TrainingMetricRepository,
};
pub use use_cases::{
    AddAnnotationCommand, AddAnnotationUseCase, CreateDatasetCommand, CreateDatasetUseCase,
    CreateDatasetVersionCommand, CreateDatasetVersionUseCase, CreateTrainingJobCommand,
    CreateTrainingJobUseCase, DatasetStatsUseCase, ListDatasetsUseCase,
    ListSampleAnnotationsUseCase, ListTrainingMetricsUseCase, RecordTrainingMetricCommand,
    RecordTrainingMetricUseCase, TransitionTrainingJobCommand, TransitionTrainingJobUseCase,
    UploadSampleCommand, UploadSampleUseCase,
};

pub const CRATE_NAME: &str = "perception_app";
