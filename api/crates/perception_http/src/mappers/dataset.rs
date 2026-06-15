use perception_app::{CreateDatasetCommand, DatasetDraft, TaskType, UseCaseError};
use perception_domain::DatasetStatus;

use crate::dto::dataset::{CreateDatasetRequest, DatasetResponse};

pub fn create_dataset_command(
    request: CreateDatasetRequest,
) -> Result<CreateDatasetCommand, UseCaseError> {
    let task_type = match request.task_type.as_str() {
        "object_detection" => TaskType::ObjectDetection,
        _ => return Err(UseCaseError::Validation("unsupported task_type")),
    };

    Ok(CreateDatasetCommand {
        name: request.name,
        description: request.description,
        task_type,
        classes: request.classes,
    })
}

pub fn dataset_response(dataset: DatasetDraft) -> DatasetResponse {
    DatasetResponse {
        id: dataset.id.to_string(),
        name: dataset.name,
        description: dataset.description,
        task_type: task_type_name(dataset.task_type),
        classes: dataset.classes,
        status: dataset_status_name(dataset.status),
    }
}

fn task_type_name(task_type: TaskType) -> &'static str {
    match task_type {
        TaskType::ObjectDetection => "object_detection",
    }
}

fn dataset_status_name(status: DatasetStatus) -> &'static str {
    match status {
        DatasetStatus::Draft => "draft",
        DatasetStatus::Ready => "ready",
        DatasetStatus::Archived => "archived",
    }
}
