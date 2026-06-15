use perception_app::TrainingMetricDraft;

use crate::dto::training_metric::TrainingMetricResponse;

pub fn training_metric_response(metric: TrainingMetricDraft) -> TrainingMetricResponse {
    TrainingMetricResponse {
        id: metric.id.to_string(),
        training_job_id: metric.training_job_id.to_string(),
        split_name: metric.split_name,
        metric_name: metric.metric_name,
        metric_value: metric.metric_value,
        step: metric.step,
        epoch: metric.epoch,
        metadata: metric.metadata,
    }
}
