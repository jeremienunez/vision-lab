use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum UseCaseError {
    #[error("validation failed: {0}")]
    Validation(&'static str),
    #[error("not found: {0}")]
    NotFound(&'static str),
    #[error("repository failed: {0}")]
    Repository(&'static str),
}
