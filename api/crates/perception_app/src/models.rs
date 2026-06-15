use perception_domain::{DatasetId, DatasetStatus};

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
