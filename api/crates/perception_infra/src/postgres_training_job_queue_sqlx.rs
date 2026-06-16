use async_trait::async_trait;
use perception_app::{
    TrainingJobQueue, TrainingJobQueueEntry, TrainingJobQueueStatus, UseCaseError,
};
use perception_domain::TrainingJobId;
use sqlx::{PgPool, Row, postgres::PgRow};

pub struct PostgresTrainingJobQueue {
    pool: PgPool,
}

impl PostgresTrainingJobQueue {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TrainingJobQueue for PostgresTrainingJobQueue {
    async fn enqueue(
        &self,
        entry: TrainingJobQueueEntry,
    ) -> Result<TrainingJobQueueEntry, UseCaseError> {
        sqlx::query(
            r#"
            INSERT INTO training_job_queue (
                training_job_id, status, locked_by, attempts
            )
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(entry.training_job_id.into_uuid())
        .bind(training_job_queue_status_to_str(entry.status))
        .bind(&entry.locked_by)
        .bind(i32::try_from(entry.attempts).map_err(|_| {
            UseCaseError::Repository("postgres training job queue attempts overflow")
        })?)
        .execute(&self.pool)
        .await
        .map_err(|_| UseCaseError::Repository("postgres training job queue enqueue failed"))?;

        Ok(entry)
    }

    async fn lease_next(
        &self,
        worker_id: String,
    ) -> Result<Option<TrainingJobQueueEntry>, UseCaseError> {
        let mut transaction =
            self.pool.begin().await.map_err(|_| {
                UseCaseError::Repository("postgres training job queue lease failed")
            })?;

        let Some(row) = sqlx::query(
            r#"
            SELECT training_job_id
            FROM training_job_queue
            WHERE status = 'queued' AND available_at <= now()
            ORDER BY created_at ASC, id ASC
            LIMIT 1
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| UseCaseError::Repository("postgres training job queue lease failed"))?
        else {
            transaction.commit().await.map_err(|_| {
                UseCaseError::Repository("postgres training job queue lease failed")
            })?;
            return Ok(None);
        };

        let training_job_id: uuid::Uuid = row.get("training_job_id");
        let leased = sqlx::query(
            r#"
            UPDATE training_job_queue
            SET status = 'leased',
                locked_by = $1,
                attempts = attempts + 1,
                leased_until = now() + interval '15 minutes',
                updated_at = now()
            WHERE training_job_id = $2
            RETURNING training_job_id, status, locked_by, attempts
            "#,
        )
        .bind(worker_id)
        .bind(training_job_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| UseCaseError::Repository("postgres training job queue lease failed"))?;

        transaction
            .commit()
            .await
            .map_err(|_| UseCaseError::Repository("postgres training job queue lease failed"))?;

        row_to_queue_entry(leased).map(Some)
    }
}

fn row_to_queue_entry(row: PgRow) -> Result<TrainingJobQueueEntry, UseCaseError> {
    let training_job_id: uuid::Uuid = row.get("training_job_id");
    let status: String = row.get("status");
    let attempts: i32 = row.get("attempts");

    Ok(TrainingJobQueueEntry {
        training_job_id: TrainingJobId::parse(training_job_id.to_string())
            .map_err(|_| UseCaseError::Repository("postgres training job queue job id invalid"))?,
        status: training_job_queue_status_from_str(&status)?,
        locked_by: row.get("locked_by"),
        attempts: u32::try_from(attempts).map_err(|_| {
            UseCaseError::Repository("postgres training job queue attempts invalid")
        })?,
    })
}

fn training_job_queue_status_to_str(status: TrainingJobQueueStatus) -> &'static str {
    match status {
        TrainingJobQueueStatus::Queued => "queued",
        TrainingJobQueueStatus::Leased => "leased",
        TrainingJobQueueStatus::Completed => "completed",
        TrainingJobQueueStatus::Failed => "failed",
        TrainingJobQueueStatus::Cancelled => "cancelled",
    }
}

fn training_job_queue_status_from_str(value: &str) -> Result<TrainingJobQueueStatus, UseCaseError> {
    match value {
        "queued" => Ok(TrainingJobQueueStatus::Queued),
        "leased" => Ok(TrainingJobQueueStatus::Leased),
        "completed" => Ok(TrainingJobQueueStatus::Completed),
        "failed" => Ok(TrainingJobQueueStatus::Failed),
        "cancelled" => Ok(TrainingJobQueueStatus::Cancelled),
        _ => Err(UseCaseError::Repository(
            "postgres training job queue status invalid",
        )),
    }
}
