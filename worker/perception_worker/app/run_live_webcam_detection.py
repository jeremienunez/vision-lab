"""Live webcam detection service."""

from dataclasses import dataclass
from pathlib import Path
from typing import Protocol

from perception_worker.adapters.inference.webcam_capture import OpenCvWebcamFrameCapture
from perception_worker.adapters.inference.yolo_object_detector import YoloObjectDetector
from perception_worker.domain.real_inference import RealInferenceResult


class WebcamCapture(Protocol):
    def capture_frame(self, *, device_index: int, output_path: Path) -> Path:
        ...


class LoadedImageDetector(Protocol):
    def detect_image(
        self,
        *,
        image_path: Path,
        output_root: Path,
        run_name: str,
        confidence_threshold: float,
    ) -> RealInferenceResult:
        ...


class ImageDetector(Protocol):
    def load(self, *, model_path: Path) -> LoadedImageDetector:
        ...


@dataclass(frozen=True)
class LiveWebcamFrameResult:
    frame_index: int
    capture_path: Path
    inference: RealInferenceResult

    @property
    def detection_count(self) -> int:
        return len(self.inference.detections)

    def to_summary(self) -> dict[str, object]:
        summary = self.inference.to_summary()
        summary["frame_index"] = self.frame_index
        summary["capture_path"] = str(self.capture_path)
        return summary


@dataclass(frozen=True)
class LiveWebcamRunResult:
    device_index: int
    model_path: Path
    capture_root: Path
    output_root: Path
    run_name: str
    confidence_threshold: float
    frames: tuple[LiveWebcamFrameResult, ...]

    @property
    def frame_count(self) -> int:
        return len(self.frames)

    def to_summary(self) -> dict[str, object]:
        return {
            "device_index": self.device_index,
            "model_path": str(self.model_path),
            "capture_root": str(self.capture_root),
            "output_root": str(self.output_root),
            "run_name": self.run_name,
            "confidence_threshold": self.confidence_threshold,
            "frame_count": self.frame_count,
            "frames": [frame.to_summary() for frame in self.frames],
        }


class LiveWebcamDetector:
    def __init__(
        self,
        *,
        capture: WebcamCapture | None = None,
        detector: ImageDetector | None = None,
    ) -> None:
        self._capture = capture or OpenCvWebcamFrameCapture()
        self._detector = detector or YoloObjectDetector()

    def run(
        self,
        *,
        device_index: int,
        capture_root: Path,
        model_path: Path,
        output_root: Path,
        run_name: str,
        confidence_threshold: float,
        frame_limit: int | None,
    ) -> LiveWebcamRunResult:
        if frame_limit is not None and frame_limit <= 0:
            raise ValueError("frame_limit must be positive when provided")

        capture_root = capture_root.expanduser().resolve()
        output_root = output_root.expanduser().resolve()
        model_path = model_path.expanduser().resolve()
        capture_root.mkdir(parents=True, exist_ok=True)
        output_root.mkdir(parents=True, exist_ok=True)

        loaded_detector = self._detector.load(model_path=model_path)
        frames: list[LiveWebcamFrameResult] = []
        frame_index = 1

        while frame_limit is None or frame_index <= frame_limit:
            frame_run_name = f"{run_name}-{frame_index:04d}"
            capture_path = capture_root / f"{frame_run_name}.png"
            captured_path = self._capture.capture_frame(
                device_index=device_index,
                output_path=capture_path,
            )
            inference = loaded_detector.detect_image(
                image_path=captured_path,
                output_root=output_root,
                run_name=frame_run_name,
                confidence_threshold=confidence_threshold,
            )
            frames.append(
                LiveWebcamFrameResult(
                    frame_index=frame_index,
                    capture_path=captured_path,
                    inference=inference,
                )
            )
            frame_index += 1

        return LiveWebcamRunResult(
            device_index=device_index,
            model_path=model_path,
            capture_root=capture_root,
            output_root=output_root,
            run_name=run_name,
            confidence_threshold=confidence_threshold,
            frames=tuple(frames),
        )
