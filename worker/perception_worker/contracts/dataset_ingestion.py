"""Strict contracts for external dataset ingestion commands."""

from pydantic import BaseModel, ConfigDict, Field


class DatasetIngestionCommand(BaseModel):
    model_config = ConfigDict(extra="forbid", frozen=True, strict=True)

    source_dataset: str = Field(min_length=1)
    split: str = Field(min_length=1)
    target_name: str = Field(min_length=1)
    classes: tuple[str, ...] = Field(min_length=1)
    max_samples: int | None = Field(default=None, gt=0)
