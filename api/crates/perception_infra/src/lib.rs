#![forbid(unsafe_code)]
//! PostgreSQL, storage, queue, and config adapters for PerceptionLab.

mod fake_inference_engine;
mod local_sample_storage;
mod postgres_annotation_repository_sqlx;
mod postgres_dataset_repository_sqlx;
mod postgres_dataset_version_repository_sqlx;
mod postgres_sample_repository_sqlx;
mod postgres_training_job_queue_sqlx;
mod postgres_training_job_repository_sqlx;
mod repository_backend;
mod svg_overlay_renderer;
mod transient_annotation_repository;
mod transient_dataset_repository;
mod transient_dataset_version_repository;
mod transient_inference_run_repository;
mod transient_model_export_repository;
mod transient_model_repository;
mod transient_sample_repository;
mod transient_training_job_queue;
mod transient_training_job_repository;
mod transient_training_metric_repository;
mod yolo_cli_inference_engine;

pub use fake_inference_engine::FakeInferenceEngine;
pub use local_sample_storage::LocalSampleStorage;
pub use postgres_annotation_repository_sqlx::PostgresAnnotationRepository;
pub use postgres_dataset_repository_sqlx::PostgresDatasetRepository;
pub use postgres_dataset_version_repository_sqlx::PostgresDatasetVersionRepository;
pub use postgres_sample_repository_sqlx::PostgresSampleRepository;
pub use postgres_training_job_queue_sqlx::PostgresTrainingJobQueue;
pub use postgres_training_job_repository_sqlx::PostgresTrainingJobRepository;
pub use repository_backend::RepositoryBackend;
pub use svg_overlay_renderer::SvgOverlayRenderer;
pub use transient_annotation_repository::TransientAnnotationRepository;
pub use transient_dataset_repository::TransientDatasetRepository;
pub use transient_dataset_version_repository::TransientDatasetVersionRepository;
pub use transient_inference_run_repository::TransientInferenceRunRepository;
pub use transient_model_export_repository::TransientModelExportRepository;
pub use transient_model_repository::TransientModelRepository;
pub use transient_sample_repository::TransientSampleRepository;
pub use transient_training_job_queue::TransientTrainingJobQueue;
pub use transient_training_job_repository::TransientTrainingJobRepository;
pub use transient_training_metric_repository::TransientTrainingMetricRepository;
pub use yolo_cli_inference_engine::{
    YoloCliCommand, YoloCliCommandOutput, YoloCliCommandRunner, YoloCliInferenceConfig,
    YoloCliInferenceEngine,
};

pub const CRATE_NAME: &str = "perception_infra";
