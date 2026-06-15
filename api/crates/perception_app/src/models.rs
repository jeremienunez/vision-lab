use std::collections::BTreeMap;

use perception_domain::{
    AnnotationId, DatasetId, DatasetStatus, DatasetVersionId, ExportStatus, InferenceRunId,
    ModelExportId, ModelId, ModelStatus, NormalizedBbox, SampleId, TrainingHyperparameters,
    TrainingJobId, TrainingJobStatus, TrainingMetricId,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetVersionDraft {
    pub id: DatasetVersionId,
    pub dataset_id: DatasetId,
    pub version_name: String,
    pub sample_count: u64,
    pub annotation_count: u64,
    pub classes_snapshot: Vec<String>,
    pub split_config: BTreeMap<String, String>,
    pub created_by: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingJobDraft {
    pub id: TrainingJobId,
    pub dataset_version_id: DatasetVersionId,
    pub model_family: String,
    pub base_model: Option<String>,
    pub status: TrainingJobStatus,
    pub hyperparameters: TrainingHyperparameters,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingJobQueueStatus {
    Queued,
    Leased,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainingJobQueueEntry {
    pub training_job_id: TrainingJobId,
    pub status: TrainingJobQueueStatus,
    pub locked_by: Option<String>,
    pub attempts: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingMetricDraft {
    pub id: TrainingMetricId,
    pub training_job_id: TrainingJobId,
    pub split_name: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub step: Option<u32>,
    pub epoch: Option<u32>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingClassMetric {
    pub training_job_id: TrainingJobId,
    pub class_name: String,
    pub split_name: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub step: Option<u32>,
    pub epoch: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDraft {
    pub id: ModelId,
    pub name: String,
    pub version: String,
    pub training_job_id: TrainingJobId,
    pub dataset_version_id: DatasetVersionId,
    pub model_family: String,
    pub artifact_uri: String,
    pub metrics_summary: BTreeMap<String, String>,
    pub status: ModelStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelExportDraft {
    pub id: ModelExportId,
    pub model_id: ModelId,
    pub format: String,
    pub artifact_uri: Option<String>,
    pub status: ExportStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DetectionDraft {
    pub class_id: u32,
    pub class_name: String,
    pub confidence: f32,
    pub bbox: NormalizedBbox,
    pub distance_m: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InferenceRunDraft {
    pub id: InferenceRunId,
    pub model_id: ModelId,
    pub filename: String,
    pub mime_type: String,
    pub latency_ms: u32,
    pub detections: Vec<DetectionDraft>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayArtifact {
    pub inference_run_id: InferenceRunId,
    pub artifact_uri: String,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InferenceRequest {
    pub model: ModelDraft,
    pub filename: String,
    pub mime_type: String,
    pub image_bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InferenceResult {
    pub run_id: InferenceRunId,
    pub model_id: ModelId,
    pub latency_ms: u32,
    pub detections: Vec<DetectionDraft>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoloAnnotationExport {
    pub dataset_id: DatasetId,
    pub classes_txt: String,
    pub files: Vec<YoloAnnotationFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoloAnnotationFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoloAnnotationImportFile {
    pub sample_filename: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YoloAnnotationImportResult {
    pub dataset_id: DatasetId,
    pub imported_count: usize,
}

impl TrainingJobQueueEntry {
    pub fn queued(training_job_id: TrainingJobId) -> Self {
        Self {
            training_job_id,
            status: TrainingJobQueueStatus::Queued,
            locked_by: None,
            attempts: 0,
        }
    }
}
