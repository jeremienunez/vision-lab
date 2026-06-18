from pathlib import Path
from typing import Any, cast

from perception_worker.app.run_live_webcam_detection import LiveWebcamDetector
from perception_worker.domain.real_inference import RealDetection, RealInferenceResult


class FakeCapture:
    def __init__(self) -> None:
        self.calls: list[tuple[int, Path]] = []

    def capture_frame(self, *, device_index: int, output_path: Path) -> Path:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_bytes(f"frame:{len(self.calls) + 1}".encode("utf-8"))
        self.calls.append((device_index, output_path))
        return output_path


class FakeLoadedDetector:
    def __init__(self) -> None:
        self.calls: list[dict[str, object]] = []

    def detect_image(
        self,
        *,
        image_path: Path,
        output_root: Path,
        run_name: str,
        confidence_threshold: float,
    ) -> RealInferenceResult:
        self.calls.append(
            {
                "image_path": image_path,
                "output_root": output_root,
                "run_name": run_name,
                "confidence_threshold": confidence_threshold,
            }
        )
        return RealInferenceResult(
            image_path=image_path,
            image_width=640,
            image_height=480,
            output_dir=output_root / run_name,
            annotated_image_path=output_root / run_name / f"{image_path.stem}.jpg",
            label_path=output_root / run_name / "labels" / f"{image_path.stem}.txt",
            detections=(
                RealDetection(
                    class_id=0,
                    class_name="person",
                    confidence=0.91,
                    bbox_xyxy=(1.0, 2.0, 300.0, 470.0),
                ),
            ),
        )


class FakeDetector:
    def __init__(self, loaded_detector: FakeLoadedDetector) -> None:
        self.loaded_detector = loaded_detector
        self.load_calls: list[Path] = []

    def load(self, *, model_path: Path) -> FakeLoadedDetector:
        self.load_calls.append(model_path)
        return self.loaded_detector


def test_live_webcam_detector_reuses_loaded_model_for_frame_limit(tmp_path: Path) -> None:
    capture = FakeCapture()
    loaded_detector = FakeLoadedDetector()
    detector = FakeDetector(loaded_detector)
    live_detector = LiveWebcamDetector(capture=capture, detector=detector)

    result = live_detector.run(
        device_index=1,
        capture_root=tmp_path / "captures",
        model_path=tmp_path / "yolo11n.pt",
        output_root=tmp_path / "runs",
        run_name="webcam-live",
        confidence_threshold=0.1,
        frame_limit=3,
    )

    assert detector.load_calls == [tmp_path / "yolo11n.pt"]
    assert [call[0] for call in capture.calls] == [1, 1, 1]
    assert [call[1].name for call in capture.calls] == [
        "webcam-live-0001.png",
        "webcam-live-0002.png",
        "webcam-live-0003.png",
    ]
    assert [call["run_name"] for call in loaded_detector.calls] == [
        "webcam-live-0001",
        "webcam-live-0002",
        "webcam-live-0003",
    ]
    assert result.frame_count == 3
    assert result.frames[0].detection_count == 1
    summary = cast(dict[str, Any], result.to_summary())
    frames = cast(list[dict[str, Any]], summary["frames"])
    detections = cast(list[dict[str, Any]], frames[0]["detections"])
    assert detections[0]["class_name"] == "person"
