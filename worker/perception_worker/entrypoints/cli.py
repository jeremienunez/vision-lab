"""Command line entrypoint for the PerceptionLab worker."""

import os
from pathlib import Path
from typing import Annotated

import typer

from perception_worker.adapters.huggingface.dataset_client import HuggingFaceDatasetClient
from perception_worker.adapters.storage.local_dataset_ingestion_store import (
    LocalDatasetIngestionStore,
)
from perception_worker.app.ingest_dataset import DatasetIngestionService
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


def parse_classes(value: str) -> tuple[str, ...]:
    return tuple(class_name.strip() for class_name in value.split(",") if class_name.strip())


def main() -> None:
    app()
