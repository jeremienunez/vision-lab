use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{
    DatasetVersionDraft, DatasetVersionRepository, TrainingJobDraft, TrainingJobQueue,
    TrainingJobQueueEntry, TrainingJobQueueStatus, TrainingJobRepository, UseCaseError,
};
use perception_domain::{DatasetId, DatasetVersionId, TrainingJobId};
use serde_json::{Value, json};
use tower::ServiceExt;

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

#[derive(Default)]
struct RouteTrainingJobRepository {
    jobs: Mutex<Vec<TrainingJobDraft>>,
}

#[derive(Default)]
struct RouteTrainingJobQueue {
    entries: Mutex<Vec<TrainingJobQueueEntry>>,
}

#[async_trait]
impl TrainingJobRepository for RouteTrainingJobRepository {
    async fn create(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        self.jobs
            .lock()
            .expect("repository mutex is available")
            .push(job.clone());
        Ok(job)
    }

    async fn get(&self, job_id: TrainingJobId) -> Result<Option<TrainingJobDraft>, UseCaseError> {
        Ok(self
            .jobs
            .lock()
            .expect("repository mutex is available")
            .iter()
            .find(|job| job.id == job_id)
            .cloned())
    }

    async fn update(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        let mut jobs = self.jobs.lock().expect("repository mutex is available");
        let stored = jobs
            .iter_mut()
            .find(|stored_job| stored_job.id == job.id)
            .ok_or(UseCaseError::NotFound("training job not found"))?;
        *stored = job.clone();
        Ok(job)
    }
}

#[async_trait]
impl TrainingJobQueue for RouteTrainingJobQueue {
    async fn enqueue(
        &self,
        entry: TrainingJobQueueEntry,
    ) -> Result<TrainingJobQueueEntry, UseCaseError> {
        self.entries
            .lock()
            .expect("queue mutex is available")
            .push(entry.clone());
        Ok(entry)
    }

    async fn lease_next(
        &self,
        worker_id: String,
    ) -> Result<Option<TrainingJobQueueEntry>, UseCaseError> {
        let mut entries = self.entries.lock().expect("queue mutex is available");
        let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.status == TrainingJobQueueStatus::Queued)
        else {
            return Ok(None);
        };

        entry.status = TrainingJobQueueStatus::Leased;
        entry.locked_by = Some(worker_id);
        entry.attempts += 1;
        Ok(Some(entry.clone()))
    }
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), 4096)
        .await
        .expect("body is readable");
    serde_json::from_slice(&body).expect("body is JSON")
}

#[tokio::test]
async fn create_training_job_route_returns_queued_job_for_existing_dataset_version() {
    let versions = Arc::new(RouteDatasetVersionRepository::default());
    let version = versions
        .create(DatasetVersionDraft {
            id: DatasetVersionId::new(),
            dataset_id: DatasetId::new(),
            version_name: "v1".to_owned(),
            sample_count: 1,
            annotation_count: 1,
            classes_snapshot: vec!["cup".to_owned()],
            split_config: BTreeMap::new(),
            created_by: "local-user".to_owned(),
        })
        .await
        .expect("version is created");
    let app = perception_http::router_with_training_job_ports(
        versions,
        Arc::new(RouteTrainingJobRepository::default()),
        Arc::new(RouteTrainingJobQueue::default()),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/training-jobs")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    json!({
                        "dataset_version_id": version.id.to_string(),
                        "model_family": "yolo",
                        "base_model": "yolo11n",
                        "hyperparameters": {
                            "epochs": 5,
                            "batch_size": 2,
                            "image_size": 640,
                            "learning_rate": 0.001
                        }
                    })
                    .to_string(),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::CREATED);
    let job = json_body(response).await;
    assert_eq!(job["dataset_version_id"], version.id.to_string());
    assert_eq!(job["model_family"], "yolo");
    assert_eq!(job["base_model"], "yolo11n");
    assert_eq!(job["status"], "queued");
}

#[tokio::test]
async fn create_training_job_route_enqueues_created_job() {
    let versions = Arc::new(RouteDatasetVersionRepository::default());
    let queue = Arc::new(RouteTrainingJobQueue::default());
    let version = versions
        .create(DatasetVersionDraft {
            id: DatasetVersionId::new(),
            dataset_id: DatasetId::new(),
            version_name: "v1".to_owned(),
            sample_count: 1,
            annotation_count: 1,
            classes_snapshot: vec!["cup".to_owned()],
            split_config: BTreeMap::new(),
            created_by: "local-user".to_owned(),
        })
        .await
        .expect("version is created");
    let app = perception_http::router_with_training_job_ports(
        versions,
        Arc::new(RouteTrainingJobRepository::default()),
        queue.clone(),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/training-jobs")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    json!({
                        "dataset_version_id": version.id.to_string(),
                        "model_family": "yolo",
                        "base_model": "yolo11n",
                        "hyperparameters": {
                            "epochs": 5,
                            "batch_size": 2,
                            "image_size": 640,
                            "learning_rate": 0.001
                        }
                    })
                    .to_string(),
                ))
                .expect("request is valid"),
        )
        .await
        .expect("route responds");
    let job = json_body(response).await;
    let leased = queue
        .lease_next("worker-1".to_owned())
        .await
        .expect("lease succeeds")
        .expect("created job is enqueued");

    assert_eq!(leased.training_job_id.to_string(), job["id"]);
    assert_eq!(leased.status, TrainingJobQueueStatus::Leased);
}
