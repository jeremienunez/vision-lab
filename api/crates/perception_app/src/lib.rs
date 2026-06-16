#![forbid(unsafe_code)]
//! Application use cases and ports for PerceptionLab.

mod error;
mod models;
pub mod ports;
pub mod use_cases;

pub use error::UseCaseError;
pub use models::{
    AnnotationDraft, DatasetDraft, DatasetStats, DatasetVersionDraft, DetectionDraft,
    InferenceRequest, InferenceResult, InferenceRunDraft, ModelComparison, ModelComparisonEntry,
    ModelDraft, ModelExportDraft, OverlayArtifact, SampleDraft, TaskType, TrainingClassMetric,
    TrainingJobDraft, TrainingJobQueueEntry, TrainingJobQueueStatus, TrainingMetricDraft,
    YoloAnnotationExport, YoloAnnotationFile, YoloAnnotationImportFile, YoloAnnotationImportResult,
};
pub use ports::{
    AnnotationRepository, DatasetRepository, DatasetVersionRepository, InferenceEngine,
    InferenceRunRepository, ModelExportRepository, ModelRepository, OverlayRenderer,
    SampleRepository, SampleStorage, SampleStorageCommand, StoredSample, TrainingJobQueue,
    TrainingJobRepository, TrainingMetricRepository,
};
pub use use_cases::{
    AddAnnotationCommand, AddAnnotationUseCase, CompareModelsCommand, CompareModelsUseCase,
    CreateDatasetCommand, CreateDatasetUseCase, CreateDatasetVersionCommand,
    CreateDatasetVersionUseCase, CreateTrainingJobCommand, CreateTrainingJobUseCase,
    DatasetStatsUseCase, ExportModelCommand, ExportModelUseCase, ExportYoloAnnotationsUseCase,
    GenerateOverlayUseCase, GetModelUseCase, ImportYoloAnnotationsCommand,
    ImportYoloAnnotationsUseCase, ListDatasetsUseCase, ListModelExportsUseCase, ListModelsUseCase,
    ListSampleAnnotationsUseCase, ListTrainingClassMetricsUseCase, ListTrainingJobsUseCase,
    ListTrainingMetricsUseCase, PromoteModelCommand, PromoteModelUseCase,
    RecordTrainingMetricCommand, RecordTrainingMetricUseCase, RegisterModelCommand,
    RegisterModelUseCase, RunInferenceCommand, RunInferenceUseCase, TransitionTrainingJobCommand,
    TransitionTrainingJobUseCase, UploadSampleCommand, UploadSampleUseCase,
};

pub const CRATE_NAME: &str = "perception_app";
