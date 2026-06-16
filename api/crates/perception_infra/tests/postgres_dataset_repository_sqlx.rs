use std::path::Path;

use perception_app::{DatasetDraft, DatasetRepository, TaskType};
use perception_domain::{DatasetId, DatasetStatus};
use perception_infra::PostgresDatasetRepository;

fn dataset_fixture(name: &str) -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: name.to_owned(),
        description: Some("Dataset persisted in PostgreSQL".to_owned()),
        task_type: TaskType::ObjectDetection,
        classes: vec!["cup".to_owned(), "book".to_owned()],
        status: DatasetStatus::Draft,
    }
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_dataset_repository_creates_gets_and_lists_datasets() {
    let pool = migrated_pool().await;
    let repository = PostgresDatasetRepository::new(pool);
    let dataset_name = format!("desk-objects-{}", DatasetId::new());
    let dataset = repository
        .create(dataset_fixture(&dataset_name))
        .await
        .expect("dataset is created");

    assert_eq!(
        repository.get(dataset.id).await.expect("dataset is read"),
        Some(dataset.clone())
    );
    let datasets = repository.list().await.expect("datasets are listed");
    assert!(datasets.contains(&dataset));
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_dataset_repository_rolls_back_dataset_when_classes_fail() {
    let pool = migrated_pool().await;
    let repository = PostgresDatasetRepository::new(pool);
    let mut dataset = dataset_fixture(&format!("invalid-classes-{}", DatasetId::new()));
    dataset.classes = vec!["cup".to_owned(), "cup".to_owned()];
    let dataset_id = dataset.id;

    assert!(repository.create(dataset).await.is_err());
    assert_eq!(
        repository
            .get(dataset_id)
            .await
            .expect("dataset lookup works"),
        None
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
