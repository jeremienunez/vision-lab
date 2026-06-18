from pathlib import Path

import pytest
from PIL import Image

from perception_worker.adapters.storage.local_yolo_directory_source import (
    LocalYoloDirectoryDatasetSource,
)


def test_yolo_directory_source_reads_yolo_export_boxes(tmp_path: Path) -> None:
    dataset_root = tmp_path / "raw-yolo"
    image_path = dataset_root / "images" / "train" / "phone.jpg"
    label_path = dataset_root / "labels" / "train" / "phone.txt"
    image_path.parent.mkdir(parents=True)
    label_path.parent.mkdir(parents=True)
    Image.new("RGB", (100, 50), color=(255, 255, 255)).save(image_path)
    label_path.write_text("0 0.500000 0.500000 0.400000 0.600000\n", encoding="utf-8")
    (dataset_root / "data.yaml").write_text(
        "\n".join(
            [
                "path: .",
                "train: images/train",
                "val: images/valid",
                "names:",
                "  0: phone",
                "  1: remote",
                "",
            ]
        ),
        encoding="utf-8",
    )
    source = LocalYoloDirectoryDatasetSource()

    samples = source.read_samples(
        source_dataset=str(dataset_root),
        split="train",
        max_samples=1,
        classes=("phone", "remote"),
    )

    assert len(samples) == 1
    assert samples[0].filename == "phone.jpg"
    assert samples[0].mime_type == "image/jpeg"
    assert samples[0].width == 100
    assert samples[0].height == 50
    assert samples[0].image_bytes == image_path.read_bytes()
    assert len(samples[0].annotations) == 1
    annotation = samples[0].annotations[0]
    assert annotation.class_name == "phone"
    assert annotation.bbox_x == pytest.approx(0.30)
    assert annotation.bbox_y == pytest.approx(0.20)
    assert annotation.bbox_width == pytest.approx(0.40)
    assert annotation.bbox_height == pytest.approx(0.60)


def test_yolo_directory_source_filters_classes_and_keeps_hard_negatives(
    tmp_path: Path,
) -> None:
    dataset_root = tmp_path / "raw-yolo"
    image_root = dataset_root / "valid" / "images"
    label_root = dataset_root / "valid" / "labels"
    image_root.mkdir(parents=True)
    label_root.mkdir(parents=True)
    Image.new("RGB", (80, 80), color=(255, 255, 255)).save(image_root / "remote.png")
    Image.new("RGB", (80, 80), color=(0, 0, 0)).save(image_root / "background.png")
    (label_root / "remote.txt").write_text(
        "\n".join(
            [
                "1 0.500000 0.500000 0.500000 0.500000",
                "2 0.500000 0.500000 0.500000 0.500000",
                "",
            ]
        ),
        encoding="utf-8",
    )
    (label_root / "background.txt").write_text("", encoding="utf-8")
    (dataset_root / "data.yaml").write_text(
        "\n".join(
            [
                "path: .",
                "valid: valid/images",
                "names: [phone, remote, laptop]",
                "",
            ]
        ),
        encoding="utf-8",
    )
    source = LocalYoloDirectoryDatasetSource()

    samples = source.read_samples(
        source_dataset=str(dataset_root / "data.yaml"),
        split="validation",
        max_samples=None,
        classes=("phone", "remote"),
    )

    assert [sample.filename for sample in samples] == ["background.png", "remote.png"]
    assert samples[0].annotations == ()
    assert [annotation.class_name for annotation in samples[1].annotations] == ["remote"]
