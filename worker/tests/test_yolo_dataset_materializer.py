from pathlib import Path

from PIL import Image

from perception_worker.adapters.training.yolo_dataset_materializer import LocalYoloDatasetWriter
from perception_worker.domain.yolo_dataset import (
    YoloDatasetAnnotation,
    YoloDatasetSample,
    YoloDatasetSnapshot,
)


def test_local_yolo_dataset_writer_materializes_ultralytics_layout(tmp_path: Path) -> None:
    source_image = tmp_path / "source-phone.png"
    Image.new("RGB", (20, 10), color=(255, 255, 255)).save(source_image)
    snapshot = YoloDatasetSnapshot(
        dataset_version_id="dsv_001",
        classes=("phone", "person"),
        samples=(
            YoloDatasetSample(
                sample_id="sample_001",
                source_path=source_image,
                filename="phone.png",
                split_name="train",
                annotations=(
                    YoloDatasetAnnotation(
                        class_id=0,
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
    writer = LocalYoloDatasetWriter(root=tmp_path / "materialized")

    result = writer.write(job_id="job_001", snapshot=snapshot)

    assert result.data_yaml_path == (
        tmp_path / "materialized" / "job_001" / "dataset" / "data.yaml"
    )
    assert result.sample_count == 1
    assert result.annotation_count == 1
    assert (
        result.root / "images" / "train" / "phone.png"
    ).read_bytes() == source_image.read_bytes()
    assert (result.root / "labels" / "train" / "phone.txt").read_text(encoding="utf-8") == (
        "0 0.250000 0.400000 0.300000 0.400000\n"
    )
    assert result.data_yaml_path.read_text(encoding="utf-8") == (
        f"path: {result.root}\n"
        "train: images/train\n"
        "val: images/train\n"
        "test: images/test\n"
        "names:\n"
        '  0: "phone"\n'
        '  1: "person"\n'
    )


def test_local_yolo_dataset_writer_uses_validation_split_when_present(tmp_path: Path) -> None:
    source_image = tmp_path / "source-phone.png"
    Image.new("RGB", (20, 10), color=(255, 255, 255)).save(source_image)
    snapshot = YoloDatasetSnapshot(
        dataset_version_id="dsv_001",
        classes=("phone",),
        samples=(
            YoloDatasetSample(
                sample_id="sample_001",
                source_path=source_image,
                filename="phone-train.png",
                split_name="train",
                annotations=(),
            ),
            YoloDatasetSample(
                sample_id="sample_002",
                source_path=source_image,
                filename="phone-val.png",
                split_name="validation",
                annotations=(),
            ),
        ),
    )
    writer = LocalYoloDatasetWriter(root=tmp_path / "materialized")

    result = writer.write(job_id="job_001", snapshot=snapshot)

    assert "val: images/validation\n" in result.data_yaml_path.read_text(encoding="utf-8")
