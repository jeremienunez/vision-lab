"""Ultralytics YOLO fine-tuning strategy."""

import importlib
from collections.abc import Callable
from pathlib import Path
from typing import Any, Protocol

from perception_worker.contracts.training_job import TrainingJobPayload
from perception_worker.domain.training_result import TrainingResult
from perception_worker.domain.yolo_dataset import MaterializedYoloDataset


class YoloDatasetMaterializer(Protocol):
    def materialize(self, job: TrainingJobPayload) -> MaterializedYoloDataset:
        """Materialize one training job dataset into a YOLO data.yaml layout."""
        ...


class YoloFineTuneTrainer:
    def __init__(
        self,
        *,
        artifact_root: Path,
        dataset_materializer: YoloDatasetMaterializer,
        model_loader: Callable[[str], Any] | None = None,
    ) -> None:
        self._artifact_root = artifact_root
        self._dataset_materializer = dataset_materializer
        self._model_loader = model_loader or load_ultralytics_model

    def train(self, job: TrainingJobPayload) -> TrainingResult:
        materialized_dataset = self._dataset_materializer.materialize(job)
        artifact_root = self._artifact_root / job.job_id
        model = self._model_loader(base_model_path(job))
        training_summary = model.train(
            data=str(materialized_dataset.data_yaml_path),
            epochs=job.hyperparameters.epochs,
            batch=job.hyperparameters.batch_size,
            imgsz=job.hyperparameters.image_size,
            lr0=job.hyperparameters.learning_rate,
            project=str(artifact_root),
            name="train",
            exist_ok=True,
            verbose=False,
        )
        best_weight_path = trained_weight_path(artifact_root)

        return TrainingResult(
            artifact_uri=best_weight_path.resolve().as_uri(),
            metrics=training_metrics(
                job=job,
                materialized_dataset=materialized_dataset,
                training_summary=training_summary,
            ),
        )


def load_ultralytics_model(model_path: str) -> Any:
    ultralytics = importlib.import_module("ultralytics")
    yolo_class = getattr(ultralytics, "YOLO")

    return yolo_class(model_path)


def base_model_path(job: TrainingJobPayload) -> str:
    return job.base_model or ".perceptionlab/models/yolo11n.pt"


def trained_weight_path(artifact_root: Path) -> Path:
    weights_root = artifact_root / "train" / "weights"
    best_weight = weights_root / "best.pt"
    if best_weight.exists():
        return best_weight

    last_weight = weights_root / "last.pt"
    if last_weight.exists():
        return last_weight

    raise FileNotFoundError(f"YOLO training did not produce weights under {weights_root}")


def training_metrics(
    *,
    job: TrainingJobPayload,
    materialized_dataset: MaterializedYoloDataset,
    training_summary: object,
) -> dict[str, float]:
    metrics = {
        "epochs": float(job.hyperparameters.epochs),
        "sample_count": float(materialized_dataset.sample_count),
        "annotation_count": float(materialized_dataset.annotation_count),
    }
    results_dict = getattr(training_summary, "results_dict", {})
    if isinstance(results_dict, dict):
        map50 = results_dict.get("metrics/mAP50(B)")
        if isinstance(map50, int | float):
            metrics["mAP50"] = float(map50)

    return metrics
