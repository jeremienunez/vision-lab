"""Command line entrypoint for the PerceptionLab worker."""

import contextlib
import io
import json
import os
import socket
from pathlib import Path
from typing import Annotated

import typer

from perception_worker.adapters.db.postgres_job_repository import PostgresTrainingJobRepository
from perception_worker.adapters.huggingface.dataset_client import HuggingFaceDatasetClient
from perception_worker.adapters.inference.webcam_capture import OpenCvWebcamFrameCapture
from perception_worker.adapters.inference.yolo_object_detector import YoloObjectDetector
from perception_worker.adapters.storage.local_dataset_ingestion_store import (
    LocalDatasetIngestionStore,
)
from perception_worker.adapters.storage.local_yolo_directory_source import (
    LocalYoloDirectoryDatasetSource,
)
from perception_worker.adapters.training.fake_trainer import FakeTrainer
from perception_worker.adapters.training.tiny_torch_trainer import TinyTorchTrainer
from perception_worker.adapters.training.yolo_dataset_materializer import (
    LocalYoloDatasetWriter,
    PostgresYoloDatasetMaterializer,
)
from perception_worker.adapters.training.yolo_finetune_trainer import YoloFineTuneTrainer
from perception_worker.app.ingest_dataset import DatasetIngestionService
from perception_worker.app.process_training_job import TrainingJobProcessor
from perception_worker.app.run_live_webcam_detection import LiveWebcamDetector
from perception_worker.contracts.dataset_ingestion import DatasetIngestionCommand

app = typer.Typer(add_completion=False)


@app.callback(invoke_without_command=True)
def bootstrap(context: typer.Context) -> None:
    if context.invoked_subcommand is None:
        typer.echo("perception-worker bootstrap loaded")


@app.command("ingest-hf")
def ingest_hf(
    source_dataset: str,
    target_name: Annotated[str, typer.Option()],
    classes: Annotated[str, typer.Option()],
    split: Annotated[str, typer.Option()] = "train",
    max_samples: Annotated[int | None, typer.Option()] = None,
) -> None:
    token = os.environ.get("HF_TOKEN")
    if not token:
        typer.echo("HF_TOKEN is required", err=True)
        raise typer.Exit(1)

    data_root = Path(os.environ.get("PERCEPTIONLAB_DATA_ROOT", "datasets")).expanduser()
    service = DatasetIngestionService(
        source=HuggingFaceDatasetClient(token=token),
        store=LocalDatasetIngestionStore(root=data_root),
    )
    result = service.ingest(
        DatasetIngestionCommand(
            source_dataset=source_dataset,
            split=split,
            target_name=target_name,
            classes=parse_classes(classes),
            max_samples=max_samples,
        )
    )

    typer.echo(
        f"ingested {result.sample_count} sample(s), "
        f"{result.annotation_count} annotation(s) into {result.dataset_root}"
    )


@app.command("ingest-yolo")
def ingest_yolo(
    source_dataset: Path,
    target_name: Annotated[str, typer.Option()],
    classes: Annotated[str, typer.Option()],
    split: Annotated[str, typer.Option()] = "train",
    max_samples: Annotated[int | None, typer.Option()] = None,
) -> None:
    data_root = Path(os.environ.get("PERCEPTIONLAB_DATA_ROOT", "datasets")).expanduser()
    service = DatasetIngestionService(
        source=LocalYoloDirectoryDatasetSource(),
        store=LocalDatasetIngestionStore(root=data_root),
    )
    result = service.ingest(
        DatasetIngestionCommand(
            source_dataset=str(source_dataset),
            split=split,
            target_name=target_name,
            classes=parse_classes(classes),
            max_samples=max_samples,
        )
    )

    typer.echo(
        f"ingested {result.sample_count} sample(s), "
        f"{result.annotation_count} annotation(s) into {result.dataset_root}"
    )


@app.command("process-once")
def process_once(
    repository_backend: Annotated[str, typer.Option()] = "postgres",
    trainer_name: Annotated[str, typer.Option("--trainer")] = "tiny_torch",
    worker_id: Annotated[str | None, typer.Option()] = None,
    artifact_root: Annotated[Path | None, typer.Option()] = None,
) -> None:
    if repository_backend != "postgres":
        typer.echo("Only postgres repository backend is supported for process-once", err=True)
        raise typer.Exit(1)

    database_url = os.environ.get("PERCEPTIONLAB_DATABASE_URL")
    if not database_url:
        typer.echo("PERCEPTIONLAB_DATABASE_URL is required", err=True)
        raise typer.Exit(1)

    resolved_worker_id = worker_id or default_worker_id()
    resolved_artifact_root = artifact_root or Path(
        os.environ.get("PERCEPTIONLAB_ARTIFACT_ROOT", ".perceptionlab/artifacts")
    )
    repository = PostgresTrainingJobRepository(
        database_url=database_url,
        worker_id=resolved_worker_id,
    )
    processor = TrainingJobProcessor(
        job_repository=repository,
        trainer=build_trainer(
            trainer_name=trainer_name,
            artifact_root=resolved_artifact_root,
            database_url=database_url,
        ),
    )
    processed = processor.run_once()

    typer.echo(json.dumps({"processed": processed, "worker_id": resolved_worker_id}, indent=2))


@app.command("detect-image")
def detect_image(
    image_path: Annotated[Path, typer.Argument()],
    model_path: Annotated[Path, typer.Option()] = Path(".perceptionlab/models/yolo11n.pt"),
    output_root: Annotated[Path, typer.Option()] = Path(".perceptionlab/real-inference"),
    run_name: Annotated[str, typer.Option()] = "image",
    confidence_threshold: Annotated[float, typer.Option("--confidence-threshold")] = 0.25,
    json_only: Annotated[bool, typer.Option()] = False,
) -> None:
    detector = YoloObjectDetector()
    if json_only:
        with contextlib.redirect_stdout(io.StringIO()):
            result = detector.detect_image(
                image_path=image_path,
                model_path=model_path,
                output_root=output_root,
                run_name=run_name,
                confidence_threshold=confidence_threshold,
            )
    else:
        result = detector.detect_image(
            image_path=image_path,
            model_path=model_path,
            output_root=output_root,
            run_name=run_name,
            confidence_threshold=confidence_threshold,
        )
    typer.echo(json.dumps(result.to_summary(), indent=2))


@app.command("detect-webcam")
def detect_webcam(
    device_index: Annotated[int, typer.Option()] = 0,
    capture_path: Annotated[Path, typer.Option()] = Path(".perceptionlab/captures/webcam.png"),
    model_path: Annotated[Path, typer.Option()] = Path(".perceptionlab/models/yolo11n.pt"),
    output_root: Annotated[Path, typer.Option()] = Path(".perceptionlab/real-inference"),
    run_name: Annotated[str, typer.Option()] = "webcam",
    confidence_threshold: Annotated[float, typer.Option("--confidence-threshold")] = 0.25,
    json_only: Annotated[bool, typer.Option()] = False,
) -> None:
    captured_image_path = OpenCvWebcamFrameCapture().capture_frame(
        device_index=device_index,
        output_path=capture_path,
    )
    detector = YoloObjectDetector()
    if json_only:
        with contextlib.redirect_stdout(io.StringIO()):
            result = detector.detect_image(
                image_path=captured_image_path,
                model_path=model_path,
                output_root=output_root,
                run_name=run_name,
                confidence_threshold=confidence_threshold,
            )
    else:
        result = detector.detect_image(
            image_path=captured_image_path,
            model_path=model_path,
            output_root=output_root,
            run_name=run_name,
            confidence_threshold=confidence_threshold,
        )
    typer.echo(json.dumps(result.to_summary(), indent=2))


@app.command("detect-webcam-live")
def detect_webcam_live(
    device_index: Annotated[int, typer.Option()] = 0,
    capture_root: Annotated[Path, typer.Option()] = Path(".perceptionlab/captures/live"),
    model_path: Annotated[Path, typer.Option()] = Path(".perceptionlab/models/yolo11n.pt"),
    output_root: Annotated[Path, typer.Option()] = Path(".perceptionlab/real-inference/live"),
    run_name: Annotated[str, typer.Option()] = "webcam-live",
    confidence_threshold: Annotated[float, typer.Option("--confidence-threshold")] = 0.25,
    frame_limit: Annotated[int | None, typer.Option()] = None,
    json_only: Annotated[bool, typer.Option()] = False,
) -> None:
    live_detector = LiveWebcamDetector()
    if json_only:
        with contextlib.redirect_stdout(io.StringIO()):
            result = live_detector.run(
                device_index=device_index,
                capture_root=capture_root,
                model_path=model_path,
                output_root=output_root,
                run_name=run_name,
                confidence_threshold=confidence_threshold,
                frame_limit=frame_limit,
            )
    else:
        result = live_detector.run(
            device_index=device_index,
            capture_root=capture_root,
            model_path=model_path,
            output_root=output_root,
            run_name=run_name,
            confidence_threshold=confidence_threshold,
            frame_limit=frame_limit,
        )
    typer.echo(json.dumps(result.to_summary(), indent=2))


def parse_classes(value: str) -> tuple[str, ...]:
    return tuple(class_name.strip() for class_name in value.split(",") if class_name.strip())


def default_worker_id() -> str:
    return f"{socket.gethostname()}-{os.getpid()}"


def build_trainer(
    trainer_name: str,
    artifact_root: Path,
    database_url: str | None = None,
) -> FakeTrainer | TinyTorchTrainer | YoloFineTuneTrainer:
    if trainer_name == "fake":
        return FakeTrainer()
    if trainer_name == "tiny_torch":
        return TinyTorchTrainer(artifact_root=artifact_root / "models")
    if trainer_name == "yolo_finetune":
        if database_url is None:
            raise typer.BadParameter("database_url is required for yolo_finetune")

        return YoloFineTuneTrainer(
            artifact_root=artifact_root / "models",
            dataset_materializer=PostgresYoloDatasetMaterializer(
                database_url=database_url,
                writer=LocalYoloDatasetWriter(root=artifact_root / "datasets"),
            ),
        )
    raise typer.BadParameter("trainer must be fake, tiny_torch, or yolo_finetune")


def main() -> None:
    app()
