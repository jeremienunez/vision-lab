use std::{collections::BTreeMap, path::Path};

use perception_app::{
    DatasetDraft, DatasetRepository, DatasetVersionDraft, DatasetVersionRepository, ModelDraft,
    ModelRepository, TaskType, TrainingJobDraft, TrainingJobRepository, TrainingMetricDraft,
    TrainingMetricRepository,
};
use perception_domain::{
    DatasetId, DatasetStatus, DatasetVersionId, ModelId, ModelStatus, TrainingHyperparameters,
    TrainingJobId, TrainingJobStatus, TrainingMetricId,
};
use perception_infra::{
    PostgresDatasetRepository, PostgresDatasetVersionRepository, PostgresModelRepository,
    PostgresTrainingJobRepository, PostgresTrainingMetricRepository,
};

fn dataset_fixture(name: &str) -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: name.to_owned(),
        description: Some("Dataset for metric and model repository".to_owned()),
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
        metadata: BTreeMap::from([("source".to_owned(), "worker".to_owned())]),
    }
}

fn model_fixture(
    training_job_id: TrainingJobId,
    dataset_version_id: DatasetVersionId,
    name: &str,
) -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: name.to_owned(),
        version: "v1".to_owned(),
        training_job_id,
        dataset_version_id,
        model_family: "tiny_torch".to_owned(),
        artifact_uri: "file:///tmp/perceptionlab/model.pt".to_owned(),
        metrics_summary: BTreeMap::from([
            ("mAP50".to_owned(), "0.91".to_owned()),
            ("classes".to_owned(), "Mask,Gloves".to_owned()),
        ]),
        status: ModelStatus::Candidate,
    }
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_training_metric_repository_creates_and_lists_metrics_in_epoch_order() {
    let pool = migrated_pool().await;
    let version = create_dataset_version(&pool).await;
    let job_repository = PostgresTrainingJobRepository::new(pool.clone());
    let metric_repository = PostgresTrainingMetricRepository::new(pool);
    let job = job_repository
        .create(job_fixture(version.id))
        .await
        .expect("job is created");
    let other_job = job_repository
        .create(job_fixture(version.id))
        .await
        .expect("other job is created");

    metric_repository
        .create(metric_fixture(job.id, 2, 0.32))
        .await
        .expect("metric is stored");
    metric_repository
        .create(metric_fixture(job.id, 1, 0.51))
        .await
        .expect("metric is stored");
    metric_repository
        .create(metric_fixture(other_job.id, 1, 0.99))
        .await
        .expect("other job metric is stored");

    let metrics = metric_repository
        .list_by_training_job(job.id)
        .await
        .expect("metrics are listed");

    assert_eq!(metrics.len(), 2);
    assert_eq!(metrics[0].epoch, Some(1));
    assert_eq!(metrics[0].metric_value, 0.51);
    assert_eq!(
        metrics[0].metadata,
        BTreeMap::from([("source".to_owned(), "worker".to_owned())])
    );
    assert_eq!(metrics[1].epoch, Some(2));
    assert_eq!(metrics[1].metric_value, 0.32);
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_model_repository_creates_lists_gets_and_updates_models() {
    let pool = migrated_pool().await;
    let version = create_dataset_version(&pool).await;
    let job_repository = PostgresTrainingJobRepository::new(pool.clone());
    let model_repository = PostgresModelRepository::new(pool);
    let job = job_repository
        .create(job_fixture(version.id))
        .await
        .expect("job is created");
    let mut model = model_repository
        .create(model_fixture(
            job.id,
            version.id,
            &format!("mvp-model-{}", ModelId::new()),
        ))
        .await
        .expect("model is created");

    assert!(
        model_repository
            .list()
            .await
            .expect("models are listed")
            .contains(&model)
    );
    assert_eq!(
        model_repository
            .get(model.id)
            .await
            .expect("model lookup succeeds")
            .expect("model exists"),
        model
    );

    model.status = ModelStatus::Validated;
    model
        .metrics_summary
        .insert("validation_loss".to_owned(), "0.21".to_owned());
    let updated = model_repository
        .update(model.clone())
        .await
        .expect("model is updated");

    assert_eq!(updated, model);
    assert_eq!(
        model_repository
            .get(model.id)
            .await
            .expect("model lookup succeeds")
            .expect("model exists"),
        model
    );
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_model_repository_normalizes_numeric_metrics_summary_values() {
    let pool = migrated_pool().await;
    let version = create_dataset_version(&pool).await;
    let job_repository = PostgresTrainingJobRepository::new(pool.clone());
    let model_repository = PostgresModelRepository::new(pool.clone());
    let job = job_repository
        .create(job_fixture(version.id))
        .await
        .expect("job is created");
    let model_id = ModelId::new();

    sqlx::query(
        r#"
        INSERT INTO models (
            id, name, version, training_job_id, dataset_version_id,
            model_family, artifact_uri, metrics_summary, status
        )
        VALUES ($1, $2, $3, $4, $5, 'tiny_torch', 'file:///tmp/model.pt', $6, 'candidate')
        "#,
    )
    .bind(model_id.into_uuid())
    .bind(format!("numeric-summary-{}", ModelId::new()))
    .bind("v1")
    .bind(job.id.into_uuid())
    .bind(version.id.into_uuid())
    .bind(sqlx::types::Json(serde_json::json!({
        "accuracy": 1.0,
        "validation_loss": 0.25,
        "epochs": 3
    })))
    .execute(&pool)
    .await
    .expect("model row is inserted");

    let model = model_repository
        .get(model_id)
        .await
        .expect("model lookup succeeds")
        .expect("model exists");

    assert_eq!(
        model.metrics_summary.get("accuracy"),
        Some(&"1.0".to_owned())
    );
    assert_eq!(
        model.metrics_summary.get("validation_loss"),
        Some(&"0.25".to_owned())
    );
    assert_eq!(model.metrics_summary.get("epochs"), Some(&"3".to_owned()));
}

async fn create_dataset_version(pool: &sqlx::PgPool) -> DatasetVersionDraft {
    let dataset_repository = PostgresDatasetRepository::new(pool.clone());
    let version_repository = PostgresDatasetVersionRepository::new(pool.clone());
    let dataset = dataset_repository
        .create(dataset_fixture(&format!(
            "metric-model-dataset-{}",
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
