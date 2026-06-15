use std::sync::Mutex;

use async_trait::async_trait;
use perception_app::{
    CreateDatasetCommand, CreateDatasetUseCase, DatasetDraft, DatasetRepository,
    ListDatasetsUseCase, TaskType, UseCaseError,
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

    async fn get(
        &self,
        dataset_id: perception_domain::DatasetId,
    ) -> Result<Option<DatasetDraft>, UseCaseError> {
        Ok(self
            .datasets
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|dataset| dataset.id == dataset_id)
            .cloned())
    }

    async fn list(&self) -> Result<Vec<DatasetDraft>, UseCaseError> {
        Ok(self
            .datasets
            .lock()
            .expect("repository mutex is available")
            .clone())
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

    assert_eq!(
        result,
        Err(UseCaseError::Validation("dataset name is required"))
    );
}

#[tokio::test]
async fn list_datasets_returns_repository_datasets() {
    let repository = InMemoryDatasetRepository::default();
    let create_use_case = CreateDatasetUseCase::new(&repository);
    let list_use_case = ListDatasetsUseCase::new(&repository);

    create_use_case
        .execute(CreateDatasetCommand {
            name: "desk-objects-v1".to_owned(),
            description: None,
            task_type: TaskType::ObjectDetection,
            classes: vec!["cup".to_owned()],
        })
        .await
        .expect("dataset is created");

    let result = list_use_case.execute().await.expect("datasets are listed");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "desk-objects-v1");
}
