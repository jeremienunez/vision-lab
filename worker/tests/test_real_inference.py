from pathlib import Path

import pytest
from PIL import Image

from perception_worker.adapters.inference.webcam_capture import OpenCvWebcamFrameCapture
from perception_worker.adapters.inference.yolo_object_detector import YoloObjectDetector


class FakeTensor:
    def __init__(self, value: object) -> None:
        self._value = value

    def item(self) -> object:
        return self._value


class FakeCoordinateTensor:
    def __init__(self, values: list[float]) -> None:
        self._values = values

    def tolist(self) -> list[float]:
        return self._values


class FakeBox:
    cls = [FakeTensor(0)]
    conf = [FakeTensor(0.87)]
    xyxy = [FakeCoordinateTensor([10.0, 20.0, 110.0, 220.0])]


class FakePrediction:
    names = {0: "person"}
    boxes = [FakeBox()]


class FakeYoloModel:
    def __init__(self) -> None:
        self.calls: list[dict[str, object]] = []

    def predict(self, **kwargs: object) -> list[FakePrediction]:
        self.calls.append(kwargs)
        return [FakePrediction()]


def test_yolo_object_detector_returns_detections_and_artifact_paths(tmp_path: Path) -> None:
    image_path = tmp_path / "capture.png"
    Image.new("RGB", (200, 400), color="white").save(image_path)
    model = FakeYoloModel()
    detector = YoloObjectDetector(model_loader=lambda _model_path: model)

    result = detector.detect_image(
        image_path=image_path,
        model_path=tmp_path / "yolo11n.pt",
        output_root=tmp_path / "runs",
        run_name="manual",
        confidence_threshold=0.25,
    )

    assert result.image_path == image_path
    assert result.image_width == 200
    assert result.image_height == 400
    assert result.output_dir == tmp_path / "runs" / "manual"
    assert result.annotated_image_path == tmp_path / "runs" / "manual" / "capture.jpg"
    assert result.label_path == tmp_path / "runs" / "manual" / "labels" / "capture.txt"
    assert result.detections[0].class_name == "person"
    assert result.detections[0].confidence == 0.87
    assert result.detections[0].bbox_xyxy == (10.0, 20.0, 110.0, 220.0)
    assert model.calls == [
        {
            "source": str(image_path),
            "project": str(tmp_path / "runs"),
            "name": "manual",
            "exist_ok": True,
            "save": True,
            "save_txt": True,
            "conf": 0.25,
            "verbose": False,
        }
    ]


def test_yolo_object_detector_rejects_missing_image(tmp_path: Path) -> None:
    detector = YoloObjectDetector(model_loader=lambda _model_path: FakeYoloModel())

    with pytest.raises(FileNotFoundError):
        detector.detect_image(
            image_path=tmp_path / "missing.png",
            model_path=tmp_path / "yolo11n.pt",
            output_root=tmp_path / "runs",
            run_name="manual",
            confidence_threshold=0.25,
        )


class FakeVideoCapture:
    def __init__(self, device_index: int) -> None:
        self.device_index = device_index
        self.released = False

    def isOpened(self) -> bool:
        return True

    def read(self) -> tuple[bool, str]:
        return True, "frame"

    def release(self) -> None:
        self.released = True


def test_webcam_capture_writes_one_frame(tmp_path: Path) -> None:
    captures: list[FakeVideoCapture] = []
    writes: list[tuple[Path, object]] = []

    def capture_factory(device_index: int) -> FakeVideoCapture:
        capture = FakeVideoCapture(device_index)
        captures.append(capture)
        return capture

    def write_image(output_path: Path, frame: object) -> bool:
        writes.append((output_path, frame))
        return True

    capture = OpenCvWebcamFrameCapture(
        capture_factory=capture_factory,
        image_writer=write_image,
    )

    output_path = capture.capture_frame(device_index=1, output_path=tmp_path / "webcam.png")

    assert output_path == tmp_path / "webcam.png"
    assert captures[0].device_index == 1
    assert captures[0].released is True
    assert writes == [(tmp_path / "webcam.png", "frame")]
