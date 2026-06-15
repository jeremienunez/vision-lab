use std::sync::Mutex;

use async_trait::async_trait;
use perception_app::{
    CreateDatasetCommand, CreateDatasetUseCase, DatasetDraft, DatasetRepository, TaskType,
    UseCaseError,
};
use perception_domain::DatasetStatus;

#[derive(Default)]
struct InMemoryDatasetRepository {
    datasets: Mutex<Vec<DatasetDraft>>,
}

#[async_trait]
impl DatasetRepository for InMemoryDatasetRepository {
    async fn create(&self, dataset: DatasetDraft) -> Result<DatasetDraft, UseCaseError> {
        self.datasets
            .lock()
            .expect("repository mutex is available")
            .push(dataset.clone());
        Ok(dataset)
    }
}

#[tokio::test]
async fn create_dataset_persists_a_draft_dataset_through_repository_port() {
    let repository = InMemoryDatasetRepository::default();
    let use_case = CreateDatasetUseCase::new(&repository);

    let result = use_case
        .execute(CreateDatasetCommand {
            name: "desk-objects-v1".to_owned(),
            description: Some("Desk object detection dataset".to_owned()),
            task_type: TaskType::ObjectDetection,
            classes: vec!["cup".to_owned(), "book".to_owned()],
        })
        .await
        .expect("dataset is created");

    assert_eq!(result.name, "desk-objects-v1");
    assert_eq!(result.status, DatasetStatus::Draft);
    assert_eq!(result.classes, ["cup", "book"]);
}

#[tokio::test]
async fn create_dataset_rejects_empty_name() {
    let repository = InMemoryDatasetRepository::default();
    let use_case = CreateDatasetUseCase::new(&repository);

    let result = use_case
        .execute(CreateDatasetCommand {
            name: " ".to_owned(),
            description: None,
            task_type: TaskType::ObjectDetection,
            classes: vec!["cup".to_owned()],
        })
        .await;

    assert_eq!(result, Err(UseCaseError::Validation("dataset name is required")));
}
