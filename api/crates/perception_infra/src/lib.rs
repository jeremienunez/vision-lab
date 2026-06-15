#![forbid(unsafe_code)]
//! PostgreSQL, storage, queue, and config adapters for PerceptionLab.

mod fake_inference_engine;
mod local_sample_storage;
mod transient_annotation_repository;
mod transient_dataset_repository;
mod transient_dataset_version_repository;
mod transient_model_export_repository;
mod transient_model_repository;
mod transient_sample_repository;
mod transient_training_job_queue;
mod transient_training_job_repository;
mod transient_training_metric_repository;

pub use fake_inference_engine::FakeInferenceEngine;
pub use local_sample_storage::LocalSampleStorage;
pub use transient_annotation_repository::TransientAnnotationRepository;
pub use transient_dataset_repository::TransientDatasetRepository;
pub use transient_dataset_version_repository::TransientDatasetVersionRepository;
pub use transient_model_export_repository::TransientModelExportRepository;
pub use transient_model_repository::TransientModelRepository;
pub use transient_sample_repository::TransientSampleRepository;
pub use transient_training_job_queue::TransientTrainingJobQueue;
pub use transient_training_job_repository::TransientTrainingJobRepository;
pub use transient_training_metric_repository::TransientTrainingMetricRepository;

pub const CRATE_NAME: &str = "perception_infra";
