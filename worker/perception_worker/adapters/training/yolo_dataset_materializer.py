"""Materialize immutable dataset versions into Ultralytics YOLO layout."""

import json
import shutil
import uuid
from pathlib import Path
from typing import Any

import psycopg

from perception_worker.contracts.training_job import TrainingJobPayload
from perception_worker.domain.yolo_dataset import (
    MaterializedYoloDataset,
    YoloDatasetAnnotation,
    YoloDatasetSample,
    YoloDatasetSnapshot,
)

YOLO_SPLITS = ("train", "validation", "test")


class LocalYoloDatasetWriter:
    def __init__(self, root: Path) -> None:
        self._root = root

    def write(self, *, job_id: str, snapshot: YoloDatasetSnapshot) -> MaterializedYoloDataset:
        dataset_root = self._root / job_id / "dataset"
        if dataset_root.exists():
            shutil.rmtree(dataset_root)

        for split_name in YOLO_SPLITS:
            (dataset_root / "images" / split_name).mkdir(parents=True, exist_ok=True)
            (dataset_root / "labels" / split_name).mkdir(parents=True, exist_ok=True)

        annotation_count = 0
        for sample in snapshot.samples:
            split_name = normalized_split_name(sample.split_name)
            filename = safe_filename(sample.filename)
            image_path = dataset_root / "images" / split_name / filename
            label_path = dataset_root / "labels" / split_name / f"{Path(filename).stem}.txt"
            shutil.copyfile(sample.source_path, image_path)
            label_path.write_text(
                "".join(yolo_line(annotation) for annotation in sample.annotations),
                encoding="utf-8",
            )
            annotation_count += len(sample.annotations)

        data_yaml_path = dataset_root / "data.yaml"
        data_yaml_path.write_text(
            data_yaml_content(dataset_root, snapshot.classes),
            encoding="utf-8",
        )

        return MaterializedYoloDataset(
            root=dataset_root,
            data_yaml_path=data_yaml_path,
            sample_count=len(snapshot.samples),
            annotation_count=annotation_count,
        )


class PostgresYoloDatasetMaterializer:
    def __init__(self, *, database_url: str, writer: LocalYoloDatasetWriter) -> None:
        self._database_url = database_url
        self._writer = writer

    def materialize(self, job: TrainingJobPayload) -> MaterializedYoloDataset:
        snapshot = self._load_snapshot(job)

        return self._writer.write(job_id=job.job_id, snapshot=snapshot)

    def _load_snapshot(self, job: TrainingJobPayload) -> YoloDatasetSnapshot:
        version_id = uuid.UUID(job.dataset_version_id)
        with psycopg.connect(self._database_url) as connection:
            version_row = connection.execute(
                """
                SELECT dataset_id, classes_snapshot, split_config
                FROM dataset_versions
                WHERE id = %s
                """,
                (version_id,),
            ).fetchone()
            if version_row is None:
                raise ValueError(f"dataset version does not exist: {job.dataset_version_id}")

            dataset_id = version_row[0]
            classes = tuple(str(class_name) for class_name in version_row[1])
            split_config = dict(version_row[2])
            samples = load_version_samples(
                connection=connection,
                dataset_id=dataset_id,
                version_id=version_id,
                split_config=split_config,
            )

        return YoloDatasetSnapshot(
            dataset_version_id=job.dataset_version_id,
            classes=classes,
            samples=tuple(samples),
        )


def load_version_samples(
    *,
    connection: psycopg.Connection[Any],
    dataset_id: uuid.UUID,
    version_id: uuid.UUID,
    split_config: dict[str, str],
) -> list[YoloDatasetSample]:
    version_sample_count = connection.execute(
        "SELECT count(*) FROM dataset_version_samples WHERE dataset_version_id = %s",
        (version_id,),
    ).fetchone()
    if version_sample_count is not None and version_sample_count[0] > 0:
        rows = connection.execute(
            """
            SELECT samples.id, samples.storage_uri, samples.filename, version_samples.split_name
            FROM dataset_version_samples version_samples
            JOIN samples ON samples.id = version_samples.sample_id
            WHERE version_samples.dataset_version_id = %s
            ORDER BY version_samples.created_at ASC, samples.id ASC
            """,
            (version_id,),
        ).fetchall()
        split_names = [str(row[3]) for row in rows]
    else:
        rows = connection.execute(
            """
            SELECT id, storage_uri, filename
            FROM samples
            WHERE dataset_id = %s
            ORDER BY created_at ASC, id ASC
            """,
            (dataset_id,),
        ).fetchall()
        split_names = fallback_split_names(sample_count=len(rows), split_config=split_config)

    return [
        YoloDatasetSample(
            sample_id=str(row[0]),
            source_path=path_from_storage_uri(str(row[1])),
            filename=str(row[2]),
            split_name=split_names[index],
            annotations=tuple(
                load_sample_annotations(
                    connection=connection,
                    sample_id=row[0],
                    dataset_id=dataset_id,
                )
            ),
        )
        for index, row in enumerate(rows)
    ]


def load_sample_annotations(
    *,
    connection: psycopg.Connection[Any],
    sample_id: uuid.UUID,
    dataset_id: uuid.UUID,
) -> list[YoloDatasetAnnotation]:
    rows = connection.execute(
        """
        SELECT class_id, class_name,
               bbox_x::real, bbox_y::real, bbox_width::real, bbox_height::real
        FROM annotations
        WHERE sample_id = %s AND dataset_id = %s
        ORDER BY created_at ASC, id ASC
        """,
        (sample_id, dataset_id),
    ).fetchall()

    return [
        YoloDatasetAnnotation(
            class_id=int(row[0]),
            class_name=str(row[1]),
            bbox_x=float(row[2]),
            bbox_y=float(row[3]),
            bbox_width=float(row[4]),
            bbox_height=float(row[5]),
        )
        for row in rows
    ]


def fallback_split_names(*, sample_count: int, split_config: dict[str, str]) -> list[str]:
    if sample_count == 0:
        return []
    if not split_config:
        return ["train" for _index in range(sample_count)]

    train_count = split_count(
        sample_count=sample_count,
        percentage=split_config.get("train", "100"),
    )
    validation_count = split_count(
        sample_count=sample_count,
        percentage=split_config.get("validation", "0"),
    )
    split_names = (
        ["train"] * train_count
        + ["validation"] * validation_count
        + ["test"] * sample_count
    )

    return split_names[:sample_count]


def split_count(*, sample_count: int, percentage: str) -> int:
    return int(sample_count * max(0, min(100, int(percentage))) / 100)


def path_from_storage_uri(storage_uri: str) -> Path:
    return Path(storage_uri.removeprefix("file://"))


def normalized_split_name(split_name: str) -> str:
    normalized = split_name.strip().lower()
    if normalized == "val":
        return "validation"
    if normalized not in YOLO_SPLITS:
        raise ValueError(f"unsupported YOLO split: {split_name}")
    return normalized


def safe_filename(filename: str) -> str:
    name = Path(filename).name
    if not name:
        raise ValueError("YOLO sample filename is required")
    return name


def yolo_line(annotation: YoloDatasetAnnotation) -> str:
    center_x = annotation.bbox_x + annotation.bbox_width / 2
    center_y = annotation.bbox_y + annotation.bbox_height / 2

    return (
        f"{annotation.class_id} "
        f"{center_x:.6f} "
        f"{center_y:.6f} "
        f"{annotation.bbox_width:.6f} "
        f"{annotation.bbox_height:.6f}\n"
    )


def data_yaml_content(dataset_root: Path, classes: tuple[str, ...]) -> str:
    names = "".join(
        f"  {index}: {json.dumps(class_name)}\n"
        for index, class_name in enumerate(classes)
    )

    return (
        f"path: {dataset_root}\n"
        "train: images/train\n"
        "val: images/validation\n"
        "test: images/test\n"
        "names:\n"
        f"{names}"
    )
