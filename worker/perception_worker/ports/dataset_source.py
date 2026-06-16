"""External dataset source port."""

from typing import Protocol

from perception_worker.domain.dataset_ingestion import DatasetImageSample


class DatasetSource(Protocol):
    def read_samples(
        self,
        source_dataset: str,
        split: str,
        max_samples: int | None,
        classes: tuple[str, ...],
    ) -> tuple[DatasetImageSample, ...]:
        """Read normalized image samples from an external dataset source."""
        ...
