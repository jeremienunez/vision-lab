"""Tiny deterministic PyTorch training strategy for local MVP validation."""

from pathlib import Path

import torch
from torch import nn

from perception_worker.contracts.training_job import TrainingJobPayload
from perception_worker.domain.training_result import TrainingResult


class TinyTorchTrainer:
    def __init__(self, artifact_root: Path) -> None:
        self._artifact_root = artifact_root

    def train(self, job: TrainingJobPayload) -> TrainingResult:
        torch.manual_seed(17)

        class_count = len(job.classes)
        model = nn.Linear(4, class_count)
        optimizer = torch.optim.SGD(model.parameters(), lr=job.hyperparameters.learning_rate)
        loss_function = nn.CrossEntropyLoss()
        inputs, targets = self._synthetic_training_batch(class_count)

        loss_value = 0.0
        for _epoch in range(job.hyperparameters.epochs):
            optimizer.zero_grad()
            predictions = model(inputs)
            loss = loss_function(predictions, targets)
            loss.backward()
            optimizer.step()
            loss_value = float(loss.detach().item())

        artifact_path = self._write_artifact(job, model, loss_value)
        return TrainingResult(
            artifact_uri=artifact_path.as_uri(),
            metrics={
                "epochs": float(job.hyperparameters.epochs),
                "train_loss": loss_value,
                "class_count": float(class_count),
            },
        )

    def _synthetic_training_batch(self, class_count: int) -> tuple[torch.Tensor, torch.Tensor]:
        sample_count = max(class_count * 2, 2)
        rows = [
            [
                float(index),
                float(index % 2),
                float((index + 1) % 3),
                1.0,
            ]
            for index in range(sample_count)
        ]
        labels = [index % class_count for index in range(sample_count)]

        return (
            torch.tensor(rows, dtype=torch.float32),
            torch.tensor(labels, dtype=torch.long),
        )

    def _write_artifact(self, job: TrainingJobPayload, model: nn.Module, loss_value: float) -> Path:
        artifact_directory = self._artifact_root / job.job_id
        artifact_directory.mkdir(parents=True, exist_ok=True)
        artifact_path = (artifact_directory / "model.pt").resolve()
        torch.save(
            {
                "model_state_dict": model.state_dict(),
                "classes": job.classes,
                "model_family": job.model_family,
                "base_model": job.base_model,
                "metrics": {"train_loss": loss_value},
            },
            artifact_path,
        )
        return artifact_path
