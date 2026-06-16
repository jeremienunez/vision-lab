import os
import uuid
from pathlib import Path

import psycopg
import pytest
from psycopg.types.json import Jsonb

from perception_worker.adapters.db.postgres_job_repository import PostgresTrainingJobRepository
from perception_worker.domain.training_result import TrainingResult


@pytest.fixture()
def database_url() -> str:
    value = os.environ.get("PERCEPTIONLAB_DATABASE_URL")
    if not value:
        pytest.skip("PERCEPTIONLAB_DATABASE_URL is required for PostgreSQL worker tests")
    return value


def test_postgres_job_repository_leases_and_completes_job(database_url: str) -> None:
    ensure_schema(database_url)
    job_id, version_id = insert_queued_job(database_url)
    repository = PostgresTrainingJobRepository(
        database_url=database_url,
        worker_id=f"worker-{job_id.hex[:8]}",
    )

    job = repository.lease_next()

    assert job is not None
    assert job.job_id == str(job_id)
    assert job.dataset_version_id == str(version_id)
    assert job.model_family == "tiny_torch"
    assert job.hyperparameters.epochs == 2
    assert job.classes == ("Mask", "Gloves")

    repository.mark_running(job.job_id)
    repository.mark_succeeded(
        job.job_id,
        TrainingResult(
            artifact_uri=f"file:///tmp/perceptionlab/{job.job_id}/model.pt",
            metrics={"epochs": 2.0, "train_loss": 0.25},
        ),
    )

    with psycopg.connect(database_url) as connection:
        row = connection.execute(
            """
            SELECT status, output_model_id IS NOT NULL AS has_model
            FROM training_jobs
            WHERE id = %s
            """,
            (job_id,),
        ).fetchone()
        queue_row = connection.execute(
            """
            SELECT status, locked_by, attempts
            FROM training_job_queue
            WHERE training_job_id = %s
            """,
            (job_id,),
        ).fetchone()
        metric_count_row = connection.execute(
            "SELECT count(*) FROM training_metrics WHERE training_job_id = %s",
            (job_id,),
        ).fetchone()
        model_row = connection.execute(
            """
            SELECT artifact_uri, dataset_version_id, status
            FROM models
            WHERE training_job_id = %s
            """,
            (job_id,),
        ).fetchone()

    assert metric_count_row is not None
    assert row == ("succeeded", True)
    assert queue_row == ("completed", f"worker-{job_id.hex[:8]}", 1)
    assert metric_count_row[0] == 2
    assert model_row == (
        f"file:///tmp/perceptionlab/{job_id}/model.pt",
        version_id,
        "candidate",
    )


def test_postgres_job_repository_marks_failed_jobs(database_url: str) -> None:
    ensure_schema(database_url)
    job_id, _version_id = insert_queued_job(database_url)
    repository = PostgresTrainingJobRepository(
        database_url=database_url,
        worker_id=f"worker-{job_id.hex[:8]}",
    )
    job = repository.lease_next()
    assert job is not None

    repository.mark_running(job.job_id)
    repository.mark_failed(job.job_id, "training failed")

    with psycopg.connect(database_url) as connection:
        row = connection.execute(
            """
            SELECT jobs.status, jobs.error_message, queue.status, queue.last_error
            FROM training_jobs jobs
            JOIN training_job_queue queue ON queue.training_job_id = jobs.id
            WHERE jobs.id = %s
            """,
            (job_id,),
        ).fetchone()

    assert row == ("failed", "training failed", "failed", "training failed")


def ensure_schema(database_url: str) -> None:
    with psycopg.connect(database_url, autocommit=True) as connection:
        exists_row = connection.execute(
            "SELECT to_regclass('public.training_jobs') IS NOT NULL"
        ).fetchone()
        assert exists_row is not None
        exists = exists_row[0]
        if exists:
            return

        migration = (
            Path(__file__).parents[2] / "api" / "migrations" / "0001_initial_schema.sql"
        ).read_text(encoding="utf-8")
        connection.execute(migration)


def insert_queued_job(database_url: str) -> tuple[uuid.UUID, uuid.UUID]:
    dataset_id = uuid.uuid4()
    version_id = uuid.uuid4()
    job_id = uuid.uuid4()
    dataset_name = f"worker-postgres-{dataset_id}"

    with psycopg.connect(database_url) as connection:
        connection.execute(
            "UPDATE training_job_queue SET status = 'completed' WHERE status = 'queued'"
        )
        connection.execute(
            """
            INSERT INTO datasets (id, name, description, task_type, classes, status)
            VALUES (%s, %s, %s, 'object_detection', %s, 'draft')
            """,
            (
                dataset_id,
                dataset_name,
                "Worker PostgreSQL test dataset",
                Jsonb(["Mask", "Gloves"]),
            ),
        )
        connection.execute(
            """
            INSERT INTO dataset_classes (dataset_id, class_id, class_name)
            VALUES (%s, 0, 'Mask'), (%s, 1, 'Gloves')
            """,
            (dataset_id, dataset_id),
        )
        connection.execute(
            """
            INSERT INTO dataset_versions (
                id, dataset_id, version_name, sample_count, annotation_count,
                classes_snapshot, split_config, created_by
            )
            VALUES (%s, %s, 'v1', 2, 3, %s, %s, 'worker-test')
            """,
            (
                version_id,
                dataset_id,
                Jsonb(["Mask", "Gloves"]),
                Jsonb({"train": "80", "validation": "10", "test": "10"}),
            ),
        )
        connection.execute(
            """
            INSERT INTO training_jobs (
                id, dataset_version_id, model_family, base_model, status, hyperparameters
            )
            VALUES (%s, %s, 'tiny_torch', NULL, 'queued', %s)
            """,
            (
                job_id,
                version_id,
                Jsonb(
                    {
                        "epochs": 2,
                        "batch_size": 1,
                        "image_size": 64,
                        "learning_rate": 0.01,
                    }
                ),
            ),
        )
        connection.execute(
            """
            INSERT INTO training_job_queue (training_job_id, status)
            VALUES (%s, 'queued')
            """,
            (job_id,),
        )

    return job_id, version_id
