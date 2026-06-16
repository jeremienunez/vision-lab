use std::{collections::BTreeMap, path::Path};

use perception_app::{
    DatasetDraft, DatasetRepository, DatasetVersionDraft, DatasetVersionRepository, TaskType,
    TrainingJobDraft, TrainingJobQueue, TrainingJobQueueEntry, TrainingJobQueueStatus,
    TrainingJobRepository,
};
use perception_domain::{
    DatasetId, DatasetStatus, DatasetVersionId, TrainingHyperparameters, TrainingJobId,
    TrainingJobStatus,
};
use perception_infra::{
    PostgresDatasetRepository, PostgresDatasetVersionRepository, PostgresTrainingJobQueue,
    PostgresTrainingJobRepository,
};

fn dataset_fixture(name: &str) -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: name.to_owned(),
        description: Some("Dataset for training job repository".to_owned()),
        task_type: TaskType::ObjectDetection,
        classes: vec!["Mask".to_owned(), "Gloves".to_owned()],
        status: DatasetStatus::Draft,
    }
}

fn version_fixture(dataset_id: DatasetId) -> DatasetVersionDraft {
    DatasetVersionDraft {
        id: DatasetVersionId::new(),
        dataset_id,
        version_name: "v1".to_owned(),
        sample_count: 2,
        annotation_count: 3,
        classes_snapshot: vec!["Mask".to_owned(), "Gloves".to_owned()],
        split_config: BTreeMap::from([
            ("train".to_owned(), "80".to_owned()),
            ("validation".to_owned(), "10".to_owned()),
            ("test".to_owned(), "10".to_owned()),
        ]),
        created_by: "local-user".to_owned(),
    }
}

fn job_fixture(dataset_version_id: DatasetVersionId) -> TrainingJobDraft {
    TrainingJobDraft {
        id: TrainingJobId::new(),
        dataset_version_id,
        model_family: "tiny_torch".to_owned(),
        base_model: Some("synthetic".to_owned()),
        status: TrainingJobStatus::Queued,
        hyperparameters: TrainingHyperparameters::new(3, 2, 64, 0.01)
            .expect("hyperparameters are valid"),
        error_message: None,
    }
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_training_job_repository_creates_gets_and_updates_jobs() {
    let pool = migrated_pool().await;
    let version = create_dataset_version(&pool).await;
    let repository = PostgresTrainingJobRepository::new(pool);
    let mut job = repository
        .create(job_fixture(version.id))
        .await
        .expect("job is created");

    assert!(
        repository
            .list()
            .await
            .expect("job list works")
            .iter()
            .any(|listed_job| listed_job.id == job.id)
    );

    assert_eq!(
        repository
            .get(job.id)
            .await
            .expect("job lookup works")
            .expect("job exists"),
        job
    );

    job.status = TrainingJobStatus::Running;
    repository
        .update(job.clone())
        .await
        .expect("job is updated");

    assert_eq!(
        repository
            .get(job.id)
            .await
            .expect("job lookup works")
            .expect("job exists")
            .status,
        TrainingJobStatus::Running
    );

    job.status = TrainingJobStatus::Failed;
    job.error_message = Some("training failed".to_owned());
    repository
        .update(job.clone())
        .await
        .expect("failed job is updated");

    assert_eq!(
        repository
            .get(job.id)
            .await
            .expect("job lookup works")
            .expect("job exists"),
        job
    );
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_training_job_queue_enqueues_and_leases_once() {
    let pool = migrated_pool().await;
    let version = create_dataset_version(&pool).await;
    let job_repository = PostgresTrainingJobRepository::new(pool.clone());
    let queue = PostgresTrainingJobQueue::new(pool);
    let job = job_repository
        .create(job_fixture(version.id))
        .await
        .expect("job is created");

    queue
        .enqueue(TrainingJobQueueEntry::queued(job.id))
        .await
        .expect("job is enqueued");

    let mut leased = None;
    for attempt in 1..=20 {
        let Some(entry) = queue
            .lease_next(format!("worker-{attempt}"))
            .await
            .expect("lease succeeds")
        else {
            break;
        };
        if entry.training_job_id == job.id {
            leased = Some(entry);
            break;
        }
    }
    let leased = leased.expect("target queued job is eventually leased");

    assert_eq!(leased.training_job_id, job.id);
    assert_eq!(leased.status, TrainingJobQueueStatus::Leased);
    assert!(leased.locked_by.is_some());
    assert_eq!(leased.attempts, 1);
}

async fn create_dataset_version(pool: &sqlx::PgPool) -> DatasetVersionDraft {
    let dataset_repository = PostgresDatasetRepository::new(pool.clone());
    let version_repository = PostgresDatasetVersionRepository::new(pool.clone());
    let dataset = dataset_repository
        .create(dataset_fixture(&format!(
            "training-job-dataset-{}",
            DatasetId::new()
        )))
        .await
        .expect("dataset is created");

    version_repository
        .create(version_fixture(dataset.id))
        .await
        .expect("version is created")
}

async fn migrated_pool() -> sqlx::PgPool {
    let database_url = std::env::var("PERCEPTIONLAB_DATABASE_URL")
        .expect("PERCEPTIONLAB_DATABASE_URL must point to a test PostgreSQL database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("test PostgreSQL is reachable");
    let migrations_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    let migrator = sqlx::migrate::Migrator::new(migrations_root)
        .await
        .expect("migrations are loadable");
    migrator.run(&pool).await.expect("initial schema applies");
    pool
}
