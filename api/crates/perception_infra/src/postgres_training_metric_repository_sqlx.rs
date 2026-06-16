use std::collections::BTreeMap;

use async_trait::async_trait;
use perception_app::{TrainingMetricDraft, TrainingMetricRepository, UseCaseError};
use perception_domain::{TrainingJobId, TrainingMetricId};
use sqlx::{PgPool, Row, postgres::PgRow, types::Json};

pub struct PostgresTrainingMetricRepository {
    pool: PgPool,
}

impl PostgresTrainingMetricRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TrainingMetricRepository for PostgresTrainingMetricRepository {
    async fn create(
        &self,
        metric: TrainingMetricDraft,
    ) -> Result<TrainingMetricDraft, UseCaseError> {
        sqlx::query(
            r#"
            INSERT INTO training_metrics (
                id, training_job_id, split_name, metric_name,
                metric_value, step, epoch, metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(metric.id.into_uuid())
        .bind(metric.training_job_id.into_uuid())
        .bind(&metric.split_name)
        .bind(&metric.metric_name)
        .bind(metric.metric_value)
        .bind(optional_u32_to_i32(
            metric.step,
            "postgres training metric step overflow",
        )?)
        .bind(optional_u32_to_i32(
            metric.epoch,
            "postgres training metric epoch overflow",
        )?)
        .bind(Json(&metric.metadata))
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres training metric create failed"))?;

        Ok(metric)
    }

    async fn list_by_training_job(
        &self,
        training_job_id: TrainingJobId,
    ) -> Result<Vec<TrainingMetricDraft>, UseCaseError> {
        let rows = sqlx::query(
            r#"
            SELECT id, training_job_id, split_name, metric_name,
                   metric_value, step, epoch, metadata
            FROM training_metrics
            WHERE training_job_id = $1
            ORDER BY epoch ASC NULLS LAST, step ASC NULLS LAST, metric_name ASC, id ASC
            "#,
        )
        .bind(training_job_id.into_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres training metric list failed"))?;

        rows.into_iter().map(row_to_training_metric).collect()
    }
}

fn row_to_training_metric(row: PgRow) -> Result<TrainingMetricDraft, UseCaseError> {
    let id: uuid::Uuid = row.get("id");
    let training_job_id: uuid::Uuid = row.get("training_job_id");
    let step: Option<i32> = row.get("step");
    let epoch: Option<i32> = row.get("epoch");
    let metadata: Json<BTreeMap<String, String>> = row.get("metadata");

    Ok(TrainingMetricDraft {
        id: TrainingMetricId::parse(id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres training metric id invalid"))?,
        training_job_id: TrainingJobId::parse(training_job_id.to_string()).map_err(|_| {
            UseCaseError::Repository("postgres training metric training job id invalid")
        })?,
        split_name: row.get("split_name"),
        metric_name: row.get("metric_name"),
        metric_value: row.get("metric_value"),
        step: optional_i32_to_u32(step, "postgres training metric step invalid")?,
        epoch: optional_i32_to_u32(epoch, "postgres training metric epoch invalid")?,
        metadata: metadata.0,
    })
}

fn optional_u32_to_i32(
    value: Option<u32>,
    error: &'static str,
) -> Result<Option<i32>, UseCaseError> {
    value
        .map(|value| i32::try_from(value).map_err(|_| UseCaseError::Repository(error)))
        .transpose()
}

fn optional_i32_to_u32(
    value: Option<i32>,
    error: &'static str,
) -> Result<Option<u32>, UseCaseError> {
    value
        .map(|value| u32::try_from(value).map_err(|_| UseCaseError::Repository(error)))
        .transpose()
}
