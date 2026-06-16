import pytest
from pydantic import ValidationError
from typer.testing import CliRunner

from perception_worker.adapters.db.in_memory_job_repository import InMemoryTrainingJobRepository
from perception_worker.adapters.training.fake_trainer import FakeTrainer
from perception_worker.app.process_training_job import TrainingJobProcessor
from perception_worker.contracts.training_job import (
    TrainingHyperparametersPayload,
    TrainingJobPayload,
)


def payload_fixture() -> TrainingJobPayload:
    return TrainingJobPayload(
        job_id="job_001",
        dataset_version_id="dsv_001",
        model_family="yolo",
        base_model="yolo11n",
        hyperparameters=TrainingHyperparametersPayload(
            epochs=2,
            batch_size=1,
            image_size=640,
            learning_rate=0.001,
        ),
        classes=("cup", "book"),
    )


def test_training_job_payload_rejects_invalid_hyperparameters() -> None:
    with pytest.raises(ValidationError):
        TrainingJobPayload(
            job_id="job_001",
            dataset_version_id="dsv_001",
            model_family="yolo",
            base_model="yolo11n",
            hyperparameters=TrainingHyperparametersPayload(
                epochs=0,
                batch_size=1,
                image_size=640,
                learning_rate=0.001,
            ),
            classes=("cup",),
        )


def test_training_worker_processes_one_job_to_success() -> None:
    repository = InMemoryTrainingJobRepository([payload_fixture()])
    processor = TrainingJobProcessor(
        job_repository=repository,
        trainer=FakeTrainer(),
    )

    processed = processor.run_once()

    assert processed is True
    assert repository.running_job_ids == ["job_001"]
    assert repository.succeeded_results["job_001"].artifact_uri == "artifact://models/job_001"
    assert repository.succeeded_results["job_001"].metrics["epochs"] == 2.0


def test_training_worker_returns_false_when_queue_is_empty() -> None:
    repository = InMemoryTrainingJobRepository([])
    processor = TrainingJobProcessor(
        job_repository=repository,
        trainer=FakeTrainer(),
    )

    assert processor.run_once() is False


def test_process_once_cli_rejects_missing_database_url(monkeypatch: pytest.MonkeyPatch) -> None:
    from perception_worker.entrypoints.cli import app

    monkeypatch.delenv("PERCEPTIONLAB_DATABASE_URL", raising=False)
    runner = CliRunner()

    result = runner.invoke(app, ["process-once", "--repository-backend", "postgres"])

    assert result.exit_code == 1
    assert "PERCEPTIONLAB_DATABASE_URL is required" in result.output
