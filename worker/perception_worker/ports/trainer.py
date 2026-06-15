"""Training strategy port."""

from typing import Protocol

from perception_worker.contracts.training_job import TrainingJobPayload
from perception_worker.domain.training_result import TrainingResult


class Trainer(Protocol):
    def train(self, job: TrainingJobPayload) -> TrainingResult:
        """Train a model for one job and return persisted artifact metadata."""
        ...
