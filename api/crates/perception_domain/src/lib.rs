#![forbid(unsafe_code)]
//! Pure domain types, value objects, and state machines for PerceptionLab.

mod error;
mod ids;
mod state;
mod value_objects;

pub use error::DomainError;
pub use ids::{
    AnnotationId, ArtifactId, DatasetId, DatasetVersionId, ModelId, SampleId, TrainingJobId,
};
pub use state::{DatasetStatus, ExportStatus, ModelStatus, TrainingJobStatus};
pub use value_objects::{ImageDimensions, NormalizedBbox, TrainingHyperparameters};

pub const CRATE_NAME: &str = "perception_domain";
