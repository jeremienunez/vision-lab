use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{
    AnnotationDraft, AnnotationRepository, DatasetDraft, DatasetRepository, SampleDraft,
    SampleRepository, SampleStorage, SampleStorageCommand, StoredSample, UseCaseError,
};
use perception_domain::{DatasetId, SampleId};
use serde_json::{Value, json};
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
            checksum: "sha256:route-checksum".to_owned(),
        })
    }
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), 4096)
        .await
        .expect("body is readable");
    serde_json::from_slice(&body).expect("body is JSON")
}

fn multipart_body() -> (String, String) {
    let boundary = "perceptionlab-boundary";
    let body = format!(
        "--{boundary}\r\n\
Content-Disposition: form-data; name=\"width\"\r\n\r\n\
640\r\n\
--{boundary}\r\n\
Content-Disposition: form-data; name=\"height\"\r\n\r\n\
480\r\n\
--{boundary}\r\n\
Content-Disposition: form-data; name=\"file\"; filename=\"cup.png\"\r\n\
Content-Type: image/png\r\n\r\n\
fake-png-bytes\r\n\
--{boundary}--\r\n"
    );

    (boundary.to_owned(), body)
}

#[tokio::test]
async fn dataset_stats_route_returns_counts_after_sample_and_annotation_creation() {
    let app = perception_http::router_with_annotation_ports(
        Arc::new(RouteDatasetRepository::default()),
        Arc::new(RouteSampleRepository::default()),
        Arc::new(RouteSampleStorage),
        Arc::new(RouteAnnotationRepository::default()),
    );

    let create_dataset_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/datasets")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    json!({
                        "name": "desk-objects-v1",
                        "description": null,
                        "task_type": "object_detection",
                        "classes": ["cup", "book"]
                    })
                    .to_string(),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");
    let dataset = json_body(create_dataset_response).await;
    let dataset_id = dataset["id"].as_str().expect("dataset id is present");
    let (boundary, body) = multipart_body();
    let upload_sample_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/datasets/{dataset_id}/samples"))
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={boundary}"),
                )
                .body(axum::body::Body::from(body))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");
    let sample = json_body(upload_sample_response).await;
    let sample_id = sample["id"].as_str().expect("sample id is present");

    let create_annotation_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/samples/{sample_id}/annotations"))
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    json!({
                        "class_name": "book",
                        "bbox": {"x": 0.10, "y": 0.20, "width": 0.30, "height": 0.40},
                        "confidence": 0.92
                    })
                    .to_string(),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");
    assert_eq!(create_annotation_response.status(), StatusCode::CREATED);

    let stats_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/datasets/{dataset_id}/stats"))
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(stats_response.status(), StatusCode::OK);
    let stats = json_body(stats_response).await;
    assert_eq!(stats["dataset_id"], dataset_id);
    assert_eq!(stats["sample_count"], 1);
    assert_eq!(stats["annotation_count"], 1);
    assert_eq!(stats["annotations_by_class"]["book"], 1);
}
