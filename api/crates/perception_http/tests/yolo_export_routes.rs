use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{
    AnnotationDraft, AnnotationRepository, DatasetDraft, DatasetRepository, SampleDraft,
    SampleRepository, SampleStorage, SampleStorageCommand, StoredSample, TaskType, UseCaseError,
};
use perception_domain::{AnnotationId, DatasetId, DatasetStatus, SampleId};
use serde_json::Value;
use tower::ServiceExt;

#[derive(Default)]
struct RouteDatasetRepository {
    datasets: Mutex<Vec<DatasetDraft>>,
}

#[async_trait]
impl DatasetRepository for RouteDatasetRepository {
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
struct RouteSampleRepository {
    samples: Mutex<Vec<SampleDraft>>,
}

#[async_trait]
impl SampleRepository for RouteSampleRepository {
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
struct RouteAnnotationRepository {
    annotations: Mutex<Vec<AnnotationDraft>>,
}

#[async_trait]
impl AnnotationRepository for RouteAnnotationRepository {
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

struct RouteSampleStorage;

#[async_trait]
impl SampleStorage for RouteSampleStorage {
    async fn store(&self, command: SampleStorageCommand) -> Result<StoredSample, UseCaseError> {
        Ok(StoredSample {
            storage_uri: format!("memory://samples/{}", command.sample_id),
            size_bytes: command.bytes.len() as u64,
            checksum: "sha256:test".to_owned(),
        })
    }
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), 8192)
        .await
        .expect("body is readable");
    serde_json::from_slice(&body).expect("body is JSON")
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
        storage_uri: "memory://samples/cup.jpg".to_owned(),
        filename: "cup.jpg".to_owned(),
        mime_type: "image/jpeg".to_owned(),
        width: 640,
        height: 480,
        size_bytes: 14,
        checksum: "sha256:cup".to_owned(),
        source: "upload".to_owned(),
        metadata: BTreeMap::new(),
    }
}

fn annotation_fixture(dataset_id: DatasetId, sample_id: SampleId) -> AnnotationDraft {
    AnnotationDraft {
        id: AnnotationId::new(),
        sample_id,
        dataset_id,
        class_name: "cup".to_owned(),
        class_id: 0,
        bbox_x: 0.10,
        bbox_y: 0.20,
        bbox_width: 0.30,
        bbox_height: 0.40,
        format: "normalized_xywh".to_owned(),
        confidence: None,
        source: "manual".to_owned(),
    }
}

#[tokio::test]
async fn export_yolo_route_returns_classes_and_label_files() {
    let datasets = Arc::new(RouteDatasetRepository::default());
    let samples = Arc::new(RouteSampleRepository::default());
    let annotations = Arc::new(RouteAnnotationRepository::default());
    let dataset = datasets
        .create(dataset_fixture())
        .await
        .expect("dataset is created");
    let sample = samples
        .create(sample_fixture(dataset.id))
        .await
        .expect("sample is created");
    annotations
        .create(annotation_fixture(dataset.id, sample.id))
        .await
        .expect("annotation is created");
    let app = perception_http::router_with_annotation_ports(
        datasets,
        samples,
        Arc::new(RouteSampleStorage),
        annotations,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/datasets/{}/export/yolo", dataset.id))
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
    let export = json_body(response).await;
    assert_eq!(export["dataset_id"], dataset.id.to_string());
    assert_eq!(export["classes_txt"], "cup\nbook\n");
    assert_eq!(export["files"][0]["path"], "labels/cup.txt");
    assert_eq!(
        export["files"][0]["content"],
        "0 0.250000 0.400000 0.300000 0.400000\n"
    );
}
