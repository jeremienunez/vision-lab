# Source Notes

This folder keeps the initial architecture layer notes from the repository foundation.

PerceptionLab implementation is now planned around:

- `apps/api-rust/` for the Rust API service.
- `workers/pytorch-trainer/` for Python/PyTorch execution.
- `infra/` for Docker Compose, PostgreSQL, storage, and queue setup.

The same boundaries still apply: domain rules stay independent, use cases depend on ports, infrastructure implements adapters, and delivery layers expose HTTP, CLI, or worker entrypoints.
