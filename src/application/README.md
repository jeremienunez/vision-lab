# Application Notes

Application use cases should orchestrate PerceptionLab workflows:

- Create dataset.
- Upload sample.
- Add annotation.
- Create immutable dataset version.
- Create training job.
- Read metrics.
- Register model.
- Run inference.
- Request export.

Use cases depend on ports, not concrete PostgreSQL, queue, storage, or PyTorch adapters.
