use std::collections::BTreeMap;

use perception_domain::{DatasetId, DatasetStatus, SampleId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskType {
    ObjectDetection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetDraft {
    pub id: DatasetId,
    pub name: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub classes: Vec<String>,
    pub status: DatasetStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleDraft {
    pub id: SampleId,
    pub dataset_id: DatasetId,
    pub storage_uri: String,
    pub filename: String,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    pub size_bytes: u64,
    pub checksum: String,
    pub source: String,
    pub metadata: BTreeMap<String, String>,
}
