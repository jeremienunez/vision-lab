"""Deterministic trainer strategy for local worker smoke tests."""

from perception_worker.contracts.training_job import TrainingJobPayload
from perception_worker.domain.training_result import TrainingResult


class FakeTrainer:
    def train(self, job: TrainingJobPayload) -> TrainingResult:
        return TrainingResult(
            artifact_uri=f"artifact://models/{job.job_id}",
            metrics={
                "epochs": float(job.hyperparameters.epochs),
                "sample_classes": float(len(job.classes)),
            },
        )
