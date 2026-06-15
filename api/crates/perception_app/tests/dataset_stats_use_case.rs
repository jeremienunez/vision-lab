use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    AnnotationDraft, AnnotationRepository, DatasetDraft, DatasetRepository, DatasetStatsUseCase,
    SampleDraft, SampleRepository, TaskType, UseCaseError,
};
use perception_domain::{AnnotationId, DatasetId, DatasetStatus, SampleId};

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

fn annotation_fixture(
    sample_id: SampleId,
    dataset_id: DatasetId,
    class_name: &str,
) -> AnnotationDraft {
    AnnotationDraft {
        id: AnnotationId::new(),
        sample_id,
        dataset_id,
        class_name: class_name.to_owned(),
        class_id: if class_name == "cup" { 0 } else { 1 },
        bbox_x: 0.10,
        bbox_y: 0.20,
        bbox_width: 0.30,
        bbox_height: 0.40,
        format: "normalized_xywh".to_owned(),
        confidence: Some(0.91),
        source: "manual".to_owned(),
    }
}

#[tokio::test]
async fn dataset_stats_count_samples_annotations_and_classes() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    let sample = samples
        .create(sample_fixture(dataset.id))
        .await
        .expect("sample is created");
    annotations
        .create(annotation_fixture(sample.id, dataset.id, "cup"))
        .await
        .expect("annotation is created");
    annotations
        .create(annotation_fixture(sample.id, dataset.id, "book"))
        .await
        .expect("annotation is created");

    let stats = DatasetStatsUseCase::new(&datasets, &samples, &annotations)
        .execute(dataset.id)
        .await
        .expect("stats are computed");

    assert_eq!(stats.dataset_id, dataset.id);
    assert_eq!(stats.sample_count, 1);
    assert_eq!(stats.annotation_count, 2);
    assert_eq!(stats.annotations_by_class.get("cup"), Some(&1));
    assert_eq!(stats.annotations_by_class.get("book"), Some(&1));
}

#[tokio::test]
async fn dataset_stats_reject_missing_dataset() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();

    let result = DatasetStatsUseCase::new(&datasets, &samples, &annotations)
        .execute(DatasetId::new())
        .await;

    assert_eq!(result, Err(UseCaseError::NotFound("dataset not found")));
}
