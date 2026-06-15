use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{
    AnnotationDraft, AnnotationRepository, DatasetDraft, DatasetRepository, DatasetVersionDraft,
    DatasetVersionRepository, SampleDraft, SampleRepository, SampleStorage, SampleStorageCommand,
    StoredSample, UseCaseError,
};
use perception_domain::{DatasetId, DatasetVersionId, SampleId};
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

#[derive(Default)]
struct RouteDatasetVersionRepository {
    versions: Mutex<Vec<DatasetVersionDraft>>,
}

#[async_trait]
impl DatasetVersionRepository for RouteDatasetVersionRepository {
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
async fn create_dataset_version_route_returns_immutable_snapshot_metadata() {
    let app = perception_http::router_with_version_ports(
        Arc::new(RouteDatasetRepository::default()),
        Arc::new(RouteSampleRepository::default()),
        Arc::new(RouteSampleStorage),
        Arc::new(RouteAnnotationRepository::default()),
        Arc::new(RouteDatasetVersionRepository::default()),
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
    assert_eq!(upload_sample_response.status(), StatusCode::CREATED);

    let version_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/datasets/{dataset_id}/versions"))
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    json!({
                        "version_name": "v1",
                        "created_by": "local-user"
                    })
                    .to_string(),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(version_response.status(), StatusCode::CREATED);
    let version = json_body(version_response).await;
    assert_eq!(version["dataset_id"], dataset_id);
    assert_eq!(version["version_name"], "v1");
    assert_eq!(version["sample_count"], 1);
    assert_eq!(version["annotation_count"], 0);
    assert_eq!(version["classes_snapshot"], json!(["cup", "book"]));
    assert_eq!(version["created_by"], "local-user");
}
