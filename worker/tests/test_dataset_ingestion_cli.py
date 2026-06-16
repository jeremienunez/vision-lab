from pathlib import Path

import pytest
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
