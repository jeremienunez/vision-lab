"""In-memory training job repository for local worker execution and tests."""

from collections.abc import Iterable

from perception_worker.contracts.training_job import TrainingJobPayload
from perception_worker.domain.training_result import TrainingResult


class InMemoryTrainingJobRepository:
    def __init__(self, jobs: Iterable[TrainingJobPayload]) -> None:
        self._queued_jobs = list(jobs)
        self.running_job_ids: list[str] = []
        self.succeeded_results: dict[str, TrainingResult] = {}
        self.failed_errors: dict[str, str] = {}

    def lease_next(self) -> TrainingJobPayload | None:
        if not self._queued_jobs:
            return None
        return self._queued_jobs.pop(0)

    def mark_running(self, job_id: str) -> None:
        self.running_job_ids.append(job_id)

    def mark_succeeded(self, job_id: str, result: TrainingResult) -> None:
        self.succeeded_results[job_id] = result

    def mark_failed(self, job_id: str, error_message: str) -> None:
        self.failed_errors[job_id] = error_message
