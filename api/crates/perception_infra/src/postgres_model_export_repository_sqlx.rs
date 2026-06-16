use async_trait::async_trait;
use perception_app::{ModelExportDraft, ModelExportRepository, UseCaseError};
use perception_domain::{ExportStatus, ModelExportId, ModelId};
use sqlx::{PgPool, Row, postgres::PgRow};

pub struct PostgresModelExportRepository {
    pool: PgPool,
}

impl PostgresModelExportRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ModelExportRepository for PostgresModelExportRepository {
    async fn create(&self, export: ModelExportDraft) -> Result<ModelExportDraft, UseCaseError> {
        sqlx::query(
            r#"
            INSERT INTO model_exports (
                id, model_id, format, artifact_uri, status, finished_at, error_message
            )
            VALUES (
                $1, $2, $3, $4, $5,
                CASE WHEN $5 IN ('succeeded', 'failed') THEN now() ELSE NULL END,
                $6
            )
            "#,
        )
        .bind(export.id.into_uuid())
        .bind(export.model_id.into_uuid())
        .bind(&export.format)
        .bind(&export.artifact_uri)
        .bind(export_status_to_str(export.status))
        .bind(&export.error_message)
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres model export create failed"))?;

        Ok(export)
    }

    async fn list_by_model(
        &self,
        model_id: ModelId,
    ) -> Result<Vec<ModelExportDraft>, UseCaseError> {
        let rows = sqlx::query(
            r#"
            SELECT id, model_id, format, artifact_uri, status, error_message
            FROM model_exports
            WHERE model_id = $1
            ORDER BY created_at ASC, id ASC
            "#,
        )
        .bind(model_id.into_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres model export list failed"))?;

        rows.into_iter().map(row_to_model_export).collect()
    }
}

fn row_to_model_export(row: PgRow) -> Result<ModelExportDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let model_id: uuid::Uuid = row.get("model_id");
    let status: String = row.get("status");

    Ok(ModelExportDraft {
        id: ModelExportId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres model export id invalid"))?,
        model_id: ModelId::parse(model_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres model export model id invalid"))?,
        format: row.get("format"),
        artifact_uri: row.get("artifact_uri"),
        status: export_status_from_str(&status)?,
        error_message: row.get("error_message"),
    })
}

fn export_status_to_str(status: ExportStatus) -> &'static str {
    match status {
        ExportStatus::Queued => "queued",
        ExportStatus::Running => "running",
        ExportStatus::Succeeded => "succeeded",
        ExportStatus::Failed => "failed",
    }
}

fn export_status_from_str(value: &str) -> Result<ExportStatus, UseCaseError> {
    match value {
        "queued" => Ok(ExportStatus::Queued),
        "running" => Ok(ExportStatus::Running),
        "succeeded" => Ok(ExportStatus::Succeeded),
        "failed" => Ok(ExportStatus::Failed),
        _ => Err(UseCaseError::Repository(
            "postgres model export status invalid",
        )),
    }
}
