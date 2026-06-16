"""Domain objects for real image inference smoke runs."""

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class RealDetection:
    class_id: int
    class_name: str
    confidence: float
    bbox_xyxy: tuple[float, float, float, float]

    def to_dict(self) -> dict[str, object]:
        return {
            "class_id": self.class_id,
            "class_name": self.class_name,
            "confidence": self.confidence,
            "bbox_xyxy": list(self.bbox_xyxy),
        }


@dataclass(frozen=True)
class RealInferenceResult:
    image_path: Path
    output_dir: Path
    annotated_image_path: Path
    label_path: Path
    detections: tuple[RealDetection, ...]

    def to_summary(self) -> dict[str, object]:
        return {
            "image_path": str(self.image_path),
            "output_dir": str(self.output_dir),
            "annotated_image_path": str(self.annotated_image_path),
            "label_path": str(self.label_path),
            "detection_count": len(self.detections),
            "detections": [detection.to_dict() for detection in self.detections],
        }
