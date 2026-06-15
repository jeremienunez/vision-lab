from pathlib import Path

from perception_worker.adapters.training.tiny_torch_trainer import TinyTorchTrainer
from perception_worker.contracts.training_job import (
    TrainingHyperparametersPayload,
    TrainingJobPayload,
)


def test_tiny_torch_trainer_runs_training_loop_and_writes_artifact(tmp_path: Path) -> None:
    trainer = TinyTorchTrainer(artifact_root=tmp_path)
    job = TrainingJobPayload(
        job_id="job_001",
        dataset_version_id="dsv_001",
        model_family="tiny_torch",
        base_model=None,
        hyperparameters=TrainingHyperparametersPayload(
            epochs=2,
            batch_size=1,
            image_size=64,
            learning_rate=0.01,
        ),
        classes=("cup", "book"),
    )

    result = trainer.train(job)

    artifact_path = Path(result.artifact_uri.removeprefix("file://"))
    assert artifact_path == (tmp_path / "job_001" / "model.pt").resolve()
    assert artifact_path.exists()
    assert artifact_path.stat().st_size > 0
    assert result.metrics["epochs"] == 2.0
    assert result.metrics["train_loss"] > 0.0
