"""Ultralytics YOLO object detector adapter."""

from collections.abc import Callable
import importlib
from pathlib import Path
from typing import Any

from perception_worker.domain.real_inference import RealDetection, RealInferenceResult


class YoloObjectDetector:
    def __init__(self, model_loader: Callable[[str], Any] | None = None) -> None:
        self._model_loader = model_loader or load_ultralytics_model

    def detect_image(
        self,
        *,
        image_path: Path,
        model_path: Path,
        output_root: Path,
        run_name: str,
        confidence_threshold: float,
    ) -> RealInferenceResult:
        resolved_image_path = image_path.expanduser().resolve()
        if not resolved_image_path.is_file():
            raise FileNotFoundError(f"Input image not found: {resolved_image_path}")

        output_root = output_root.expanduser().resolve()
        output_root.mkdir(parents=True, exist_ok=True)
        model = self._model_loader(str(model_path.expanduser()))
        predictions = model.predict(
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

        return RealInferenceResult(
            image_path=resolved_image_path,
            output_dir=output_dir,
            annotated_image_path=output_dir / f"{resolved_image_path.stem}.jpg",
            label_path=output_dir / "labels" / f"{resolved_image_path.stem}.txt",
            detections=tuple(parse_detections(predictions)),
        )


def load_ultralytics_model(model_path: str) -> Any:
    ultralytics = importlib.import_module("ultralytics")
    yolo_class = getattr(ultralytics, "YOLO")

    return yolo_class(model_path)


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
