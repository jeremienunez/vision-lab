"""PostgreSQL training job repository for real worker execution."""

import uuid
from collections.abc import Mapping

import psycopg
from psycopg.types.json import Jsonb

from perception_worker.contracts.training_job import (
    TrainingHyperparametersPayload,
    TrainingJobPayload,
)
from perception_worker.domain.training_result import TrainingResult


class PostgresTrainingJobRepository:
    def __init__(self, database_url: str, worker_id: str) -> None:
        self._database_url = database_url
        self._worker_id = worker_id

    def lease_next(self) -> TrainingJobPayload | None:
        with psycopg.connect(self._database_url) as connection:
            with connection.transaction():
                row = connection.execute(
                    """
                    SELECT
                      queue.training_job_id,
                      jobs.dataset_version_id,
                      jobs.model_family,
                      jobs.base_model,
                      jobs.hyperparameters,
                      versions.classes_snapshot
                    FROM training_job_queue queue
                    JOIN training_jobs jobs ON jobs.id = queue.training_job_id
                    JOIN dataset_versions versions ON versions.id = jobs.dataset_version_id
                    WHERE queue.status = 'queued'
                      AND queue.available_at <= now()
                      AND jobs.status = 'queued'
                    ORDER BY queue.created_at ASC, queue.id ASC
                    LIMIT 1
                    FOR UPDATE SKIP LOCKED
                    """
                ).fetchone()

                if row is None:
                    return None

                training_job_id = row[0]
                connection.execute(
                    """
                    UPDATE training_job_queue
                    SET status = 'leased',
                        locked_by = %s,
                        attempts = attempts + 1,
                        leased_until = now() + interval '15 minutes',
                        updated_at = now()
                    WHERE training_job_id = %s
                    """,
                    (self._worker_id, training_job_id),
                )

        return TrainingJobPayload(
            job_id=str(row[0]),
            dataset_version_id=str(row[1]),
            model_family=str(row[2]),
            base_model=row[3],
            hyperparameters=hyperparameters_payload(row[4]),
            classes=tuple(str(class_name) for class_name in row[5]),
        )

    def mark_running(self, job_id: str) -> None:
        with psycopg.connect(self._database_url) as connection:
            connection.execute(
                """
                UPDATE training_jobs
                SET status = 'running',
                    started_at = COALESCE(started_at, now())
                WHERE id = %s
                """,
                (uuid.UUID(job_id),),
            )

    def mark_succeeded(self, job_id: str, result: TrainingResult) -> None:
        training_job_id = uuid.UUID(job_id)
        model_id = uuid.uuid4()

        with psycopg.connect(self._database_url) as connection:
            with connection.transaction():
                row = connection.execute(
                    """
                    SELECT dataset_version_id, model_family
                    FROM training_jobs
                    WHERE id = %s
                    """,
                    (training_job_id,),
                ).fetchone()
                if row is None:
                    raise ValueError(f"training job does not exist: {job_id}")

                dataset_version_id = row[0]
                model_family = str(row[1])
                for metric_name, metric_value in result.metrics.items():
                    connection.execute(
                        """
                        INSERT INTO training_metrics (
                            id, training_job_id, split_name, metric_name,
                            metric_value, metadata
                        )
                        VALUES (%s, %s, 'train', %s, %s, %s)
                        """,
                        (
                            uuid.uuid4(),
                            training_job_id,
                            metric_name,
                            metric_value,
                            Jsonb({"source": "worker"}),
                        ),
                    )

                connection.execute(
                    """
                    INSERT INTO models (
                        id, name, version, training_job_id, dataset_version_id,
                        model_family, artifact_uri, metrics_summary, status
                    )
                    VALUES (%s, %s, 'v1', %s, %s, %s, %s, %s, 'candidate')
                    """,
                    (
                        model_id,
                        f"worker-{job_id}",
                        training_job_id,
                        dataset_version_id,
                        model_family,
                        result.artifact_uri,
                        Jsonb(result.metrics),
                    ),
                )
                connection.execute(
                    """
                    UPDATE training_jobs
                    SET status = 'succeeded',
                        finished_at = now(),
                        output_model_id = %s,
                        error_message = NULL
                    WHERE id = %s
                    """,
                    (model_id, training_job_id),
                )
                connection.execute(
                    """
                    UPDATE training_job_queue
                    SET status = 'completed',
                        updated_at = now()
                    WHERE training_job_id = %s
                    """,
                    (training_job_id,),
                )

    def mark_failed(self, job_id: str, error_message: str) -> None:
        training_job_id = uuid.UUID(job_id)
        with psycopg.connect(self._database_url) as connection:
            with connection.transaction():
                connection.execute(
                    """
                    UPDATE training_jobs
                    SET status = 'failed',
                        finished_at = now(),
                        error_message = %s
                    WHERE id = %s
                    """,
                    (error_message, training_job_id),
                )
                connection.execute(
                    """
                    UPDATE training_job_queue
                    SET status = 'failed',
                        last_error = %s,
                        updated_at = now()
                    WHERE training_job_id = %s
                    """,
                    (error_message, training_job_id),
                )


def hyperparameters_payload(value: Mapping[str, object]) -> TrainingHyperparametersPayload:
    epochs = integer_field(value=value, field="epochs")
    batch_size = integer_field(value=value, field="batch_size")
    image_size = integer_field(value=value, field="image_size")
    learning_rate = number_field(value=value, field="learning_rate")

    return TrainingHyperparametersPayload(
        epochs=epochs,
        batch_size=batch_size,
        image_size=image_size,
        learning_rate=learning_rate,
    )


def integer_field(value: Mapping[str, object], field: str) -> int:
    raw = value[field]
    if isinstance(raw, bool) or not isinstance(raw, int):
        raise ValueError(f"training job {field} must be an integer")
    return raw


def number_field(value: Mapping[str, object], field: str) -> float:
    raw = value[field]
    if isinstance(raw, bool) or not isinstance(raw, int | float):
        raise ValueError(f"training job {field} must be numeric")
    return float(raw)
