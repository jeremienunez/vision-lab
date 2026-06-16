from pathlib import Path

import pytest
from typer.testing import CliRunner

from perception_worker.domain.real_inference import (
    RealDetection,
    RealInferenceResult,
)
from perception_worker.entrypoints import cli


class FakeDetector:
    def detect_image(
        self,
        *,
        image_path: Path,
        model_path: Path,
        output_root: Path,
        run_name: str,
        confidence_threshold: float,
    ) -> RealInferenceResult:
        return RealInferenceResult(
            image_path=image_path,
            output_dir=output_root / run_name,
            annotated_image_path=output_root / run_name / "capture.jpg",
            label_path=output_root / run_name / "labels" / "capture.txt",
            detections=(
                RealDetection(
                    class_id=0,
                    class_name="person",
                    confidence=0.87,
                    bbox_xyxy=(10.0, 20.0, 110.0, 220.0),
                ),
            ),
        )


class FakeCapture:
    def capture_frame(self, *, device_index: int, output_path: Path) -> Path:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_bytes(f"device:{device_index}".encode("utf-8"))
        return output_path


def test_detect_image_cli_prints_json_summary(
    monkeypatch: pytest.MonkeyPatch,
    tmp_path: Path,
) -> None:
    image_path = tmp_path / "capture.png"
    image_path.write_bytes(b"png")
    monkeypatch.setattr(cli, "YoloObjectDetector", lambda: FakeDetector())
    runner = CliRunner()

    result = runner.invoke(
        cli.app,
        [
            "detect-image",
            str(image_path),
            "--output-root",
            str(tmp_path / "runs"),
            "--model-path",
            str(tmp_path / "yolo11n.pt"),
            "--run-name",
            "manual",
        ],
    )

    assert result.exit_code == 0
    assert '"class_name": "person"' in result.output
    assert '"detection_count": 1' in result.output
    assert str(tmp_path / "runs" / "manual" / "capture.jpg") in result.output


def test_detect_webcam_cli_captures_frame_then_detects(
    monkeypatch: pytest.MonkeyPatch,
    tmp_path: Path,
) -> None:
    monkeypatch.setattr(cli, "OpenCvWebcamFrameCapture", lambda: FakeCapture())
    monkeypatch.setattr(cli, "YoloObjectDetector", lambda: FakeDetector())
    runner = CliRunner()

    result = runner.invoke(
        cli.app,
        [
            "detect-webcam",
            "--device-index",
            "1",
            "--capture-path",
            str(tmp_path / "captures" / "webcam.png"),
            "--output-root",
            str(tmp_path / "runs"),
            "--model-path",
            str(tmp_path / "yolo11n.pt"),
        ],
    )

    assert result.exit_code == 0
    assert (tmp_path / "captures" / "webcam.png").read_bytes() == b"device:1"
    assert '"class_name": "person"' in result.output
