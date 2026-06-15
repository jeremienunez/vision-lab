#![forbid(unsafe_code)]
//! PostgreSQL, storage, queue, and config adapters for PerceptionLab.

mod local_sample_storage;
mod transient_annotation_repository;
mod transient_dataset_repository;
mod transient_sample_repository;

pub use local_sample_storage::LocalSampleStorage;
pub use transient_annotation_repository::TransientAnnotationRepository;
pub use transient_dataset_repository::TransientDatasetRepository;
pub use transient_sample_repository::TransientSampleRepository;

pub const CRATE_NAME: &str = "perception_infra";
