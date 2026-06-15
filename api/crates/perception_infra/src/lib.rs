#![forbid(unsafe_code)]
//! PostgreSQL, storage, queue, and config adapters for PerceptionLab.

mod transient_dataset_repository;

pub use transient_dataset_repository::TransientDatasetRepository;

pub const CRATE_NAME: &str = "perception_infra";
