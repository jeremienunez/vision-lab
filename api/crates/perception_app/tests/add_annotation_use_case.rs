use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    AddAnnotationCommand, AddAnnotationUseCase, AnnotationDraft, AnnotationRepository,
    DatasetDraft, DatasetRepository, ListSampleAnnotationsUseCase, SampleDraft, SampleRepository,
    TaskType, UseCaseError,
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
}

#[derive(Default)]
struct InMemoryAnnotationRepository {
    annotations: Mutex<Vec<AnnotationDraft>>,
}

#[async_trait]
impl AnnotationRepository for InMemoryAnnotationRepository {
    async fn create(
        &self,
        annotation: AnnotationDraft,
    ) -> Result<AnnotationDraft, UseCaseError> {
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
async fn add_annotation_persists_normalized_bbox_for_existing_sample_and_class() {
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
    let use_case = AddAnnotationUseCase::new(&datasets, &samples, &annotations);

    let annotation = use_case
        .execute(AddAnnotationCommand {
            sample_id: sample.id,
            class_name: "book".to_owned(),
            bbox_x: 0.10,
            bbox_y: 0.20,
            bbox_width: 0.30,
            bbox_height: 0.40,
            confidence: Some(0.92),
        })
        .await
        .expect("annotation is added");

    assert_eq!(annotation.sample_id, sample.id);
    assert_eq!(annotation.dataset_id, dataset.id);
    assert_eq!(annotation.class_name, "book");
    assert_eq!(annotation.class_id, 1);
    assert_eq!(annotation.format, "normalized_xywh");
    assert_eq!(annotation.source, "manual");

    let listed = ListSampleAnnotationsUseCase::new(&samples, &annotations)
        .execute(sample.id)
        .await
        .expect("annotations are listed");

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].class_name, "book");
}

#[tokio::test]
async fn add_annotation_rejects_unknown_class() {
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
    let use_case = AddAnnotationUseCase::new(&datasets, &samples, &annotations);

    let result = use_case
        .execute(AddAnnotationCommand {
            sample_id: sample.id,
            class_name: "phone".to_owned(),
            bbox_x: 0.10,
            bbox_y: 0.20,
            bbox_width: 0.30,
            bbox_height: 0.40,
            confidence: None,
        })
        .await;

    assert_eq!(result, Err(UseCaseError::Validation("unknown dataset class")));
}

#[tokio::test]
async fn add_annotation_rejects_invalid_bbox() {
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
    let use_case = AddAnnotationUseCase::new(&datasets, &samples, &annotations);

    let result = use_case
        .execute(AddAnnotationCommand {
            sample_id: sample.id,
            class_name: "cup".to_owned(),
            bbox_x: 0.90,
            bbox_y: 0.20,
            bbox_width: 0.30,
            bbox_height: 0.40,
            confidence: None,
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation("invalid normalized bbox"))
    );
}
