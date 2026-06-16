use std::collections::BTreeMap;

use async_trait::async_trait;
use perception_app::{DatasetVersionDraft, DatasetVersionRepository, UseCaseError};
use perception_domain::{DatasetId, DatasetVersionId};
use sqlx::{PgPool, Row, postgres::PgRow, types::Json};

pub struct PostgresDatasetVersionRepository {
    pool: PgPool,
}

impl PostgresDatasetVersionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatasetVersionRepository for PostgresDatasetVersionRepository {
    async fn create(
        &self,
        version: DatasetVersionDraft,
    ) -> Result<DatasetVersionDraft, UseCaseError> {
        sqlx::query(
            r#"
            INSERT INTO dataset_versions (
                id, dataset_id, version_name, sample_count, annotation_count,
                classes_snapshot, split_config, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(version.id.into_uuid())
        .bind(version.dataset_id.into_uuid())
        .bind(&version.version_name)
        .bind(i32::try_from(version.sample_count).map_err(|_| {
            UseCaseError::Repository("postgres dataset version sample count overflow")
        })?)
        .bind(i32::try_from(version.annotation_count).map_err(|_| {
            UseCaseError::Repository("postgres dataset version annotation count overflow")
        })?)
        .bind(Json(&version.classes_snapshot))
        .bind(Json(&version.split_config))
        .bind(&version.created_by)
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres dataset version create failed"))?;

        Ok(version)
    }

    async fn get(
        &self,
        version_id: DatasetVersionId,
    ) -> Result<Option<DatasetVersionDraft>, UseCaseError> {
        let row = sqlx::query(
            r#"
            SELECT id, dataset_id, version_name, sample_count, annotation_count,
                   classes_snapshot, split_config, created_by
            FROM dataset_versions
            WHERE id = $1
            "#,
        )
        .bind(version_id.into_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres dataset version get failed"))?;

        row.map(row_to_dataset_version).transpose()
    }
}

fn row_to_dataset_version(row: PgRow) -> Result<DatasetVersionDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let dataset_id: uuid::Uuid = row.get("dataset_id");
    let sample_count: i32 = row.get("sample_count");
    let annotation_count: i32 = row.get("annotation_count");
    let classes_snapshot: Json<Vec<String>> = row.get("classes_snapshot");
    let split_config: Json<BTreeMap<String, String>> = row.get("split_config");

    Ok(DatasetVersionDraft {
        id: DatasetVersionId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres dataset version id invalid"))?,
        dataset_id: DatasetId::parse(dataset_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres dataset version dataset id invalid"))?,
        version_name: row.get("version_name"),
        sample_count: u64::try_from(sample_count).map_err(|_| {
            UseCaseError::Repository("postgres dataset version sample count invalid")
        })?,
        annotation_count: u64::try_from(annotation_count).map_err(|_| {
            UseCaseError::Repository("postgres dataset version annotation count invalid")
        })?,
        classes_snapshot: classes_snapshot.0,
        split_config: split_config.0,
        created_by: row.get("created_by"),
    })
}
