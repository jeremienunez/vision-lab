"""Training job repository port used by the worker app."""

from typing import Protocol

from perception_worker.contracts.training_job import TrainingJobPayload
from perception_worker.domain.training_result import TrainingResult


class TrainingJobRepository(Protocol):
    def lease_next(self) -> TrainingJobPayload | None:
        """Lease the next queued job for this worker."""
        ...

    def mark_running(self, job_id: str) -> None:
        """Mark a leased job as running."""
        ...

    def mark_succeeded(self, job_id: str, result: TrainingResult) -> None:
        """Persist successful training result metadata."""
        ...

    def mark_failed(self, job_id: str, error_message: str) -> None:
        """Persist a failed job state with a readable error."""
        ...
