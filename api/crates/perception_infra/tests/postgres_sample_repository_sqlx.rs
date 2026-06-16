use std::{collections::BTreeMap, path::Path};

use perception_app::{DatasetDraft, DatasetRepository, SampleDraft, SampleRepository, TaskType};
use perception_domain::{DatasetId, DatasetStatus, SampleId};
use perception_infra::{PostgresDatasetRepository, PostgresSampleRepository};

fn dataset_fixture(name: &str) -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: name.to_owned(),
        description: Some("Dataset for sample repository".to_owned()),
        task_type: TaskType::ObjectDetection,
        classes: vec!["cup".to_owned(), "book".to_owned()],
        status: DatasetStatus::Draft,
    }
}

fn sample_fixture(dataset_id: DatasetId, filename: &str) -> SampleDraft {
    SampleDraft {
        id: SampleId::new(),
        dataset_id,
        storage_uri: format!("file:///tmp/{filename}"),
        filename: filename.to_owned(),
        mime_type: "image/png".to_owned(),
        width: 640,
        height: 480,
        size_bytes: 14,
        checksum: format!("sha256:{filename}"),
        source: "upload".to_owned(),
        metadata: BTreeMap::from([("camera".to_owned(), "phone".to_owned())]),
    }
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_sample_repository_creates_gets_and_lists_samples_by_dataset() {
    let pool = migrated_pool().await;
    let dataset_repository = PostgresDatasetRepository::new(pool.clone());
    let sample_repository = PostgresSampleRepository::new(pool);
    let dataset = dataset_repository
        .create(dataset_fixture(&format!(
            "sample-dataset-{}",
            DatasetId::new()
        )))
        .await
        .expect("dataset is created");
    let sample = sample_repository
        .create(sample_fixture(dataset.id, "cup.png"))
        .await
        .expect("sample is created");

    assert_eq!(
        sample_repository
            .get(sample.id)
            .await
            .expect("sample lookup works"),
        Some(sample.clone())
    );
    assert_eq!(
        sample_repository
            .list_by_dataset(dataset.id)
            .await
            .expect("dataset samples are listed"),
        vec![sample]
    );
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
