"""Domain records for materialized YOLO training datasets."""

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class YoloDatasetAnnotation:
    class_id: int
    class_name: str
    bbox_x: float
    bbox_y: float
    bbox_width: float
    bbox_height: float


@dataclass(frozen=True)
class YoloDatasetSample:
    sample_id: str
    source_path: Path
    filename: str
    split_name: str
    annotations: tuple[YoloDatasetAnnotation, ...]


@dataclass(frozen=True)
class YoloDatasetSnapshot:
    dataset_version_id: str
    classes: tuple[str, ...]
    samples: tuple[YoloDatasetSample, ...]


@dataclass(frozen=True)
class MaterializedYoloDataset:
    root: Path
    data_yaml_path: Path
    sample_count: int
    annotation_count: int
