use serde::{Deserialize, Serialize};

use crate::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetStatus {
    Draft,
    Ready,
    Archived,
}

impl DatasetStatus {
    pub fn transition_to(self, next: Self) -> Result<Self, DomainError> {
        match (self, next) {
            (Self::Draft, Self::Ready | Self::Archived) => Ok(next),
            (Self::Ready, Self::Archived) => Ok(next),
            (current, next) if current == next => Ok(next),
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainingJobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl TrainingJobStatus {
    pub fn transition_to(self, next: Self) -> Result<Self, DomainError> {
        match (self, next) {
            (Self::Queued, Self::Running | Self::Cancelled) => Ok(next),
            (Self::Running, Self::Succeeded | Self::Failed) => Ok(next),
            (current, next) if current == next => Ok(next),
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelStatus {
    Candidate,
    Validated,
    Promoted,
    Archived,
}

impl ModelStatus {
    pub fn transition_to(self, next: Self) -> Result<Self, DomainError> {
        match (self, next) {
            (Self::Candidate, Self::Validated | Self::Archived) => Ok(next),
            (Self::Validated, Self::Promoted | Self::Archived) => Ok(next),
            (Self::Promoted, Self::Archived) => Ok(next),
            (current, next) if current == next => Ok(next),
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
}

impl ExportStatus {
    pub fn transition_to(self, next: Self) -> Result<Self, DomainError> {
        match (self, next) {
            (Self::Queued, Self::Running) => Ok(next),
            (Self::Running, Self::Succeeded | Self::Failed) => Ok(next),
            (current, next) if current == next => Ok(next),
            _ => Err(DomainError::InvalidStatusTransition),
        }
    }
}
