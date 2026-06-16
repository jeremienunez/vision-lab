use async_trait::async_trait;
use perception_app::{DatasetDraft, DatasetRepository, TaskType, UseCaseError};
use perception_domain::{DatasetId, DatasetStatus};
use sqlx::{PgPool, Row, postgres::PgRow, types::Json};

pub struct PostgresDatasetRepository {
    pool: PgPool,
}

impl PostgresDatasetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatasetRepository for PostgresDatasetRepository {
    async fn create(&self, dataset: DatasetDraft) -> Result<DatasetDraft, UseCaseError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| UseCaseError::Repository("postgres dataset transaction failed"))?;

        sqlx::query(
            r#"
            INSERT INTO datasets (id, name, description, task_type, classes, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(dataset.id.into_uuid())
        .bind(&dataset.name)
        .bind(&dataset.description)
        .bind(task_type_to_str(&dataset.task_type))
        .bind(Json(&dataset.classes))
        .bind(dataset_status_to_str(dataset.status))
        .execute(&mut *transaction)
        .await
        .map_err(|_| UseCaseError::Repository("postgres dataset create failed"))?;

        for (class_id, class_name) in dataset.classes.iter().enumerate() {
            sqlx::query(
                r#"
                INSERT INTO dataset_classes (dataset_id, class_id, class_name)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(dataset.id.into_uuid())
            .bind(
                i32::try_from(class_id)
                    .map_err(|_| UseCaseError::Repository("postgres dataset class id overflow"))?,
            )
            .bind(class_name)
            .execute(&mut *transaction)
            .await
            .map_err(|_| UseCaseError::Repository("postgres dataset class create failed"))?;
        }

        transaction
            .commit()
            .await
            .map_err(|_| UseCaseError::Repository("postgres dataset transaction failed"))?;

        Ok(dataset)
    }

    async fn get(&self, dataset_id: DatasetId) -> Result<Option<DatasetDraft>, UseCaseError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, task_type, classes, status
            FROM datasets
            WHERE id = $1
            "#,
        )
        .bind(dataset_id.into_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres dataset get failed"))?;

        row.map(row_to_dataset).transpose()
    }

    async fn list(&self) -> Result<Vec<DatasetDraft>, UseCaseError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, task_type, classes, status
            FROM datasets
            ORDER BY created_at ASC, id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres dataset list failed"))?;

        rows.into_iter().map(row_to_dataset).collect()
    }
}

fn row_to_dataset(row: PgRow) -> Result<DatasetDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let task_type: String = row.get("task_type");
    let status: String = row.get("status");
    let classes: Json<Vec<String>> = row.get("classes");

    Ok(DatasetDraft {
        id: DatasetId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres dataset id invalid"))?,
        name: row.get("name"),
        description: row.get("description"),
        task_type: task_type_from_str(&task_type)?,
        classes: classes.0,
        status: dataset_status_from_str(&status)?,
    })
}

fn task_type_to_str(task_type: &TaskType) -> &'static str {
    match task_type {
        TaskType::ObjectDetection => "object_detection",
    }
}

fn task_type_from_str(value: &str) -> Result<TaskType, UseCaseError> {
    match value {
        "object_detection" => Ok(TaskType::ObjectDetection),
        _ => Err(UseCaseError::Repository(
            "postgres dataset task type invalid",
        )),
    }
}

fn dataset_status_to_str(status: DatasetStatus) -> &'static str {
    match status {
        DatasetStatus::Draft => "draft",
        DatasetStatus::Ready => "ready",
        DatasetStatus::Archived => "archived",
    }
}

fn dataset_status_from_str(value: &str) -> Result<DatasetStatus, UseCaseError> {
    match value {
        "draft" => Ok(DatasetStatus::Draft),
        "ready" => Ok(DatasetStatus::Ready),
        "archived" => Ok(DatasetStatus::Archived),
        _ => Err(UseCaseError::Repository("postgres dataset status invalid")),
    }
}
