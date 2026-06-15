use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use perception_app::{
    DatasetVersionDraft, DatasetVersionRepository, TrainingJobDraft, TrainingJobQueue,
    TrainingJobQueueEntry, TrainingJobQueueStatus, TrainingJobRepository, TrainingMetricDraft,
    TrainingMetricRepository, UseCaseError,
};
use perception_domain::{
    DatasetId, DatasetVersionId, TrainingHyperparameters, TrainingJobId, TrainingJobStatus,
    TrainingMetricId,
};
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

#[derive(Default)]
struct RouteTrainingMetricRepository {
    metrics: Mutex<Vec<TrainingMetricDraft>>,
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

#[async_trait]
impl TrainingMetricRepository for RouteTrainingMetricRepository {
    async fn create(
        &self,
        metric: TrainingMetricDraft,
    ) -> Result<TrainingMetricDraft, UseCaseError> {
        self.metrics
            .lock()
            .expect("repository mutex is available")
            .push(metric.clone());
        Ok(metric)
    }

    async fn list_by_training_job(
        &self,
        training_job_id: TrainingJobId,
    ) -> Result<Vec<TrainingMetricDraft>, UseCaseError> {
        Ok(self
            .metrics
            .lock()
            .expect("repository mutex is available")
            .iter()
            .filter(|metric| metric.training_job_id == training_job_id)
            .cloned()
            .collect())
    }
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), 4096)
        .await
        .expect("body is readable");
    serde_json::from_slice(&body).expect("body is JSON")
}

fn training_job_fixture() -> TrainingJobDraft {
    TrainingJobDraft {
        id: TrainingJobId::new(),
        dataset_version_id: DatasetVersionId::new(),
        model_family: "tiny_torch".to_owned(),
        base_model: None,
        status: TrainingJobStatus::Running,
        hyperparameters: TrainingHyperparameters::new(2, 1, 64, 0.01)
            .expect("hyperparameters are valid"),
        error_message: None,
    }
}

fn metric_fixture(
    training_job_id: TrainingJobId,
    epoch: u32,
    metric_value: f64,
) -> TrainingMetricDraft {
    TrainingMetricDraft {
        id: TrainingMetricId::new(),
        training_job_id,
        split_name: "train".to_owned(),
        metric_name: "loss".to_owned(),
        metric_value,
        step: Some(epoch),
        epoch: Some(epoch),
        metadata: BTreeMap::new(),
    }
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
        Arc::new(RouteTrainingMetricRepository::default()),
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
        Arc::new(RouteTrainingMetricRepository::default()),
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

#[tokio::test]
async fn list_training_job_metrics_route_returns_metrics_ordered_by_epoch() {
    let versions = Arc::new(RouteDatasetVersionRepository::default());
    let jobs = Arc::new(RouteTrainingJobRepository::default());
    let metrics = Arc::new(RouteTrainingMetricRepository::default());
    let job = jobs
        .create(training_job_fixture())
        .await
        .expect("job is created");
    metrics
        .create(metric_fixture(job.id, 2, 0.32))
        .await
        .expect("epoch 2 metric is created");
    metrics
        .create(metric_fixture(job.id, 1, 0.51))
        .await
        .expect("epoch 1 metric is created");
    let app = perception_http::router_with_training_job_ports(
        versions,
        jobs,
        Arc::new(RouteTrainingJobQueue::default()),
        metrics,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/training-jobs/{}/metrics", job.id))
                .body(axum::body::Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("route responds");

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(
        body["metrics"]
            .as_array()
            .expect("metrics are an array")
            .len(),
        2
    );
    assert_eq!(body["metrics"][0]["epoch"], 1);
    assert_eq!(body["metrics"][0]["metric_value"], 0.51);
    assert_eq!(body["metrics"][1]["epoch"], 2);
    assert_eq!(body["metrics"][1]["metric_value"], 0.32);
}
