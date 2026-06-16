"""Local filesystem store for materialized external datasets."""

import json
from pathlib import Path

from perception_worker.contracts.dataset_ingestion import DatasetIngestionCommand
from perception_worker.domain.dataset_ingestion import (
    DatasetAnnotation,
    DatasetImageSample,
    DatasetIngestionResult,
)


class LocalDatasetIngestionStore:
    def __init__(self, root: Path) -> None:
        self._root = root

    def write_dataset(
        self,
        command: DatasetIngestionCommand,
        samples: tuple[DatasetImageSample, ...],
    ) -> DatasetIngestionResult:
        dataset_root = self._root / command.target_name
        image_root = dataset_root / "images"
        label_root = dataset_root / "labels"
        image_root.mkdir(parents=True, exist_ok=True)
        label_root.mkdir(parents=True, exist_ok=True)

        annotation_count = 0
        class_ids = {class_name: index for index, class_name in enumerate(command.classes)}

        for sample in samples:
            (image_root / sample.filename).write_bytes(sample.image_bytes)
            label_path = label_root / f"{Path(sample.filename).stem}.txt"
            label_content = "".join(
                yolo_line(annotation=annotation, class_ids=class_ids)
                for annotation in sample.annotations
            )
            label_path.write_text(label_content, encoding="utf-8")
            annotation_count += len(sample.annotations)

        manifest_path = dataset_root / "manifest.json"
        manifest_path.write_text(
            json.dumps(
                {
                    "source_dataset": command.source_dataset,
                    "split": command.split,
                    "target_name": command.target_name,
                    "classes": list(command.classes),
                    "sample_count": len(samples),
                    "annotation_count": annotation_count,
                },
                indent=2,
                sort_keys=True,
            )
            + "\n",
            encoding="utf-8",
        )

        return DatasetIngestionResult(
            dataset_root=dataset_root,
            manifest_path=manifest_path,
            sample_count=len(samples),
            annotation_count=annotation_count,
            classes=command.classes,
        )


def yolo_line(annotation: DatasetAnnotation, class_ids: dict[str, int]) -> str:
    class_id = class_ids[annotation.class_name]
    center_x = annotation.bbox_x + annotation.bbox_width / 2
    center_y = annotation.bbox_y + annotation.bbox_height / 2

    return (
        f"{class_id} "
        f"{center_x:.6f} "
        f"{center_y:.6f} "
        f"{annotation.bbox_width:.6f} "
        f"{annotation.bbox_height:.6f}\n"
    )
