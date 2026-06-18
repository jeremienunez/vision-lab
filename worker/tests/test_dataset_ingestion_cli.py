from pathlib import Path

import pytest
from PIL import Image
from typer.testing import CliRunner

from perception_worker.entrypoints.cli import app


def test_ingest_hf_cli_rejects_missing_token(
    monkeypatch: pytest.MonkeyPatch,
    tmp_path: Path,
) -> None:
    monkeypatch.delenv("HF_TOKEN", raising=False)
    monkeypatch.setenv("PERCEPTIONLAB_DATA_ROOT", str(tmp_path))
    runner = CliRunner()

    result = runner.invoke(
        app,
        [
            "ingest-hf",
            "owner/desk-objects",
            "--target-name",
            "desk-objects-hf",
            "--classes",
            "cup,book",
        ],
    )

    assert result.exit_code == 1
    assert "HF_TOKEN is required" in result.output


def test_parse_classes_trims_empty_values() -> None:
    from perception_worker.entrypoints.cli import parse_classes

    assert parse_classes(" cup, book ,,phone ") == ("cup", "book", "phone")


def test_ingest_yolo_cli_materializes_local_export(
    monkeypatch: pytest.MonkeyPatch,
    tmp_path: Path,
) -> None:
    raw_dataset_root = tmp_path / "raw-yolo"
    image_path = raw_dataset_root / "images" / "train" / "phone.jpg"
    label_path = raw_dataset_root / "labels" / "train" / "phone.txt"
    image_path.parent.mkdir(parents=True)
    label_path.parent.mkdir(parents=True)
    Image.new("RGB", (100, 50), color=(255, 255, 255)).save(image_path)
    label_path.write_text("0 0.500000 0.500000 0.400000 0.600000\n", encoding="utf-8")
    (raw_dataset_root / "data.yaml").write_text(
        "\n".join(
            [
                "path: .",
                "train: images/train",
                "names:",
                "  0: phone",
                "  1: remote",
                "",
            ]
        ),
        encoding="utf-8",
    )
    monkeypatch.setenv("PERCEPTIONLAB_DATA_ROOT", str(tmp_path))
    runner = CliRunner()

    result = runner.invoke(
        app,
        [
            "ingest-yolo",
            str(raw_dataset_root),
            "--target-name",
            "phone-remote-mix",
            "--classes",
            "phone,remote",
            "--split",
            "train",
            "--max-samples",
            "1",
        ],
    )

    assert result.exit_code == 0
    assert "ingested 1 sample(s), 1 annotation(s)" in result.output
    assert (tmp_path / "phone-remote-mix" / "manifest.json").exists()
