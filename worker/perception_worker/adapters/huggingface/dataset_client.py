"""Hugging Face dataset client adapter."""

from collections.abc import Iterable, Mapping, Sequence
from io import BytesIO
from typing import Protocol, cast

from perception_worker.domain.dataset_ingestion import (
    DatasetAnnotation,
    DatasetImageSample,
    DatasetIngestionError,
)

NORMALIZED_BBOX_EDGE_EPSILON = 0.000001


class HuggingFaceLoader(Protocol):
    def __call__(
        self,
        source_dataset: str,
        *,
        split: str,
        token: str | None,
    ) -> Iterable[Mapping[str, object]]:
        """Load rows from a Hugging Face dataset."""
        ...


class HuggingFaceDatasetClient:
    def __init__(
        self,
        token: str | None,
        loader: HuggingFaceLoader | None = None,
    ) -> None:
        self._token = token
        self._loader = loader or load_huggingface_dataset

    def read_samples(
        self,
        source_dataset: str,
        split: str,
        max_samples: int | None,
        classes: tuple[str, ...],
    ) -> tuple[DatasetImageSample, ...]:
        try:
            rows = self._loader(source_dataset, split=split, token=self._token)
            samples = []
            for index, row in enumerate(rows):
                if max_samples is not None and index >= max_samples:
                    break
                samples.append(
                    row_to_sample(
                        row=row,
                        source_dataset=source_dataset,
                        index=index,
                        classes=classes,
                    )
                )
        except Exception:
            raise DatasetIngestionError("failed to load Hugging Face dataset") from None

        return tuple(samples)


def load_huggingface_dataset(
    source_dataset: str,
    *,
    split: str,
    token: str | None,
) -> Iterable[Mapping[str, object]]:
    from datasets import load_dataset  # type: ignore[import-untyped]

    dataset = load_dataset(source_dataset, split=split, token=token)

    return cast(Iterable[Mapping[str, object]], dataset)


def row_to_sample(
    row: Mapping[str, object],
    source_dataset: str,
    index: int,
    classes: tuple[str, ...],
) -> DatasetImageSample:
    image = row.get("image")

    if image is None:
        raise DatasetIngestionError("Hugging Face row is missing image")

    width = int(getattr(image, "width"))
    height = int(getattr(image, "height"))
    image_format = normalized_image_format(image)
    extension = "jpg" if image_format == "JPEG" else image_format.lower()
    image_bytes = encode_image(image=image, image_format=image_format)
    annotations = annotations_from_row(row=row, width=width, height=height, classes=classes)

    return DatasetImageSample(
        filename=f"{safe_dataset_name(source_dataset)}_{index + 1:06d}.{extension}",
        mime_type=f"image/{'jpeg' if extension == 'jpg' else extension}",
        width=width,
        height=height,
        image_bytes=image_bytes,
        annotations=annotations,
    )


def encode_image(image: object, image_format: str) -> bytes:
    buffer = BytesIO()
    image_to_save = image
    if image_format == "JPEG" and getattr(image, "mode", None) not in (None, "RGB", "L"):
        convert = getattr(image, "convert", None)
        if callable(convert):
            image_to_save = convert("RGB")

    save = getattr(image_to_save, "save")
    save(buffer, format=image_format)
    return buffer.getvalue()


def normalized_image_format(image: object) -> str:
    image_format = str(getattr(image, "format", None) or "JPEG").upper()
    if image_format == "JPG":
        return "JPEG"
    if image_format not in {"JPEG", "PNG", "WEBP"}:
        return "JPEG"
    return image_format


def annotations_from_row(
    row: Mapping[str, object],
    width: int,
    height: int,
    classes: tuple[str, ...],
) -> tuple[DatasetAnnotation, ...]:
    objects = row.get("objects")

    if not isinstance(objects, Mapping):
        return ()

    bboxes = objects.get("bbox")
    categories = objects.get("category")

    if not isinstance(bboxes, Sequence) or not isinstance(categories, Sequence):
        return ()

    annotations: list[DatasetAnnotation] = []

    for bbox, category in zip(bboxes, categories, strict=False):
        if not isinstance(bbox, Sequence) or isinstance(bbox, str):
            continue
        class_index = int(category)
        class_name = classes[class_index]
        x, y, bbox_width, bbox_height = [float(value) for value in bbox[:4]]
        bbox_x, bbox_y, normalized_width, normalized_height = normalized_bbox_for_api(
            x=x,
            y=y,
            bbox_width=bbox_width,
            bbox_height=bbox_height,
            image_width=width,
            image_height=height,
        )
        annotations.append(
            DatasetAnnotation(
                class_name=class_name,
                bbox_x=bbox_x,
                bbox_y=bbox_y,
                bbox_width=normalized_width,
                bbox_height=normalized_height,
            )
        )

    return tuple(annotations)


def normalized_bbox_for_api(
    x: float,
    y: float,
    bbox_width: float,
    bbox_height: float,
    image_width: int,
    image_height: int,
) -> tuple[float, float, float, float]:
    bbox_x = bounded_origin(x / image_width)
    bbox_y = bounded_origin(y / image_height)

    return (
        bbox_x,
        bbox_y,
        bounded_extent(origin=bbox_x, extent=bbox_width / image_width),
        bounded_extent(origin=bbox_y, extent=bbox_height / image_height),
    )


def bounded_origin(value: float) -> float:
    return min(max(value, 0.0), 1.0 - NORMALIZED_BBOX_EDGE_EPSILON)


def bounded_extent(origin: float, extent: float) -> float:
    max_extent = max(NORMALIZED_BBOX_EDGE_EPSILON, 1.0 - origin - NORMALIZED_BBOX_EDGE_EPSILON)
    return min(max(extent, NORMALIZED_BBOX_EDGE_EPSILON), max_extent)


def safe_dataset_name(source_dataset: str) -> str:
    return source_dataset.replace("/", "_").replace(" ", "_")
