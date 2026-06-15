use crate::{DatasetDraft, DatasetRepository, TaskType, UseCaseError};
use perception_domain::{DatasetId, DatasetStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateDatasetCommand {
    pub name: String,
    pub description: Option<String>,
    pub task_type: TaskType,
    pub classes: Vec<String>,
}

pub struct CreateDatasetUseCase<'repository> {
    repository: &'repository dyn DatasetRepository,
}

impl<'repository> CreateDatasetUseCase<'repository> {
    pub fn new(repository: &'repository dyn DatasetRepository) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        command: CreateDatasetCommand,
    ) -> Result<DatasetDraft, UseCaseError> {
        if command.name.trim().is_empty() {
            return Err(UseCaseError::Validation("dataset name is required"));
        }

        let dataset = DatasetDraft {
            id: DatasetId::new(),
            name: command.name.trim().to_owned(),
            description: command.description,
            task_type: command.task_type,
            classes: command.classes,
            status: DatasetStatus::Draft,
        };

        self.repository.create(dataset).await
    }
}
