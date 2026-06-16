from pathlib import Path

import pytest
from PIL import Image

from perception_worker.adapters.huggingface.dataset_client import (
    HuggingFaceDatasetClient,
    row_to_sample,
)
from perception_worker.adapters.storage.local_dataset_ingestion_store import (
    LocalDatasetIngestionStore,
)
from perception_worker.app.ingest_dataset import DatasetIngestionService
from perception_worker.contracts.dataset_ingestion import DatasetIngestionCommand
from perception_worker.domain.dataset_ingestion import (
    DatasetAnnotation,
    DatasetImageSample,
    DatasetIngestionError,
)


class FakeDatasetSource:
    def __init__(self, samples: tuple[DatasetImageSample, ...]) -> None:
        self.samples = samples
        self.calls: list[tuple[str, str, int | None, tuple[str, ...]]] = []

    def read_samples(
        self,
        source_dataset: str,
        split: str,
        max_samples: int | None,
        classes: tuple[str, ...],
    ) -> tuple[DatasetImageSample, ...]:
        self.calls.append((source_dataset, split, max_samples, classes))
        return self.samples


def test_ingests_hf_like_samples_to_local_yolo_layout(tmp_path: Path) -> None:
    source = FakeDatasetSource(
        (
            DatasetImageSample(
                filename="cup.jpg",
                mime_type="image/jpeg",
                width=100,
                height=100,
                image_bytes=b"fake-jpeg-bytes",
                annotations=(
                    DatasetAnnotation(
                        class_name="cup",
                        bbox_x=0.10,
                        bbox_y=0.20,
                        bbox_width=0.30,
                        bbox_height=0.40,
                    ),
                ),
            ),
        ),
    )
    store = LocalDatasetIngestionStore(root=tmp_path)
    service = DatasetIngestionService(source=source, store=store)

    result = service.ingest(
        DatasetIngestionCommand(
            source_dataset="owner/desk-objects",
            split="train",
            target_name="desk-objects-hf",
            classes=("cup", "book"),
            max_samples=1,
        )
    )

    dataset_root = tmp_path / "desk-objects-hf"
    assert source.calls == [("owner/desk-objects", "train", 1, ("cup", "book"))]
    assert result.dataset_root == dataset_root
    assert result.sample_count == 1
    assert result.annotation_count == 1
    assert result.classes == ("cup", "book")
    assert (dataset_root / "images" / "cup.jpg").read_bytes() == b"fake-jpeg-bytes"
    assert (dataset_root / "labels" / "cup.txt").read_text(encoding="utf-8") == (
        "0 0.250000 0.400000 0.300000 0.400000\n"
    )
    assert result.manifest_path == dataset_root / "manifest.json"
    manifest = result.manifest_path.read_text(encoding="utf-8")
    assert '"source_dataset": "owner/desk-objects"' in manifest
    assert '"task_type": "object_detection"' in manifest
    assert '"version_name": "v1"' in manifest
    assert '"path": "images/cup.jpg"' in manifest
    assert '"yolo_label_path": "labels/cup.txt"' in manifest
    assert '"bbox": {' in manifest
    assert '"x": 0.1' in manifest


def test_ingestion_rejects_unknown_annotation_class(tmp_path: Path) -> None:
    source = FakeDatasetSource(
        (
            DatasetImageSample(
                filename="phone.jpg",
                mime_type="image/jpeg",
                width=100,
                height=100,
                image_bytes=b"fake-jpeg-bytes",
                annotations=(
                    DatasetAnnotation(
                        class_name="phone",
                        bbox_x=0.10,
                        bbox_y=0.20,
                        bbox_width=0.30,
                        bbox_height=0.40,
                    ),
                ),
            ),
        ),
    )
    service = DatasetIngestionService(
        source=source,
        store=LocalDatasetIngestionStore(root=tmp_path),
    )

    with pytest.raises(DatasetIngestionError, match="annotation class is not declared"):
        service.ingest(
            DatasetIngestionCommand(
                source_dataset="owner/desk-objects",
                split="train",
                target_name="desk-objects-hf",
                classes=("cup", "book"),
                max_samples=1,
            )
        )


def test_huggingface_dataset_client_maps_object_detection_rows_without_leaking_token() -> None:
    captured_token: list[str | None] = []

    def fake_loader(
        source_dataset: str,
        *,
        split: str,
        token: str | None,
    ) -> list[dict[str, object]]:
        captured_token.append(token)
        return [
            {
                "image": FakeImage(width=100, height=200, image_bytes=b"fake-png-bytes"),
                "objects": {
                    "bbox": [[10, 20, 30, 40]],
                    "category": [0],
                },
            }
        ]

    client = HuggingFaceDatasetClient(token="hf_secret_token", loader=fake_loader)

    samples = tuple(
        client.read_samples(
            source_dataset="owner/desk-objects",
            split="train",
            max_samples=1,
            classes=("cup", "book"),
        )
    )

    assert captured_token == ["hf_secret_token"]
    assert samples == (
        DatasetImageSample(
            filename="owner_desk-objects_000001.png",
            mime_type="image/png",
            width=100,
            height=200,
            image_bytes=b"fake-png-bytes",
            annotations=(
                DatasetAnnotation(
                    class_name="cup",
                    bbox_x=0.10,
                    bbox_y=0.10,
                    bbox_width=0.30,
                    bbox_height=0.20,
                ),
            ),
        ),
    )


def test_huggingface_dataset_client_redacts_token_from_loader_errors() -> None:
    def failing_loader(
        source_dataset: str,
        *,
        split: str,
        token: str | None,
    ) -> list[dict[str, object]]:
        raise RuntimeError(f"network failure with token {token}")

    client = HuggingFaceDatasetClient(token="hf_secret_token", loader=failing_loader)

    with pytest.raises(DatasetIngestionError) as error:
        tuple(
            client.read_samples(
                source_dataset="owner/private-dataset",
                split="train",
                max_samples=1,
                classes=("cup",),
            )
        )

    assert "hf_secret_token" not in str(error.value)
    assert str(error.value) == "failed to load Hugging Face dataset"
    assert error.value.__cause__ is None


def test_huggingface_row_to_sample_encodes_formatless_cmyk_images_as_jpeg() -> None:
    image = Image.new("CMYK", (100, 100))

    sample = row_to_sample(
        row={
            "image": image,
            "objects": {
                "bbox": [[10, 20, 30, 40]],
                "category": [0],
            },
        },
        source_dataset="owner/cmyk-dataset",
        index=0,
        classes=("Mask",),
    )

    assert sample.filename == "owner_cmyk-dataset_000001.jpg"
    assert sample.mime_type == "image/jpeg"
    assert sample.image_bytes.startswith(b"\xff\xd8")


def test_huggingface_row_to_sample_contracts_edge_touching_boxes_for_api_f32() -> None:
    image = Image.new("RGB", (900, 450))

    sample = row_to_sample(
        row={
            "image": image,
            "objects": {
                "bbox": [[518, 19, 121, 431]],
                "category": [0],
            },
        },
        source_dataset="owner/edge-dataset",
        index=0,
        classes=("Coverall",),
    )

    annotation = sample.annotations[0]
    assert annotation.bbox_y + annotation.bbox_height < 1.0
    assert annotation.bbox_height < 431 / 450


class FakeImage:
    format = "PNG"

    def __init__(self, width: int, height: int, image_bytes: bytes) -> None:
        self.width = width
        self.height = height
        self._image_bytes = image_bytes

    def save(self, buffer: object, format: str) -> None:
        buffer.write(self._image_bytes)  # type: ignore[attr-defined]
