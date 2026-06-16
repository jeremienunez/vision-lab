use async_trait::async_trait;
use perception_app::{SampleDraft, SampleRepository, UseCaseError};
use perception_domain::{DatasetId, SampleId};
use sqlx::{PgPool, Row, postgres::PgRow, types::Json};
use std::collections::BTreeMap;

pub struct PostgresSampleRepository {
    pool: PgPool,
}

impl PostgresSampleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SampleRepository for PostgresSampleRepository {
    async fn create(&self, sample: SampleDraft) -> Result<SampleDraft, UseCaseError> {
        let size_bytes = i64::try_from(sample.size_bytes)
            .map_err(|_| UseCaseError::Repository("postgres sample size overflow"))?;
        sqlx::query(
            r#"
            INSERT INTO samples (
                id, dataset_id, storage_uri, filename, mime_type,
                width, height, size_bytes, checksum, source, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(sample.id.into_uuid())
        .bind(sample.dataset_id.into_uuid())
        .bind(&sample.storage_uri)
        .bind(&sample.filename)
        .bind(&sample.mime_type)
        .bind(
            i32::try_from(sample.width)
                .map_err(|_| UseCaseError::Repository("postgres sample width overflow"))?,
        )
        .bind(
            i32::try_from(sample.height)
                .map_err(|_| UseCaseError::Repository("postgres sample height overflow"))?,
        )
        .bind(size_bytes)
        .bind(&sample.checksum)
        .bind(&sample.source)
        .bind(Json(&sample.metadata))
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres sample create failed"))?;

        Ok(sample)
    }

    async fn get(&self, sample_id: SampleId) -> Result<Option<SampleDraft>, UseCaseError> {
        let row = sqlx::query(
            r#"
            SELECT id, dataset_id, storage_uri, filename, mime_type,
                   width, height, size_bytes, checksum, source, metadata
            FROM samples
            WHERE id = $1
            "#,
        )
        .bind(sample_id.into_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres sample get failed"))?;

        row.map(row_to_sample).transpose()
    }

    async fn list_by_dataset(
        &self,
        dataset_id: DatasetId,
    ) -> Result<Vec<SampleDraft>, UseCaseError> {
        let rows = sqlx::query(
            r#"
            SELECT id, dataset_id, storage_uri, filename, mime_type,
                   width, height, size_bytes, checksum, source, metadata
            FROM samples
            WHERE dataset_id = $1
            ORDER BY created_at ASC, id ASC
            "#,
        )
        .bind(dataset_id.into_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres sample list failed"))?;

        rows.into_iter().map(row_to_sample).collect()
    }
}

fn row_to_sample(row: PgRow) -> Result<SampleDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let dataset_id: uuid::Uuid = row.get("dataset_id");
    let width: i32 = row.get("width");
    let height: i32 = row.get("height");
    let size_bytes: i64 = row.get("size_bytes");
    let metadata: Json<BTreeMap<String, String>> = row.get("metadata");

    Ok(SampleDraft {
        id: SampleId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres sample id invalid"))?,
        dataset_id: DatasetId::parse(dataset_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres sample dataset id invalid"))?,
        storage_uri: row.get("storage_uri"),
        filename: row.get("filename"),
        mime_type: row.get("mime_type"),
        width: u32::try_from(width)
            .map_err(|_| UseCaseError::Repository("postgres sample width invalid"))?,
        height: u32::try_from(height)
            .map_err(|_| UseCaseError::Repository("postgres sample height invalid"))?,
        size_bytes: u64::try_from(size_bytes)
            .map_err(|_| UseCaseError::Repository("postgres sample size invalid"))?,
        checksum: row.get("checksum"),
        source: row.get("source"),
        metadata: metadata.0,
    })
}
