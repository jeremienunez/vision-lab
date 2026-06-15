use std::collections::BTreeMap;

use perception_app::{TrainingMetricDraft, TrainingMetricRepository};
use perception_domain::{TrainingJobId, TrainingMetricId};
use perception_infra::TransientTrainingMetricRepository;

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
async fn transient_training_metric_repository_lists_metrics_by_job_in_epoch_order() {
    let repository = TransientTrainingMetricRepository::default();
    let training_job_id = TrainingJobId::new();
    repository
        .create(metric_fixture(training_job_id, 2, 0.32))
        .await
        .expect("metric is stored");
    repository
        .create(metric_fixture(training_job_id, 1, 0.51))
        .await
        .expect("metric is stored");
    repository
        .create(metric_fixture(TrainingJobId::new(), 1, 0.99))
        .await
        .expect("other job metric is stored");

    let metrics = repository
        .list_by_training_job(training_job_id)
        .await
        .expect("metrics are listed");

    assert_eq!(metrics.len(), 2);
    assert_eq!(metrics[0].epoch, Some(1));
    assert_eq!(metrics[0].metric_value, 0.51);
    assert_eq!(metrics[1].epoch, Some(2));
    assert_eq!(metrics[1].metric_value, 0.32);
}
