use std::collections::BTreeMap;

use async_trait::async_trait;
use perception_app::{ModelDraft, ModelRepository, UseCaseError};
use perception_domain::{DatasetVersionId, ModelId, ModelStatus, TrainingJobId};
use serde_json::Value;
use sqlx::{PgPool, Row, postgres::PgRow, types::Json};

pub struct PostgresModelRepository {
    pool: PgPool,
}

impl PostgresModelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ModelRepository for PostgresModelRepository {
    async fn create(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        sqlx::query(
            r#"
            INSERT INTO models (
                id, name, version, training_job_id, dataset_version_id,
                model_family, artifact_uri, metrics_summary, status, promoted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CASE WHEN $9 = 'promoted' THEN now() ELSE NULL END)
            "#,
        )
        .bind(model.id.into_uuid())
        .bind(&model.name)
        .bind(&model.version)
        .bind(model.training_job_id.into_uuid())
        .bind(model.dataset_version_id.into_uuid())
        .bind(&model.model_family)
        .bind(&model.artifact_uri)
        .bind(Json(&model.metrics_summary))
        .bind(model_status_to_str(model.status))
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres model create failed"))?;

        Ok(model)
    }

    async fn list(&self) -> Result<Vec<ModelDraft>, UseCaseError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, version, training_job_id, dataset_version_id,
                   model_family, artifact_uri, metrics_summary, status
            FROM models
            ORDER BY created_at ASC, id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres model list failed"))?;

        rows.into_iter().map(row_to_model).collect()
    }

    async fn get(&self, model_id: ModelId) -> Result<Option<ModelDraft>, UseCaseError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, version, training_job_id, dataset_version_id,
                   model_family, artifact_uri, metrics_summary, status
            FROM models
            WHERE id = $1
            "#,
        )
        .bind(model_id.into_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres model get failed"))?;

        row.map(row_to_model).transpose()
    }

    async fn update(&self, model: ModelDraft) -> Result<ModelDraft, UseCaseError> {
        let result = sqlx::query(
            r#"
            UPDATE models
            SET name = $2,
                version = $3,
                training_job_id = $4,
                dataset_version_id = $5,
                model_family = $6,
                artifact_uri = $7,
                metrics_summary = $8,
                status = $9,
                promoted_at = CASE
                    WHEN $9 = 'promoted' AND promoted_at IS NULL THEN now()
                    ELSE promoted_at
                END
            WHERE id = $1
            "#,
        )
        .bind(model.id.into_uuid())
        .bind(&model.name)
        .bind(&model.version)
        .bind(model.training_job_id.into_uuid())
        .bind(model.dataset_version_id.into_uuid())
        .bind(&model.model_family)
        .bind(&model.artifact_uri)
        .bind(Json(&model.metrics_summary))
        .bind(model_status_to_str(model.status))
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres model update failed"))?;

        if result.rows_affected() == 0 {
            return Err(UseCaseError::NotFound("model not found"));
        }

        Ok(model)
    }
}

fn row_to_model(row: PgRow) -> Result<ModelDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let training_job_id: uuid::Uuid = row.get("training_job_id");
    let dataset_version_id: uuid::Uuid = row.get("dataset_version_id");
    let metrics_summary: Json<Value> = row.get("metrics_summary");
    let status: String = row.get("status");

    Ok(ModelDraft {
        id: ModelId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres model id invalid"))?,
        name: row.get("name"),
        version: row.get("version"),
        training_job_id: TrainingJobId::parse(training_job_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres model training job id invalid"))?,
        dataset_version_id: DatasetVersionId::parse(dataset_version_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres model dataset version id invalid"))?,
        model_family: row.get("model_family"),
        artifact_uri: row.get("artifact_uri"),
        metrics_summary: metrics_summary_from_json(metrics_summary.0)?,
        status: model_status_from_str(&status)?,
    })
}

fn metrics_summary_from_json(value: Value) -> Result<BTreeMap<String, String>, UseCaseError> {
    let Value::Object(object) = value else {
        return Err(UseCaseError::Repository(
            "postgres model metrics summary invalid",
        ));
    };

    Ok(object
        .into_iter()
        .map(|(key, value)| (key, json_value_to_string(value)))
        .collect())
}

fn json_value_to_string(value: Value) -> String {
    match value {
        Value::String(value) => value,
        Value::Null => "null".to_owned(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::Array(_) | Value::Object(_) => value.to_string(),
    }
}

fn model_status_to_str(status: ModelStatus) -> &'static str {
    match status {
        ModelStatus::Candidate => "candidate",
        ModelStatus::Validated => "validated",
        ModelStatus::Promoted => "promoted",
        ModelStatus::Archived => "archived",
    }
}

fn model_status_from_str(value: &str) -> Result<ModelStatus, UseCaseError> {
    match value {
        "candidate" => Ok(ModelStatus::Candidate),
        "validated" => Ok(ModelStatus::Validated),
        "promoted" => Ok(ModelStatus::Promoted),
        "archived" => Ok(ModelStatus::Archived),
        _ => Err(UseCaseError::Repository("postgres model status invalid")),
    }
}
