use async_trait::async_trait;
use perception_app::{AnnotationDraft, AnnotationRepository, UseCaseError};
use perception_domain::{AnnotationId, DatasetId, SampleId};
use sqlx::{PgPool, Row, postgres::PgRow};

pub struct PostgresAnnotationRepository {
    pool: PgPool,
}

impl PostgresAnnotationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AnnotationRepository for PostgresAnnotationRepository {
    async fn create(&self, annotation: AnnotationDraft) -> Result<AnnotationDraft, UseCaseError> {
        sqlx::query(
            r#"
            INSERT INTO annotations (
                id, sample_id, dataset_id, class_id, class_name,
                bbox_x, bbox_y, bbox_width, bbox_height,
                format, confidence, source
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(annotation.id.into_uuid())
        .bind(annotation.sample_id.into_uuid())
        .bind(annotation.dataset_id.into_uuid())
        .bind(
            i32::try_from(annotation.class_id)
                .map_err(|_| UseCaseError::Repository("postgres annotation class id overflow"))?,
        )
        .bind(&annotation.class_name)
        .bind(annotation.bbox_x)
        .bind(annotation.bbox_y)
        .bind(annotation.bbox_width)
        .bind(annotation.bbox_height)
        .bind(&annotation.format)
        .bind(annotation.confidence)
        .bind(&annotation.source)
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres annotation create failed"))?;

        Ok(annotation)
    }

    async fn list_by_sample(
        &self,
        sample_id: SampleId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError> {
        let rows = sqlx::query(
            r#"
            SELECT id, sample_id, dataset_id, class_id, class_name,
                   bbox_x::real AS bbox_x,
                   bbox_y::real AS bbox_y,
                   bbox_width::real AS bbox_width,
                   bbox_height::real AS bbox_height,
                   format,
                   confidence::real AS confidence,
                   source
            FROM annotations
            WHERE sample_id = $1
            ORDER BY created_at ASC, id ASC
            "#,
        )
        .bind(sample_id.into_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres annotation list failed"))?;

        rows.into_iter().map(row_to_annotation).collect()
    }

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<AnnotationDraft>, UseCaseError> {
        let rows = sqlx::query(
            r#"
            SELECT id, sample_id, dataset_id, class_id, class_name,
                   bbox_x::real AS bbox_x,
                   bbox_y::real AS bbox_y,
                   bbox_width::real AS bbox_width,
                   bbox_height::real AS bbox_height,
                   format,
                   confidence::real AS confidence,
                   source
            FROM annotations
            WHERE dataset_id = $1
            ORDER BY created_at ASC, id ASC
            "#,
        )
        .bind(dataset_id.into_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres annotation list failed"))?;

        rows.into_iter().map(row_to_annotation).collect()
    }
}

fn row_to_annotation(row: PgRow) -> Result<AnnotationDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let sample_id: uuid::Uuid = row.get("sample_id");
    let dataset_id: uuid::Uuid = row.get("dataset_id");
    let class_id: i32 = row.get("class_id");

    Ok(AnnotationDraft {
        id: AnnotationId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres annotation id invalid"))?,
        sample_id: SampleId::parse(sample_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres annotation sample id invalid"))?,
        dataset_id: DatasetId::parse(dataset_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres annotation dataset id invalid"))?,
        class_name: row.get("class_name"),
        class_id: u32::try_from(class_id)
            .map_err(|_| UseCaseError::Repository("postgres annotation class id invalid"))?,
        bbox_x: row.get("bbox_x"),
        bbox_y: row.get("bbox_y"),
        bbox_width: row.get("bbox_width"),
        bbox_height: row.get("bbox_height"),
        format: row.get("format"),
        confidence: row.get("confidence"),
        source: row.get("source"),
    })
}
