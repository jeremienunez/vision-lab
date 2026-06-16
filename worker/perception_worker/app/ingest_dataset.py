"""Application service for external dataset ingestion."""

from perception_worker.contracts.dataset_ingestion import DatasetIngestionCommand
from perception_worker.domain.dataset_ingestion import (
    DatasetImageSample,
    DatasetIngestionError,
    DatasetIngestionResult,
)
from perception_worker.ports.dataset_ingestion_store import DatasetIngestionStore
from perception_worker.ports.dataset_source import DatasetSource


class DatasetIngestionService:
    def __init__(self, source: DatasetSource, store: DatasetIngestionStore) -> None:
        self._source = source
        self._store = store

    def ingest(self, command: DatasetIngestionCommand) -> DatasetIngestionResult:
        samples = self._source.read_samples(
            source_dataset=command.source_dataset,
            split=command.split,
            max_samples=command.max_samples,
            classes=command.classes,
        )
        self._validate_annotations(samples=samples, classes=command.classes)

        return self._store.write_dataset(command=command, samples=samples)

    def _validate_annotations(
        self,
        samples: tuple[DatasetImageSample, ...],
        classes: tuple[str, ...],
    ) -> None:
        declared_classes = set(classes)

        for sample in samples:
            for annotation in sample.annotations:
                if annotation.class_name not in declared_classes:
                    raise DatasetIngestionError("annotation class is not declared")
