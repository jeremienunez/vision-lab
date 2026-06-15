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

- `api/crates/perception_domain/` - Rust domain crate for newtypes, value objects, entities, state machines, and domain errors.
- `api/crates/perception_app/` - Rust application crate for ports and use cases.
- `api/crates/perception_infra/` - Rust infrastructure crate for PostgreSQL, storage, queue, and config adapters.
- `api/crates/perception_http/` - Rust HTTP crate for routes, DTOs, mappers, API errors, and OpenAPI.
- `api/crates/perception_api/` - Rust executable crate for typed bootstrap.
- `worker/perception_worker/` - Python worker for contracts, app services, ports, adapters, PyTorch training, inference, export, and entrypoints.
- `contracts/` - OpenAPI and JSON schema contracts.
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

Rust crate direction:

```text
perception_domain -> no project crate
perception_app    -> perception_domain
perception_infra  -> perception_app + perception_domain
perception_http   -> perception_app + perception_domain
perception_api    -> perception_http + perception_infra
```

Python worker direction:

```text
entrypoints -> app -> ports/domain/contracts
adapters    -> ports/domain/contracts
```

## SOLID Criteria

- Single Responsibility: dataset, sample, annotation, job, model, export, and inference modules each have one reason to change.
- Open Closed: new storage backends, model families, and export formats extend ports instead of rewriting use cases.
- Liskov Substitution: adapters honor the same contracts across local filesystem, MinIO, and future object storage.
- Interface Segregation: ports are small, task-oriented, and not generic service bags.
- Dependency Inversion: application logic depends on abstractions, not concrete SQL, queue, or storage clients.

## Design Patterns

- Hexagonal architecture / Ports & Adapters for API, PostgreSQL, storage, queue, worker, inference, QA.
- One use case per product intention: `create_dataset`, `upload_sample`, `add_annotation`, `create_dataset_version`, `create_training_job`, `run_inference`.
- Newtypes for ids: `DatasetId`, `SampleId`, `AnnotationId`, `DatasetVersionId`, `TrainingJobId`, `ModelId`, `ArtifactId`.
- Value objects for `NormalizedBbox`, `ImageDimensions`, `Checksum`, `ConfidenceScore`, `ArtifactUri`, `TrainingHyperparameters`.
- Repository ports in application, SQLx adapters in infrastructure.
- Unit of Work for dataset version creation, worker locking, training finalization, and model plus metrics writes.
- Strategy for fake/tiny/real training, storage, inference, and export modes.
- State machines for datasets, training jobs, models, and exports.
- DTO + Mapper at HTTP, DB, worker, and OpenAPI boundaries.
- Error mapping at public API and worker boundaries.

## Local Validation

Dependency Cruiser currently validates the JavaScript policy tooling in `scripts/` and `tests/`. `npm run validate:conventions` validates the required architecture folders and blocks vague filenames before P0 implementation starts. Rust and Python implementation checks must be added during the technical pass with the equivalent tools: `cargo test`, `cargo clippy`, Python linting, and Python tests.

Run:

```bash
npm run validate:conventions
npm run lint:architecture
```
