use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("identifier is not a valid UUID")]
    InvalidId,
    #[error("normalized bounding box must stay within [0, 1]")]
    InvalidNormalizedBbox,
    #[error("image dimensions must be non-zero")]
    InvalidImageDimensions,
    #[error("training hyperparameters must be positive")]
    InvalidTrainingHyperparameters,
    #[error("status transition is not allowed")]
    InvalidStatusTransition,
}
