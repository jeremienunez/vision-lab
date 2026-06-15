"""Training result produced by a worker training strategy."""

from dataclasses import dataclass


@dataclass(frozen=True)
class TrainingResult:
    artifact_uri: str
    metrics: dict[str, float]
