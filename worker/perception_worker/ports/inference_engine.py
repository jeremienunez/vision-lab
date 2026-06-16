"""Real inference strategy port."""

from pathlib import Path
from typing import Protocol

from perception_worker.domain.real_inference import RealInferenceResult


class RealInferenceEngine(Protocol):
    def detect_image(
        self,
        *,
        image_path: Path,
        model_path: Path,
        output_root: Path,
        run_name: str,
        confidence_threshold: float,
    ) -> RealInferenceResult:
        """Run object detection on one image and return persisted artifact metadata."""
        ...
