# Architecture Spec

## Product Architecture

PerceptionLab is API-first ML infrastructure:

- Rust API service orchestrates datasets, samples, annotations, versions, jobs, models, exports, and inference requests.
- Python/PyTorch worker consumes queued jobs, materializes datasets, trains or fine-tunes models, writes metrics, and stores artifacts.
- PostgreSQL stores business state and traceability data.
- Object storage or filesystem storage stores images, checkpoints, model artifacts, ONNX exports, and overlays.
- A queue separates HTTP requests from long-running ML work.
- Docker Compose is the local proof environment for the MVP.

## Planned Components

- `apps/api-rust/` - Rust API service, likely Axum during technical pass.
- `workers/pytorch-trainer/` - Python worker for PyTorch training, inference, export, and overlays.
- `infra/` - Docker Compose, database, storage, queue, and local operational notes.
- `datasets/seed/` - minimal demo dataset for recruiter-friendly quickstart.
- `doc/demo/` - expected input image, overlay, and JSON response artifacts.

## Dependency Direction

The implementation should preserve these boundaries:

- Domain types and rules know nothing about HTTP, SQL, queues, storage, or PyTorch.
- Application use cases own orchestration and depend on ports.
- Infrastructure adapters implement ports for PostgreSQL, storage, queue, and artifact handling.
- Delivery adapters expose HTTP, CLI, worker entrypoints, and OpenAPI.
- Python worker contracts are explicit through queue payloads, database state, and artifact paths.

## SOLID Criteria

- Single Responsibility: dataset, sample, annotation, job, model, export, and inference modules each have one reason to change.
- Open Closed: new storage backends, model families, and export formats extend ports instead of rewriting use cases.
- Liskov Substitution: adapters honor the same contracts across local filesystem, MinIO, and future object storage.
- Interface Segregation: ports are small, task-oriented, and not generic service bags.
- Dependency Inversion: application logic depends on abstractions, not concrete SQL, queue, or storage clients.

## Design Patterns

- Port and adapter for storage, repositories, queue, artifact registry, training worker, and inference runtime.
- Use case service for application workflows such as dataset creation, sample upload, version creation, job creation, and inference request.
- Value object for dataset name, task type, class list, normalized bounding box, checksum, model status, and job status.
- Repository only for real persistence behavior.
- Factory only when creation rules branch by model family, export format, or storage backend.

## Local Validation

Dependency Cruiser currently validates the JavaScript policy tooling in `scripts/` and `tests/`. Rust and Python implementation checks must be added during the technical pass with the equivalent tools: `cargo test`, `cargo clippy`, Python tests, and integration checks.

Run:

```bash
npm run lint:architecture
```
