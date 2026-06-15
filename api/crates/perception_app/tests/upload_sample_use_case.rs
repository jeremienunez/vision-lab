use std::sync::Mutex;

use async_trait::async_trait;
use perception_app::{
    DatasetDraft, DatasetRepository, SampleDraft, SampleRepository, SampleStorage,
    SampleStorageCommand, StoredSample, TaskType, UploadSampleCommand, UploadSampleUseCase,
    UseCaseError,
};
use perception_domain::{DatasetId, DatasetStatus, SampleId};

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

    async fn get(&self, dataset_id: DatasetId) -> Result<Option<DatasetDraft>, UseCaseError> {
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

#[derive(Default)]
struct InMemorySampleRepository {
    samples: Mutex<Vec<SampleDraft>>,
}

#[async_trait]
impl SampleRepository for InMemorySampleRepository {
    async fn create(&self, sample: SampleDraft) -> Result<SampleDraft, UseCaseError> {
        self.samples
            .lock()
            .expect("repository mutex is available")
            .push(sample.clone());
        Ok(sample)
    }

    async fn get(&self, sample_id: SampleId) -> Result<Option<SampleDraft>, UseCaseError> {
        Ok(self
            .samples
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|sample| sample.id == sample_id)
            .cloned())
    }

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<SampleDraft>, UseCaseError> {
        Ok(self
            .samples
            .lock()
            .expect("repository mutex is available")
            .iter()
            .filter(|sample| sample.dataset_id == dataset_id)
            .cloned()
            .collect())
    }
}

struct RecordingSampleStorage;

#[async_trait]
impl SampleStorage for RecordingSampleStorage {
    async fn store(&self, command: SampleStorageCommand) -> Result<StoredSample, UseCaseError> {
        Ok(StoredSample {
            storage_uri: format!(
                "memory://datasets/{}/samples/{}",
                command.dataset_id, command.sample_id
            ),
            size_bytes: command.bytes.len() as u64,
            checksum: "sha256:test-checksum".to_owned(),
        })
    }
}

fn dataset_fixture() -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: "desk-objects-v1".to_owned(),
        description: None,
        task_type: TaskType::ObjectDetection,
        classes: vec!["cup".to_owned()],
        status: DatasetStatus::Draft,
    }
}

#[tokio::test]
async fn upload_sample_stores_image_and_persists_metadata_for_existing_dataset() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let storage = RecordingSampleStorage;
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    let use_case = UploadSampleUseCase::new(&datasets, &samples, &storage);

    let sample = use_case
        .execute(UploadSampleCommand {
            dataset_id: dataset.id,
            filename: "cup.png".to_owned(),
            mime_type: "image/png".to_owned(),
            width: 640,
            height: 480,
            bytes: b"fake-png-bytes".to_vec(),
        })
        .await
        .expect("sample is uploaded");

    assert_eq!(sample.dataset_id, dataset.id);
    assert_eq!(sample.filename, "cup.png");
    assert_eq!(sample.mime_type, "image/png");
    assert_eq!(sample.width, 640);
    assert_eq!(sample.height, 480);
    assert_eq!(sample.size_bytes, 14);
    assert_eq!(sample.checksum, "sha256:test-checksum");
    assert_eq!(sample.source, "upload");
    assert!(sample.storage_uri.starts_with("memory://datasets/"));
}

#[tokio::test]
async fn upload_sample_rejects_missing_dataset() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let storage = RecordingSampleStorage;
    let use_case = UploadSampleUseCase::new(&datasets, &samples, &storage);

    let result = use_case
        .execute(UploadSampleCommand {
            dataset_id: DatasetId::new(),
            filename: "cup.png".to_owned(),
            mime_type: "image/png".to_owned(),
            width: 640,
            height: 480,
            bytes: b"fake-png-bytes".to_vec(),
        })
        .await;

    assert_eq!(result, Err(UseCaseError::NotFound("dataset not found")));
}

#[tokio::test]
async fn upload_sample_rejects_non_image_mime_type() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let storage = RecordingSampleStorage;
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    let use_case = UploadSampleUseCase::new(&datasets, &samples, &storage);

    let result = use_case
        .execute(UploadSampleCommand {
            dataset_id: dataset.id,
            filename: "notes.txt".to_owned(),
            mime_type: "text/plain".to_owned(),
            width: 640,
            height: 480,
            bytes: b"not an image".to_vec(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation("unsupported image mime type"))
    );
}
