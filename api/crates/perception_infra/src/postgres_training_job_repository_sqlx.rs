use async_trait::async_trait;
use perception_app::{TrainingJobDraft, TrainingJobRepository, UseCaseError};
use perception_domain::{
    DatasetVersionId, TrainingHyperparameters, TrainingJobId, TrainingJobStatus,
};
use serde_json::{Value, json};
use sqlx::{PgPool, Row, postgres::PgRow, types::Json};

pub struct PostgresTrainingJobRepository {
    pool: PgPool,
}

impl PostgresTrainingJobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TrainingJobRepository for PostgresTrainingJobRepository {
    async fn create(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        sqlx::query(
            r#"
            INSERT INTO training_jobs (
                id, dataset_version_id, model_family, base_model,
                status, hyperparameters, error_message
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(job.id.into_uuid())
        .bind(job.dataset_version_id.into_uuid())
        .bind(&job.model_family)
        .bind(&job.base_model)
        .bind(training_job_status_to_str(job.status))
        .bind(Json(training_hyperparameters_to_json(job.hyperparameters)))
        .bind(&job.error_message)
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres training job create failed"))?;

        Ok(job)
    }

    async fn get(&self, job_id: TrainingJobId) -> Result<Option<TrainingJobDraft>, UseCaseError> {
        let row = sqlx::query(
            r#"
            SELECT id, dataset_version_id, model_family, base_model,
                   status, hyperparameters, error_message
            FROM training_jobs
            WHERE id = $1
            "#,
        )
        .bind(job_id.into_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres training job get failed"))?;

        row.map(row_to_training_job).transpose()
    }

    async fn update(&self, job: TrainingJobDraft) -> Result<TrainingJobDraft, UseCaseError> {
        let result = sqlx::query(
            r#"
            UPDATE training_jobs
            SET dataset_version_id = $2,
                model_family = $3,
                base_model = $4,
                status = $5,
                hyperparameters = $6,
                error_message = $7,
                started_at = CASE
                    WHEN $5 = 'running' AND started_at IS NULL THEN now()
                    ELSE started_at
                END,
                finished_at = CASE
                    WHEN $5 IN ('succeeded', 'failed', 'cancelled') THEN now()
                    ELSE finished_at
                END
            WHERE id = $1
            "#,
        )
        .bind(job.id.into_uuid())
        .bind(job.dataset_version_id.into_uuid())
        .bind(&job.model_family)
        .bind(&job.base_model)
        .bind(training_job_status_to_str(job.status))
        .bind(Json(training_hyperparameters_to_json(job.hyperparameters)))
        .bind(&job.error_message)
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres training job update failed"))?;

        if result.rows_affected() == 0 {
            return Err(UseCaseError::NotFound("training job not found"));
        }

        Ok(job)
    }
}

fn row_to_training_job(row: PgRow) -> Result<TrainingJobDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let dataset_version_id: uuid::Uuid = row.get("dataset_version_id");
    let status: String = row.get("status");
    let hyperparameters: Json<Value> = row.get("hyperparameters");

    Ok(TrainingJobDraft {
        id: TrainingJobId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres training job id invalid"))?,
        dataset_version_id: DatasetVersionId::parse(dataset_version_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres training job version id invalid"))?,
        model_family: row.get("model_family"),
        base_model: row.get("base_model"),
        status: training_job_status_from_str(&status)?,
        hyperparameters: training_hyperparameters_from_json(&hyperparameters.0)?,
        error_message: row.get("error_message"),
    })
}

fn training_hyperparameters_to_json(hyperparameters: TrainingHyperparameters) -> Value {
    json!({
        "epochs": hyperparameters.epochs,
        "batch_size": hyperparameters.batch_size,
        "image_size": hyperparameters.image_size,
        "learning_rate": hyperparameters.learning_rate,
    })
}

fn training_hyperparameters_from_json(
    value: &Value,
) -> Result<TrainingHyperparameters, UseCaseError> {
    let epochs = u16_json_field(value, "epochs")?;
    let batch_size = u16_json_field(value, "batch_size")?;
    let image_size = u16_json_field(value, "image_size")?;
    let learning_rate =
        value
            .get("learning_rate")
            .and_then(Value::as_f64)
            .ok_or(UseCaseError::Repository(
                "postgres training job learning rate invalid",
            ))? as f32;

    TrainingHyperparameters::new(epochs, batch_size, image_size, learning_rate)
        .map_err(|_| UseCaseError::Repository("postgres training job hyperparameters invalid"))
}

fn u16_json_field(value: &Value, field: &'static str) -> Result<u16, UseCaseError> {
    let raw = value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or(UseCaseError::Repository(
            "postgres training job hyperparameter invalid",
        ))?;

    u16::try_from(raw)
        .map_err(|_| UseCaseError::Repository("postgres training job hyperparameter overflow"))
}

fn training_job_status_to_str(status: TrainingJobStatus) -> &'static str {
    match status {
        TrainingJobStatus::Queued => "queued",
        TrainingJobStatus::Running => "running",
        TrainingJobStatus::Succeeded => "succeeded",
        TrainingJobStatus::Failed => "failed",
        TrainingJobStatus::Cancelled => "cancelled",
    }
}

fn training_job_status_from_str(value: &str) -> Result<TrainingJobStatus, UseCaseError> {
    match value {
        "queued" => Ok(TrainingJobStatus::Queued),
        "running" => Ok(TrainingJobStatus::Running),
        "succeeded" => Ok(TrainingJobStatus::Succeeded),
        "failed" => Ok(TrainingJobStatus::Failed),
        "cancelled" => Ok(TrainingJobStatus::Cancelled),
        _ => Err(UseCaseError::Repository(
            "postgres training job status invalid",
        )),
    }
}
