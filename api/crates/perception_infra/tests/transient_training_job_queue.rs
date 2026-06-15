use perception_app::{TrainingJobQueue, TrainingJobQueueEntry, TrainingJobQueueStatus};
use perception_domain::TrainingJobId;
use perception_infra::TransientTrainingJobQueue;

#[tokio::test]
async fn transient_training_job_queue_enqueues_and_leases_next_job() {
    let queue = TransientTrainingJobQueue::default();
    let training_job_id = TrainingJobId::new();

    queue
        .enqueue(TrainingJobQueueEntry::queued(training_job_id))
        .await
        .expect("job is enqueued");

    let leased = queue
        .lease_next("worker-1".to_owned())
        .await
        .expect("lease succeeds")
        .expect("queued job exists");

    assert_eq!(leased.training_job_id, training_job_id);
    assert_eq!(leased.status, TrainingJobQueueStatus::Leased);
    assert_eq!(leased.locked_by, Some("worker-1".to_owned()));
    assert_eq!(leased.attempts, 1);

    let empty = queue
        .lease_next("worker-2".to_owned())
        .await
        .expect("lease succeeds");

    assert_eq!(empty, None);
}
