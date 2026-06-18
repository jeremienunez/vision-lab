from pathlib import Path

from perception_worker.adapters.training.yolo_finetune_trainer import YoloFineTuneTrainer
from perception_worker.contracts.training_job import (
    TrainingHyperparametersPayload,
    TrainingJobPayload,
)
from perception_worker.domain.training_result import TrainingResult
from perception_worker.domain.yolo_dataset import MaterializedYoloDataset


class RecordingDatasetMaterializer:
    def __init__(self, data_yaml_path: Path) -> None:
        self.data_yaml_path = data_yaml_path
        self.jobs: list[str] = []

    def materialize(self, job: TrainingJobPayload) -> MaterializedYoloDataset:
        self.jobs.append(job.job_id)
        self.data_yaml_path.parent.mkdir(parents=True, exist_ok=True)
        self.data_yaml_path.write_text("path: dataset\n", encoding="utf-8")
        return MaterializedYoloDataset(
            root=self.data_yaml_path.parent,
            data_yaml_path=self.data_yaml_path,
            sample_count=4,
            annotation_count=6,
        )


class RecordingYoloModel:
    def __init__(self) -> None:
        self.train_calls: list[dict[str, object]] = []

    def train(self, **kwargs: object) -> object:
        self.train_calls.append(kwargs)
        weights_root = Path(str(kwargs["project"])) / str(kwargs["name"]) / "weights"
        weights_root.mkdir(parents=True, exist_ok=True)
        (weights_root / "best.pt").write_bytes(b"fine-tuned-yolo")
        return type("TrainingSummary", (), {"results_dict": {"metrics/mAP50(B)": 0.73}})()


def test_yolo_finetune_trainer_runs_ultralytics_training_and_returns_best_weight(
    tmp_path: Path,
) -> None:
    model = RecordingYoloModel()
    loaded_model_paths: list[str] = []
    base_model = tmp_path / "yolo11n.pt"
    base_model.write_bytes(b"base")
    materializer = RecordingDatasetMaterializer(tmp_path / "dataset" / "data.yaml")

    def load_model(model_path: str) -> RecordingYoloModel:
        loaded_model_paths.append(model_path)
        return model

    trainer = YoloFineTuneTrainer(
        artifact_root=tmp_path / "artifacts",
        dataset_materializer=materializer,
        model_loader=load_model,
    )
    job = TrainingJobPayload(
        job_id="job_001",
        dataset_version_id="dsv_001",
        model_family="yolo_finetune",
        base_model=str(base_model),
        hyperparameters=TrainingHyperparametersPayload(
            epochs=3,
            batch_size=2,
            image_size=320,
            learning_rate=0.001,
        ),
        classes=("phone", "person"),
    )

    result = trainer.train(job)

    assert isinstance(result, TrainingResult)
    assert loaded_model_paths == [str(base_model)]
    assert materializer.jobs == ["job_001"]
    assert model.train_calls == [
        {
            "data": str(tmp_path / "dataset" / "data.yaml"),
            "epochs": 3,
            "batch": 2,
            "imgsz": 320,
            "lr0": 0.001,
            "project": str(tmp_path / "artifacts" / "job_001"),
            "name": "train",
            "exist_ok": True,
            "verbose": False,
        }
    ]
    assert result.artifact_uri == (
        tmp_path / "artifacts" / "job_001" / "train" / "weights" / "best.pt"
    ).as_uri()
    assert result.metrics == {
        "epochs": 3.0,
        "sample_count": 4.0,
        "annotation_count": 6.0,
        "mAP50": 0.73,
    }
