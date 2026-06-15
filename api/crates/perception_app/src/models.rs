use std::collections::BTreeMap;

use perception_domain::{AnnotationId, DatasetId, DatasetStatus, SampleId};

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

#[derive(Debug, Clone, PartialEq)]
pub struct AnnotationDraft {
    pub id: AnnotationId,
    pub sample_id: SampleId,
    pub dataset_id: DatasetId,
    pub class_name: String,
    pub class_id: u32,
    pub bbox_x: f32,
    pub bbox_y: f32,
    pub bbox_width: f32,
    pub bbox_height: f32,
    pub format: String,
    pub confidence: Option<f32>,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetStats {
    pub dataset_id: DatasetId,
    pub sample_count: u64,
    pub annotation_count: u64,
    pub annotations_by_class: BTreeMap<String, u64>,
}
