use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    AnnotationDraft, AnnotationRepository, DatasetDraft, DatasetRepository,
    ImportYoloAnnotationsCommand, ImportYoloAnnotationsUseCase, SampleDraft, SampleRepository,
    TaskType, UseCaseError, YoloAnnotationImportFile,
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

fn sample_fixture(dataset_id: DatasetId, filename: &str) -> SampleDraft {
    SampleDraft {
        id: SampleId::new(),
        dataset_id,
        storage_uri: format!("memory://samples/{filename}"),
        filename: filename.to_owned(),
        mime_type: "image/jpeg".to_owned(),
        width: 640,
        height: 480,
        size_bytes: 14,
        checksum: format!("sha256:{filename}"),
        source: "upload".to_owned(),
        metadata: BTreeMap::new(),
    }
}

#[tokio::test]
async fn import_yolo_annotations_persists_normalized_xywh_annotations() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    let sample = samples
        .create(sample_fixture(dataset.id, "cup.jpg"))
        .await
        .expect("sample is created");

    let result = ImportYoloAnnotationsUseCase::new(&datasets, &samples, &annotations)
        .execute(ImportYoloAnnotationsCommand {
            dataset_id: dataset.id,
            files: vec![YoloAnnotationImportFile {
                sample_filename: "cup.jpg".to_owned(),
                content: "1 0.250000 0.400000 0.300000 0.400000\n".to_owned(),
            }],
        })
        .await
        .expect("YOLO annotations are imported");

    assert_eq!(result.dataset_id, dataset.id);
    assert_eq!(result.imported_count, 1);

    let listed = annotations
        .list_by_sample(sample.id)
        .await
        .expect("annotations are listed");

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].sample_id, sample.id);
    assert_eq!(listed[0].dataset_id, dataset.id);
    assert_eq!(listed[0].class_id, 1);
    assert_eq!(listed[0].class_name, "book");
    assert!((listed[0].bbox_x - 0.10).abs() < 0.0001);
    assert!((listed[0].bbox_y - 0.20).abs() < 0.0001);
    assert!((listed[0].bbox_width - 0.30).abs() < 0.0001);
    assert!((listed[0].bbox_height - 0.40).abs() < 0.0001);
    assert_eq!(listed[0].format, "normalized_xywh");
    assert_eq!(listed[0].source, "yolo_import");
}

#[tokio::test]
async fn import_yolo_annotations_rejects_unknown_class_id() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    samples
        .create(sample_fixture(dataset.id, "cup.jpg"))
        .await
        .expect("sample is created");

    let result = ImportYoloAnnotationsUseCase::new(&datasets, &samples, &annotations)
        .execute(ImportYoloAnnotationsCommand {
            dataset_id: dataset.id,
            files: vec![YoloAnnotationImportFile {
                sample_filename: "cup.jpg".to_owned(),
                content: "9 0.250000 0.400000 0.300000 0.400000\n".to_owned(),
            }],
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation("unknown yolo class id"))
    );
}

#[tokio::test]
async fn import_yolo_annotations_rejects_unknown_sample_filename() {
    let datasets = InMemoryDatasetRepository::default();
    let samples = InMemorySampleRepository::default();
    let annotations = InMemoryAnnotationRepository::default();
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");

    let result = ImportYoloAnnotationsUseCase::new(&datasets, &samples, &annotations)
        .execute(ImportYoloAnnotationsCommand {
            dataset_id: dataset.id,
            files: vec![YoloAnnotationImportFile {
                sample_filename: "missing.jpg".to_owned(),
                content: "0 0.250000 0.400000 0.300000 0.400000\n".to_owned(),
            }],
        })
        .await;

    assert_eq!(result, Err(UseCaseError::NotFound("sample not found")));
}
