use perception_app::{DatasetDraft, DatasetRepository, TaskType};
use perception_domain::{DatasetId, DatasetStatus};
use perception_infra::TransientDatasetRepository;

#[tokio::test]
async fn transient_dataset_repository_creates_and_lists_datasets() {
    let repository = TransientDatasetRepository::default();

    repository
        .create(DatasetDraft {
            id: DatasetId::new(),
            name: "desk-objects-v1".to_owned(),
            description: None,
            task_type: TaskType::ObjectDetection,
            classes: vec!["cup".to_owned()],
            status: DatasetStatus::Draft,
        })
        .await
        .expect("dataset is persisted");

    let datasets = repository.list().await.expect("datasets are listed");

    assert_eq!(datasets.len(), 1);
    assert_eq!(datasets[0].name, "desk-objects-v1");
}
