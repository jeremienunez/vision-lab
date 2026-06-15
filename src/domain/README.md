# Domain Notes

PerceptionLab domain concepts are documented in `doc/specs/domain-model.md`.

Core business objects:

- Dataset
- Sample
- Annotation
- DatasetVersion
- TrainingJob
- TrainingMetric
- Model
- ModelExport
- InferenceRun
- Artifact

Implementation should keep these rules pure and independent from HTTP, SQL, queues, storage, and PyTorch.
