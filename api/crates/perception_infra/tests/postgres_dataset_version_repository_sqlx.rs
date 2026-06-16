use std::{collections::BTreeMap, path::Path};

use perception_app::{
    DatasetDraft, DatasetRepository, DatasetVersionDraft, DatasetVersionRepository, TaskType,
};
use perception_domain::{DatasetId, DatasetStatus, DatasetVersionId};
use perception_infra::{PostgresDatasetRepository, PostgresDatasetVersionRepository};

fn dataset_fixture(name: &str) -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: name.to_owned(),
        description: Some("Dataset for version repository".to_owned()),
        task_type: TaskType::ObjectDetection,
        classes: vec!["cup".to_owned(), "book".to_owned()],
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
        classes_snapshot: vec!["cup".to_owned(), "book".to_owned()],
        split_config: BTreeMap::from([
            ("train".to_owned(), "80".to_owned()),
            ("validation".to_owned(), "10".to_owned()),
            ("test".to_owned(), "10".to_owned()),
        ]),
        created_by: "local-user".to_owned(),
    }
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_dataset_version_repository_creates_and_gets_versions() {
    let pool = migrated_pool().await;
    let dataset_repository = PostgresDatasetRepository::new(pool.clone());
    let version_repository = PostgresDatasetVersionRepository::new(pool);
    let dataset = dataset_repository
        .create(dataset_fixture(&format!(
            "version-dataset-{}",
            DatasetId::new()
        )))
        .await
        .expect("dataset is created");
    let version = version_repository
        .create(version_fixture(dataset.id))
        .await
        .expect("version is created");

    assert_eq!(
        version_repository
            .get(version.id)
            .await
            .expect("version lookup works"),
        Some(version)
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
