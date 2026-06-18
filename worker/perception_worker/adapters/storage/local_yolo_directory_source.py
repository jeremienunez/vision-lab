"""Read local Ultralytics YOLO directory exports as dataset ingestion samples."""

from collections.abc import Mapping, Sequence
from pathlib import Path

from PIL import Image
import yaml  # type: ignore[import-untyped]

from perception_worker.domain.dataset_ingestion import (
    DatasetAnnotation,
    DatasetImageSample,
    DatasetIngestionError,
)

NORMALIZED_BBOX_EDGE_EPSILON = 0.000001
SUPPORTED_IMAGE_SUFFIXES = {".jpg", ".jpeg", ".png", ".webp"}


class LocalYoloDirectoryDatasetSource:
    def read_samples(
        self,
        source_dataset: str,
        split: str,
        max_samples: int | None,
        classes: tuple[str, ...],
    ) -> tuple[DatasetImageSample, ...]:
        data_yaml_path = data_yaml_path_from_source(source_dataset)
        config = read_yolo_data_yaml(data_yaml_path)
        dataset_root = dataset_root_from_config(data_yaml_path=data_yaml_path, config=config)
        image_root = split_image_root(dataset_root=dataset_root, config=config, split=split)
        class_names_by_id = class_names_from_config(config)
        requested_classes = set(classes)
        image_paths = sorted(
            path
            for path in image_root.rglob("*")
            if path.is_file() and path.suffix.lower() in SUPPORTED_IMAGE_SUFFIXES
        )
        if max_samples is not None:
            image_paths = image_paths[:max_samples]

        return tuple(
            sample_from_image(
                image_path=image_path,
                label_path=label_path_for_image(image_path),
                class_names_by_id=class_names_by_id,
                requested_classes=requested_classes,
            )
            for image_path in image_paths
        )


def data_yaml_path_from_source(source_dataset: str) -> Path:
    source_path = Path(source_dataset).expanduser()
    if source_path.is_dir():
        source_path = source_path / "data.yaml"
    if not source_path.exists():
        raise DatasetIngestionError(f"YOLO data.yaml does not exist: {source_path}")
    if source_path.name not in {"data.yaml", "dataset.yaml"}:
        raise DatasetIngestionError(f"YOLO source must be a dataset directory or data.yaml: {source_path}")

    return source_path


def read_yolo_data_yaml(data_yaml_path: Path) -> Mapping[str, object]:
    loaded = yaml.safe_load(data_yaml_path.read_text(encoding="utf-8"))
    if not isinstance(loaded, Mapping):
        raise DatasetIngestionError(f"YOLO data.yaml must contain a mapping: {data_yaml_path}")

    return loaded


def dataset_root_from_config(
    *,
    data_yaml_path: Path,
    config: Mapping[str, object],
) -> Path:
    path_value = config.get("path")
    if not isinstance(path_value, str) or not path_value.strip():
        return data_yaml_path.parent

    configured_path = Path(path_value).expanduser()
    if configured_path.is_absolute():
        return configured_path

    return (data_yaml_path.parent / configured_path).resolve()


def split_image_root(
    *,
    dataset_root: Path,
    config: Mapping[str, object],
    split: str,
) -> Path:
    split_keys = yolo_split_keys(split)
    split_value = next(
        (
            value
            for split_key in split_keys
            if isinstance((value := config.get(split_key)), str) and value.strip()
        ),
        None,
    )
    if not isinstance(split_value, str) or not split_value.strip():
        raise DatasetIngestionError(f"YOLO data.yaml is missing split path: {split_keys[0]}")

    image_root = Path(split_value).expanduser()
    if not image_root.is_absolute():
        image_root = dataset_root / image_root
    if not image_root.exists() or not image_root.is_dir():
        raise DatasetIngestionError(f"YOLO image split directory does not exist: {image_root}")

    return image_root


def yolo_split_keys(split: str) -> tuple[str, ...]:
    normalized = split.strip().lower()
    if normalized in {"validation", "valid", "val"}:
        return ("val", "valid")
    if normalized in {"train", "test"}:
        return (normalized,)
    raise DatasetIngestionError(f"unsupported YOLO split: {split}")


def class_names_from_config(config: Mapping[str, object]) -> dict[int, str]:
    names = config.get("names")
    if isinstance(names, Mapping):
        return {
            int(class_id): str(class_name)
            for class_id, class_name in names.items()
        }
    if isinstance(names, Sequence) and not isinstance(names, str):
        return {
            class_id: str(class_name)
            for class_id, class_name in enumerate(names)
        }

    raise DatasetIngestionError("YOLO data.yaml must define names as a list or mapping")


def sample_from_image(
    *,
    image_path: Path,
    label_path: Path,
    class_names_by_id: dict[int, str],
    requested_classes: set[str],
) -> DatasetImageSample:
    with Image.open(image_path) as image:
        width = int(image.width)
        height = int(image.height)
        image_format = normalized_image_format(image_path=image_path, image=image)

    return DatasetImageSample(
        filename=image_path.name,
        mime_type=mime_type_for_image_format(image_format),
        width=width,
        height=height,
        image_bytes=image_path.read_bytes(),
        annotations=tuple(
            annotations_from_label(
                label_path=label_path,
                class_names_by_id=class_names_by_id,
                requested_classes=requested_classes,
            )
        ),
    )


def label_path_for_image(image_path: Path) -> Path:
    parts = list(image_path.parts)
    for index, part in enumerate(parts):
        if part == "images":
            parts[index] = "labels"
            return Path(*parts).with_suffix(".txt")

    raise DatasetIngestionError(f"YOLO image path is not under an images directory: {image_path}")


def annotations_from_label(
    *,
    label_path: Path,
    class_names_by_id: dict[int, str],
    requested_classes: set[str],
) -> list[DatasetAnnotation]:
    if not label_path.exists():
        return []

    annotations: list[DatasetAnnotation] = []
    for line_number, line in enumerate(label_path.read_text(encoding="utf-8").splitlines(), start=1):
        stripped_line = line.strip()
        if not stripped_line:
            continue
        parts = stripped_line.split()
        if len(parts) < 5:
            raise DatasetIngestionError(f"invalid YOLO label line {line_number}: {label_path}")

        class_id = int(parts[0])
        class_name = class_names_by_id.get(class_id)
        if class_name is None:
            raise DatasetIngestionError(f"YOLO class id {class_id} is missing from data.yaml names")
        if class_name not in requested_classes:
            continue

        center_x, center_y, bbox_width, bbox_height = [float(value) for value in parts[1:5]]
        annotations.append(
            annotation_from_yolo_box(
                class_name=class_name,
                center_x=center_x,
                center_y=center_y,
                bbox_width=bbox_width,
                bbox_height=bbox_height,
            )
        )

    return annotations


def annotation_from_yolo_box(
    *,
    class_name: str,
    center_x: float,
    center_y: float,
    bbox_width: float,
    bbox_height: float,
) -> DatasetAnnotation:
    width = bounded_size(bbox_width)
    height = bounded_size(bbox_height)
    bbox_x = bounded_origin(center_x - width / 2)
    bbox_y = bounded_origin(center_y - height / 2)

    return DatasetAnnotation(
        class_name=class_name,
        bbox_x=bbox_x,
        bbox_y=bbox_y,
        bbox_width=bounded_extent(origin=bbox_x, extent=width),
        bbox_height=bounded_extent(origin=bbox_y, extent=height),
    )


def normalized_image_format(*, image_path: Path, image: Image.Image) -> str:
    image_format = str(image.format or "").upper()
    if image_format == "JPG":
        return "JPEG"
    if image_format in {"JPEG", "PNG", "WEBP"}:
        return image_format

    suffix = image_path.suffix.lower()
    if suffix in {".jpg", ".jpeg"}:
        return "JPEG"
    if suffix == ".png":
        return "PNG"
    if suffix == ".webp":
        return "WEBP"

    raise DatasetIngestionError(f"unsupported image format: {image_path}")


def mime_type_for_image_format(image_format: str) -> str:
    if image_format == "JPEG":
        return "image/jpeg"
    if image_format == "PNG":
        return "image/png"
    if image_format == "WEBP":
        return "image/webp"

    raise DatasetIngestionError(f"unsupported image format: {image_format}")


def bounded_size(value: float) -> float:
    return min(max(value, NORMALIZED_BBOX_EDGE_EPSILON), 1.0 - NORMALIZED_BBOX_EDGE_EPSILON)


def bounded_origin(value: float) -> float:
    return min(max(value, 0.0), 1.0 - NORMALIZED_BBOX_EDGE_EPSILON)


def bounded_extent(*, origin: float, extent: float) -> float:
    max_extent = max(NORMALIZED_BBOX_EDGE_EPSILON, 1.0 - origin - NORMALIZED_BBOX_EDGE_EPSILON)
    return min(max(extent, NORMALIZED_BBOX_EDGE_EPSILON), max_extent)
