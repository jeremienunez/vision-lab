use async_trait::async_trait;
use perception_domain::{DatasetId, SampleId};

use crate::UseCaseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleStorageCommand {
    pub dataset_id: DatasetId,
    pub sample_id: SampleId,
    pub filename: String,
    pub mime_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredSample {
    pub storage_uri: String,
    pub size_bytes: u64,
    pub checksum: String,
}

#[async_trait]
pub trait SampleStorage: Send + Sync {
    async fn store(&self, command: SampleStorageCommand) -> Result<StoredSample, UseCaseError>;
}
