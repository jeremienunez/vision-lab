"""Strict contracts for training jobs consumed by the worker."""

from pydantic import BaseModel, ConfigDict, Field


class TrainingHyperparametersPayload(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True, strict=True)

    epochs: int = Field(gt=0)
    batch_size: int = Field(gt=0)
    image_size: int = Field(gt=0)
    learning_rate: float = Field(gt=0)


class TrainingJobPayload(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True, strict=True)

    job_id: str = Field(min_length=1)
    dataset_version_id: str = Field(min_length=1)
    model_family: str = Field(min_length=1)
    base_model: str | None = None
    hyperparameters: TrainingHyperparametersPayload
    classes: tuple[str, ...] = Field(min_length=1)
