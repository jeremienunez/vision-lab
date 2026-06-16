"""Dataset ingestion storage port."""

from typing import Protocol

from perception_worker.contracts.dataset_ingestion import DatasetIngestionCommand
from perception_worker.domain.dataset_ingestion import DatasetImageSample, DatasetIngestionResult


class DatasetIngestionStore(Protocol):
    def write_dataset(
        self,
        command: DatasetIngestionCommand,
        samples: tuple[DatasetImageSample, ...],
    ) -> DatasetIngestionResult:
        """Persist materialized dataset files and return ingestion metadata."""
        ...
