"""Ultralytics YOLO object detector adapter."""

from collections.abc import Callable
import importlib
from pathlib import Path
from typing import Any

from PIL import Image

from perception_worker.domain.real_inference import RealDetection, RealInferenceResult


class YoloObjectDetector:
    def __init__(self, model_loader: Callable[[str], Any] | None = None) -> None:
        self._model_loader = model_loader or load_ultralytics_model

    def load(self, *, model_path: Path) -> "LoadedYoloObjectDetector":
        model = self._model_loader(str(model_path.expanduser()))
        return LoadedYoloObjectDetector(model=model)

    def detect_image(
        self,
        *,
        image_path: Path,
        model_path: Path,
        output_root: Path,
        run_name: str,
        confidence_threshold: float,
    ) -> RealInferenceResult:
        return self.load(model_path=model_path).detect_image(
            image_path=image_path,
            output_root=output_root,
            run_name=run_name,
            confidence_threshold=confidence_threshold,
        )


class LoadedYoloObjectDetector:
    def __init__(self, model: Any) -> None:
        self._model = model

    def detect_image(
        self,
        *,
        image_path: Path,
        output_root: Path,
        run_name: str,
        confidence_threshold: float,
    ) -> RealInferenceResult:
        resolved_image_path = image_path.expanduser().resolve()
        if not resolved_image_path.is_file():
            raise FileNotFoundError(f"Input image not found: {resolved_image_path}")

        output_root = output_root.expanduser().resolve()
        output_root.mkdir(parents=True, exist_ok=True)
        predictions = self._model.predict(
            source=str(resolved_image_path),
            project=str(output_root),
            name=run_name,
            exist_ok=True,
            save=True,
            save_txt=True,
            conf=confidence_threshold,
            verbose=False,
        )
        output_dir = output_root / run_name
        image_width, image_height = read_image_size(resolved_image_path)

        return RealInferenceResult(
            image_path=resolved_image_path,
            image_width=image_width,
            image_height=image_height,
            output_dir=output_dir,
            annotated_image_path=output_dir / f"{resolved_image_path.stem}.jpg",
            label_path=output_dir / "labels" / f"{resolved_image_path.stem}.txt",
            detections=tuple(parse_detections(predictions)),
        )


def load_ultralytics_model(model_path: str) -> Any:
    ultralytics = importlib.import_module("ultralytics")
    yolo_class = getattr(ultralytics, "YOLO")

    return yolo_class(model_path)


def read_image_size(image_path: Path) -> tuple[int, int]:
    with Image.open(image_path) as image:
        return image.size


def parse_detections(predictions: Any) -> list[RealDetection]:
    detections: list[RealDetection] = []
    if not predictions:
        return detections

    prediction = predictions[0]
    names = getattr(prediction, "names", {})
    for box in getattr(prediction, "boxes", []):
        class_id = int(box.cls[0].item())
        bbox_values = box.xyxy[0].tolist()
        detections.append(
            RealDetection(
                class_id=class_id,
                class_name=str(names.get(class_id, class_id)),
                confidence=float(box.conf[0].item()),
                bbox_xyxy=(
                    float(bbox_values[0]),
                    float(bbox_values[1]),
                    float(bbox_values[2]),
                    float(bbox_values[3]),
                ),
            )
        )

    return detections
