"""OpenCV webcam frame capture adapter."""

from collections.abc import Callable
from pathlib import Path
from typing import Any, Protocol


class VideoCaptureLike(Protocol):
    def isOpened(self) -> bool:
        ...

    def read(self) -> tuple[bool, Any]:
        ...

    def release(self) -> None:
        ...


class OpenCvWebcamFrameCapture:
    def __init__(
        self,
        capture_factory: Callable[[int], VideoCaptureLike] | None = None,
        image_writer: Callable[[Path, Any], bool] | None = None,
    ) -> None:
        self._capture_factory = capture_factory or default_capture_factory
        self._image_writer = image_writer or default_image_writer

    def capture_frame(self, *, device_index: int, output_path: Path) -> Path:
        output_path = output_path.expanduser().resolve()
        output_path.parent.mkdir(parents=True, exist_ok=True)
        capture = self._capture_factory(device_index)
        try:
            if not capture.isOpened():
                raise RuntimeError(f"Webcam device {device_index} could not be opened")

            ok, frame = capture.read()
            if not ok:
                raise RuntimeError(f"Webcam device {device_index} did not return a frame")

            if not self._image_writer(output_path, frame):
                raise RuntimeError(f"Could not write webcam frame to {output_path}")
        finally:
            capture.release()

        return output_path


def default_capture_factory(device_index: int) -> VideoCaptureLike:
    import cv2

    return cv2.VideoCapture(device_index)


def default_image_writer(output_path: Path, frame: Any) -> bool:
    import cv2

    return bool(cv2.imwrite(str(output_path), frame))
