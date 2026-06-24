use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    AnnotationDraft, AnnotationRepository, CreateDatasetVersionCommand,
    CreateDatasetVersionUseCase, DatasetDraft, DatasetRepository, DatasetVersionDraft,
    DatasetVersionRepository, SampleDraft, SampleRepository, TaskType, UseCaseError,
};
use perception_domain::{AnnotationId, DatasetId, DatasetStatus, DatasetVersionId, SampleId};

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

#[derive(Default)]
struct InMemoryAnnotationRepository {
    annotations: Mutex<Vec<AnnotationDraft>>,
}

#[async_trait]
impl AnnotationRepository for InMemoryAnnotationRepository {
    async fn create(&self, annotation: AnnotationDraft) -> Result<AnnotationDraft, UseCaseError> {
        self.annotations
            .lock()
            .expect("repository mutex is available")
            .push(annotation.clone());
        Ok(annotation)
    }

    async fn list_by_sample(
        &self,
        sample_id: SampleId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError> {
        Ok(self
            .annotations
            .lock()
            .expect("repository mutex is available")
            .iter()
            .filter(|annotation| annotation.sample_id == sample_id)
            .cloned()
            .collect())
    }

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError> {
        Ok(self
            .annotations
            .lock()
            .expect("repository mutex is available")
            .iter()
            .filter(|annotation| annotation.dataset_id == dataset_id)
            .cloned()
            .collect())
    }
}

#[derive(Default)]
struct InMemoryDatasetVersionRepository {
    versions: Mutex<Vec<DatasetVersionDraft>>,
}

#[async_trait]
impl DatasetVersionRepository for InMemoryDatasetVersionRepository {
    async fn create(
        &self,
        version: DatasetVersionDraft,
    ) -> Result<DatasetVersionDraft, UseCaseError> {
        self.versions
            .lock()
            .expect("repository mutex is available")
            .push(version.clone());
        Ok(version)
    }

    async fn get(
        &self,
        version_id: DatasetVersionId,
    ) -> Result<Option<DatasetVersionDraft>, UseCaseError> {
        Ok(self
            .versions
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|version| version.id == version_id)
            .cloned())
    }

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<DatasetVersionDraft>, UseCaseError> {
        Ok(self
            .versions
            .lock()
            .expect("repository mutex is available")
            .iter()
            .filter(|version| version.dataset_id == dataset_id)
            .cloned()
            .collect())
    }
}

fn dataset_fixture() -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: "desk-objects-v1".to_owned(),
        description: None,
        task_type: TaskType::ObjectDetection,
        classes: vec!["cup".to_owned(), "book".to_owned()],
        status: DatasetStatus::Draft,
    }
}

fn sample_fixture(dataset_id: DatasetId) -> SampleDraft {
    SampleDraft {
        id: SampleId::new(),
        dataset_id,
        storage_uri: "memory://sample".to_owned(),
        filename: "cup.png".to_owned(),
        mime_type: "image/png".to_owned(),
        width: 640,
        height: 480,
        size_bytes: 14,
        checksum: "sha256:test".to_owned(),
        source: "upload".to_owned(),
        metadata: BTreeMap::new(),
    }
}

#[tokio::test]
async fn create_dataset_version_captures_dataset_snapshot_counts_and_classes() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();
    let versions = InMemoryDatasetVersionRepository::default();
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    let sample = samples
        .create(sample_fixture(dataset.id))
        .await
        .expect("sample is created");
    annotations
        .create(AnnotationDraft {
            id: AnnotationId::new(),
            sample_id: sample.id,
            dataset_id: dataset.id,
            class_name: "cup".to_owned(),
            class_id: 0,
            bbox_x: 0.10,
            bbox_y: 0.20,
            bbox_width: 0.30,
            bbox_height: 0.40,
            format: "normalized_xywh".to_owned(),
            confidence: Some(0.91),
            source: "manual".to_owned(),
        })
        .await
        .expect("annotation is created");

    let version = CreateDatasetVersionUseCase::new(&datasets, &samples, &annotations, &versions)
        .execute(CreateDatasetVersionCommand {
            dataset_id: dataset.id,
            version_name: "v1".to_owned(),
            split_config: BTreeMap::new(),
            created_by: "local-user".to_owned(),
        })
        .await
        .expect("version is created");

    assert_eq!(version.dataset_id, dataset.id);
    assert_eq!(version.version_name, "v1");
    assert_eq!(version.sample_count, 1);
    assert_eq!(version.annotation_count, 1);
    assert_eq!(version.classes_snapshot, ["cup", "book"]);
    assert_eq!(version.created_by, "local-user");
}

#[tokio::test]
async fn create_dataset_version_rejects_dataset_without_samples() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();
    let versions = InMemoryDatasetVersionRepository::default();
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");

    let result = CreateDatasetVersionUseCase::new(&datasets, &samples, &annotations, &versions)
        .execute(CreateDatasetVersionCommand {
            dataset_id: dataset.id,
            version_name: "v1".to_owned(),
            split_config: BTreeMap::new(),
            created_by: "local-user".to_owned(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation(
            "dataset version requires at least one sample"
        ))
    );
}

#[tokio::test]
async fn create_dataset_version_persists_valid_split_config() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();
    let versions = InMemoryDatasetVersionRepository::default();
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    samples
        .create(sample_fixture(dataset.id))
        .await
        .expect("sample is created");
    let split_config = BTreeMap::from([
        ("train".to_owned(), "70".to_owned()),
        ("validation".to_owned(), "20".to_owned()),
        ("test".to_owned(), "10".to_owned()),
    ]);

    let version = CreateDatasetVersionUseCase::new(&datasets, &samples, &annotations, &versions)
        .execute(CreateDatasetVersionCommand {
            dataset_id: dataset.id,
            version_name: "v2".to_owned(),
            split_config: split_config.clone(),
            created_by: "local-user".to_owned(),
        })
        .await
        .expect("version is created");

    assert_eq!(version.split_config, split_config);
}

#[tokio::test]
async fn create_dataset_version_rejects_split_config_that_does_not_sum_to_100() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();
    let versions = InMemoryDatasetVersionRepository::default();
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    samples
        .create(sample_fixture(dataset.id))
        .await
        .expect("sample is created");

    let result = CreateDatasetVersionUseCase::new(&datasets, &samples, &annotations, &versions)
        .execute(CreateDatasetVersionCommand {
            dataset_id: dataset.id,
            version_name: "v2".to_owned(),
            split_config: BTreeMap::from([
                ("train".to_owned(), "80".to_owned()),
                ("validation".to_owned(), "20".to_owned()),
                ("test".to_owned(), "20".to_owned()),
            ]),
            created_by: "local-user".to_owned(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation("dataset split must sum to 100"))
    );
}
