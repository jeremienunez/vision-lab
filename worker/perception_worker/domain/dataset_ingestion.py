"""Domain objects for external dataset ingestion."""

from dataclasses import dataclass
from pathlib import Path


class DatasetIngestionError(RuntimeError):
    """Raised when an external dataset cannot be ingested safely."""


@dataclass(frozen=True)
class DatasetAnnotation:
    class_name: str
    bbox_x: float
    bbox_y: float
    bbox_width: float
    bbox_height: float


@dataclass(frozen=True)
class DatasetImageSample:
    filename: str
    mime_type: str
    width: int
    height: int
    image_bytes: bytes
    annotations: tuple[DatasetAnnotation, ...]


@dataclass(frozen=True)
class DatasetIngestionResult:
    dataset_root: Path
    manifest_path: Path
    sample_count: int
    annotation_count: int
    classes: tuple[str, ...]
