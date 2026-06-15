"""Application service for processing one leased training job."""

from perception_worker.ports.job_repository import TrainingJobRepository
from perception_worker.ports.trainer import Trainer


class TrainingJobProcessor:
    def __init__(self, job_repository: TrainingJobRepository, trainer: Trainer) -> None:
        self._job_repository = job_repository
        self._trainer = trainer

    def run_once(self) -> bool:
        job = self._job_repository.lease_next()
        if job is None:
            return False

        self._job_repository.mark_running(job.job_id)

        try:
            result = self._trainer.train(job)
        except Exception as error:
            self._job_repository.mark_failed(job.job_id, str(error))
            raise

        self._job_repository.mark_succeeded(job.job_id, result)
        return True
