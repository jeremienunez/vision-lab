use std::{collections::BTreeMap, path::Path};

use perception_app::{
    AnnotationDraft, AnnotationRepository, DatasetDraft, DatasetRepository, SampleDraft,
    SampleRepository, TaskType,
};
use perception_domain::{AnnotationId, DatasetId, DatasetStatus, SampleId};
use perception_infra::{
    PostgresAnnotationRepository, PostgresDatasetRepository, PostgresSampleRepository,
};

fn dataset_fixture(name: &str) -> DatasetDraft {
    DatasetDraft {
        id: DatasetId::new(),
        name: name.to_owned(),
        description: Some("Dataset for annotation repository".to_owned()),
        task_type: TaskType::ObjectDetection,
        classes: vec!["cup".to_owned(), "book".to_owned()],
        status: DatasetStatus::Draft,
    }
}

fn sample_fixture(dataset_id: DatasetId) -> SampleDraft {
    SampleDraft {
        id: SampleId::new(),
        dataset_id,
        storage_uri: "file:///tmp/cup.png".to_owned(),
        filename: "cup.png".to_owned(),
        mime_type: "image/png".to_owned(),
        width: 640,
        height: 480,
        size_bytes: 14,
        checksum: format!("sha256:{}", SampleId::new()),
        source: "upload".to_owned(),
        metadata: BTreeMap::new(),
    }
}

fn annotation_fixture(sample: &SampleDraft) -> AnnotationDraft {
    AnnotationDraft {
        id: AnnotationId::new(),
        sample_id: sample.id,
        dataset_id: sample.dataset_id,
        class_name: "cup".to_owned(),
        class_id: 0,
        bbox_x: 0.10,
        bbox_y: 0.20,
        bbox_width: 0.30,
        bbox_height: 0.40,
        format: "normalized_xywh".to_owned(),
        confidence: Some(0.91),
        source: "manual".to_owned(),
    }
}

#[tokio::test]
#[ignore = "requires local PostgreSQL; run with PERCEPTIONLAB_DATABASE_URL and --ignored"]
async fn postgres_annotation_repository_creates_and_lists_annotations() {
    let pool = migrated_pool().await;
    let dataset_repository = PostgresDatasetRepository::new(pool.clone());
    let sample_repository = PostgresSampleRepository::new(pool.clone());
    let annotation_repository = PostgresAnnotationRepository::new(pool);
    let dataset = dataset_repository
        .create(dataset_fixture(&format!(
            "annotation-dataset-{}",
            DatasetId::new()
        )))
        .await
        .expect("dataset is created");
    let sample = sample_repository
        .create(sample_fixture(dataset.id))
        .await
        .expect("sample is created");
    let annotation = annotation_repository
        .create(annotation_fixture(&sample))
        .await
        .expect("annotation is created");

    assert_eq!(
        annotation_repository
            .list_by_sample(sample.id)
            .await
            .expect("sample annotations are listed"),
        vec![annotation.clone()]
    );
    assert_eq!(
        annotation_repository
            .list_by_dataset(dataset.id)
            .await
            .expect("dataset annotations are listed"),
        vec![annotation]
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
