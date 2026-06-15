use perception_app::{TrainingJobDraft, TrainingJobRepository};
use perception_domain::{
    DatasetVersionId, TrainingHyperparameters, TrainingJobId, TrainingJobStatus,
};
use perception_infra::TransientTrainingJobRepository;

#[tokio::test]
async fn transient_training_job_repository_creates_and_gets_jobs() {
    let repository = TransientTrainingJobRepository::default();
    let job_id = TrainingJobId::new();

    repository
        .create(TrainingJobDraft {
            id: job_id,
            dataset_version_id: DatasetVersionId::new(),
            model_family: "yolo".to_owned(),
            base_model: Some("yolo11n".to_owned()),
            status: TrainingJobStatus::Queued,
            hyperparameters: TrainingHyperparameters::new(5, 2, 640, 0.001)
                .expect("hyperparameters are valid"),
            error_message: None,
        })
        .await
        .expect("job is persisted");

    assert_eq!(
        repository
            .get(job_id)
            .await
            .expect("job lookup succeeds")
            .expect("job exists")
            .status,
        TrainingJobStatus::Queued
    );
}
