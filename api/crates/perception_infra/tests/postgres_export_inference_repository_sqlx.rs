use std::{collections::BTreeMap, path::Path};

use perception_app::{
    DatasetDraft, DatasetRepository, DatasetVersionDraft, DatasetVersionRepository, DetectionDraft,
    InferenceRunDraft, InferenceRunRepository, ModelDraft, ModelExportDraft, ModelExportRepository,
    ModelRepository, TaskType, TrainingJobDraft, TrainingJobRepository,
};
use perception_domain::{
    DatasetId, DatasetStatus, DatasetVersionId, ExportStatus, InferenceRunId, ModelExportId,
    ModelId, ModelStatus, NormalizedBbox, TrainingHyperparameters, TrainingJobId,
    TrainingJobStatus,
};
use perception_infra::{
    PostgresDatasetRepository, PostgresDatasetVersionRepository, PostgresInferenceRunRepository,
    PostgresModelExportRepository, PostgresModelRepository, PostgresTrainingJobRepository,
};

fn dataset_fixture(name: &str) -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: name.to_owned(),
        description: Some("Dataset for export and inference repository".to_owned()),
        task_type: TaskType::ObjectDetection,
        classes: vec!["Cup".to_owned(), "Bottle".to_owned()],
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
        classes_snapshot: vec!["Cup".to_owned(), "Bottle".to_owned()],
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
        status: TrainingJobStatus::Succeeded,
        hyperparameters: TrainingHyperparameters::new(3, 2, 64, 0.01)
            .expect("hyperparameters are valid"),
        error_message: None,
    }
}

fn model_fixture(
    training_job_id: TrainingJobId,
    dataset_version_id: DatasetVersionId,
) -> ModelDraft {
    ModelDraft {
        id: ModelId::new(),
        name: format!("export-inference-model-{}", ModelId::new()),
        version: "v1".to_owned(),
        training_job_id,
        dataset_version_id,
        model_family: "tiny_torch".to_owned(),
        artifact_uri: "file:///tmp/perceptionlab/model.pt".to_owned(),
        metrics_summary: BTreeMap::from([("train_loss".to_owned(), "0.25".to_owned())]),
        status: ModelStatus::Candidate,
    }
}

fn export_fixture(model_id: ModelId, format: &str) -> ModelExportDraft {
    ModelExportDraft {
        id: ModelExportId::new(),
        model_id,
        format: format.to_owned(),
        artifact_uri: Some(format!("file:///tmp/model.{format}")),
        status: ExportStatus::Succeeded,
        error_message: None,
    }
}

fn run_fixture(model_id: ModelId) -> InferenceRunDraft {
    InferenceRunDraft {
        id: InferenceRunId::new(),
        model_id,
        filename: "cup.jpg".to_owned(),
        mime_type: "image/jpeg".to_owned(),
        latency_ms: 12,
        detections: vec![DetectionDraft {
            class_id: 0,
            class_name: "Cup".to_owned(),
            confidence: 0.91,
            bbox: NormalizedBbox::new(0.1, 0.2, 0.3, 0.4).expect("bbox is valid"),
            distance_m: Some(1.4),
        }],
    }
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_model_export_repository_creates_and_lists_exports_by_model() {
    let pool = migrated_pool().await;
    let model = create_model(&pool).await;
    let other_model = create_model(&pool).await;
    let repository = PostgresModelExportRepository::new(pool);
    let export = repository
        .create(export_fixture(model.id, "onnx"))
        .await
        .expect("export is stored");
    repository
        .create(export_fixture(other_model.id, "coreml"))
        .await
        .expect("other export is stored");

    let listed = repository
        .list_by_model(model.id)
        .await
        .expect("exports are listed by model");

    assert_eq!(listed, vec![export]);
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_inference_run_repository_creates_and_gets_runs_with_detections() {
    let pool = migrated_pool().await;
    let model = create_model(&pool).await;
    let repository = PostgresInferenceRunRepository::new(pool);
    let run = repository
        .create(run_fixture(model.id))
        .await
        .expect("run is stored");

    let fetched = repository
        .get(run.id)
        .await
        .expect("run lookup succeeds")
        .expect("run exists");

    assert_eq!(fetched, run);
}

async fn create_model(pool: &sqlx::PgPool) -> ModelDraft {
    let dataset_repository = PostgresDatasetRepository::new(pool.clone());
    let version_repository = PostgresDatasetVersionRepository::new(pool.clone());
    let job_repository = PostgresTrainingJobRepository::new(pool.clone());
    let model_repository = PostgresModelRepository::new(pool.clone());
    let dataset = dataset_repository
        .create(dataset_fixture(&format!(
            "export-inference-dataset-{}",
            DatasetId::new()
        )))
        .await
        .expect("dataset is created");
    let version = version_repository
        .create(version_fixture(dataset.id))
        .await
        .expect("version is created");
    let job = job_repository
        .create(job_fixture(version.id))
        .await
        .expect("job is created");

    model_repository
        .create(model_fixture(job.id, version.id))
        .await
        .expect("model is created")
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
