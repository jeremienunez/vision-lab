use std::{collections::BTreeMap, sync::Mutex};

use async_trait::async_trait;
use perception_app::{
    ListTrainingClassMetricsUseCase, ListTrainingMetricsUseCase, RecordTrainingMetricCommand,
    RecordTrainingMetricUseCase, TrainingJobDraft, TrainingJobRepository, TrainingMetricDraft,
    TrainingMetricRepository, UseCaseError,
};
use perception_domain::{
    DatasetVersionId, TrainingHyperparameters, TrainingJobId, TrainingJobStatus,
};

#[derive(Default)]
struct InMemoryTrainingJobRepository {
    jobs: Mutex<Vec<TrainingJobDraft>>,
}

#[async_trait]
impl TrainingJobRepository for InMemoryTrainingJobRepository {
    async fn create(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        self.jobs
            .lock()
            .expect("repository mutex is available")
            .push(job.clone());
        Ok(job)
    }

    async fn list(&self) -> Result<Vec<TrainingJobDraft>, UseCaseError> {
        Ok(self
            .jobs
            .lock()
            .expect("repository mutex is available")
            .clone())
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

#[derive(Default)]
struct InMemoryTrainingMetricRepository {
    metrics: Mutex<Vec<TrainingMetricDraft>>,
}

#[async_trait]
impl TrainingMetricRepository for InMemoryTrainingMetricRepository {
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
        let mut metrics = self
            .metrics
            .lock()
            .expect("repository mutex is available")
            .iter()
            .filter(|metric| metric.training_job_id == training_job_id)
            .cloned()
            .collect::<Vec<_>>();
        metrics.sort_by_key(|metric| (metric.epoch, metric.step, metric.metric_name.clone()));
        Ok(metrics)
    }
}

fn running_job_fixture() -> TrainingJobDraft {
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

#[tokio::test]
async fn record_training_metric_persists_metrics_ordered_by_epoch() {
    let jobs = InMemoryTrainingJobRepository::default();
    let metrics = InMemoryTrainingMetricRepository::default();
    let job = jobs
        .create(running_job_fixture())
        .await
        .expect("job is created");

    RecordTrainingMetricUseCase::new(&jobs, &metrics)
        .execute(RecordTrainingMetricCommand {
            training_job_id: job.id,
            split_name: "train".to_owned(),
            metric_name: "loss".to_owned(),
            metric_value: 0.32,
            step: Some(2),
            epoch: Some(2),
            metadata: BTreeMap::new(),
        })
        .await
        .expect("epoch 2 metric is recorded");
    RecordTrainingMetricUseCase::new(&jobs, &metrics)
        .execute(RecordTrainingMetricCommand {
            training_job_id: job.id,
            split_name: "train".to_owned(),
            metric_name: "loss".to_owned(),
            metric_value: 0.51,
            step: Some(1),
            epoch: Some(1),
            metadata: BTreeMap::new(),
        })
        .await
        .expect("epoch 1 metric is recorded");

    let stored = ListTrainingMetricsUseCase::new(&jobs, &metrics)
        .execute(job.id)
        .await
        .expect("metrics are listed");

    assert_eq!(
        stored.iter().map(|metric| metric.epoch).collect::<Vec<_>>(),
        vec![Some(1), Some(2)]
    );
    assert_eq!(stored[0].metric_value, 0.51);
    assert_eq!(stored[1].metric_value, 0.32);
}

#[tokio::test]
async fn record_training_metric_rejects_missing_training_job() {
    let jobs = InMemoryTrainingJobRepository::default();
    let metrics = InMemoryTrainingMetricRepository::default();

    let result = RecordTrainingMetricUseCase::new(&jobs, &metrics)
        .execute(RecordTrainingMetricCommand {
            training_job_id: TrainingJobId::new(),
            split_name: "train".to_owned(),
            metric_name: "loss".to_owned(),
            metric_value: 0.32,
            step: Some(1),
            epoch: Some(1),
            metadata: BTreeMap::new(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::NotFound("training job not found"))
    );
}

#[tokio::test]
async fn record_training_metric_rejects_invalid_metric_contract() {
    let jobs = InMemoryTrainingJobRepository::default();
    let metrics = InMemoryTrainingMetricRepository::default();
    let job = jobs
        .create(running_job_fixture())
        .await
        .expect("job is created");

    let result = RecordTrainingMetricUseCase::new(&jobs, &metrics)
        .execute(RecordTrainingMetricCommand {
            training_job_id: job.id,
            split_name: "debug".to_owned(),
            metric_name: " ".to_owned(),
            metric_value: f64::NAN,
            step: Some(1),
            epoch: Some(1),
            metadata: BTreeMap::new(),
        })
        .await;

    assert_eq!(
        result,
        Err(UseCaseError::Validation("invalid training metric"))
    );
}

#[tokio::test]
async fn list_training_class_metrics_returns_only_metrics_tagged_with_class_name() {
    let jobs = InMemoryTrainingJobRepository::default();
    let metrics = InMemoryTrainingMetricRepository::default();
    let job = jobs
        .create(running_job_fixture())
        .await
        .expect("job is created");

    RecordTrainingMetricUseCase::new(&jobs, &metrics)
        .execute(RecordTrainingMetricCommand {
            training_job_id: job.id,
            split_name: "validation".to_owned(),
            metric_name: "mAP50".to_owned(),
            metric_value: 0.82,
            step: None,
            epoch: Some(1),
            metadata: BTreeMap::from([("class_name".to_owned(), "cup".to_owned())]),
        })
        .await
        .expect("cup class metric is recorded");
    RecordTrainingMetricUseCase::new(&jobs, &metrics)
        .execute(RecordTrainingMetricCommand {
            training_job_id: job.id,
            split_name: "validation".to_owned(),
            metric_name: "mAP50".to_owned(),
            metric_value: 0.74,
            step: None,
            epoch: Some(1),
            metadata: BTreeMap::from([("class_name".to_owned(), "book".to_owned())]),
        })
        .await
        .expect("book class metric is recorded");
    RecordTrainingMetricUseCase::new(&jobs, &metrics)
        .execute(RecordTrainingMetricCommand {
            training_job_id: job.id,
            split_name: "validation".to_owned(),
            metric_name: "mAP50".to_owned(),
            metric_value: 0.79,
            step: None,
            epoch: Some(1),
            metadata: BTreeMap::new(),
        })
        .await
        .expect("aggregate metric is recorded");

    let class_metrics = ListTrainingClassMetricsUseCase::new(&jobs, &metrics)
        .execute(job.id)
        .await
        .expect("class metrics are listed");

    assert_eq!(class_metrics.len(), 2);
    assert_eq!(class_metrics[0].class_name, "book");
    assert_eq!(class_metrics[0].metric_value, 0.74);
    assert_eq!(class_metrics[1].class_name, "cup");
    assert_eq!(class_metrics[1].metric_value, 0.82);
}
