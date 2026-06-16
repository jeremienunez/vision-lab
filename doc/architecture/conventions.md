# Architecture Conventions

## Naming

- Use domain vocabulary: dataset, sample, annotation, dataset version, training job, metric, model, export, inference run, artifact.
- Prefer explicit module names over generic helpers.
- Keep modules small enough to understand in one read.

## API And Worker Boundary

- Rust API owns HTTP, validation, persistence orchestration, and job creation.
- Python worker owns PyTorch execution, dataset materialization, metrics writing, artifact creation, inference, and export.
- The web dashboard owns operational presentation only; it reads public API contracts and must not duplicate application use cases.
- Rust and Python communicate through explicit queue payloads, database state, and artifact URIs.
- A failed job must always preserve a readable error message.

## Patterns

- Hexagonal architecture is mandatory.
- Each product operation is a use case.
- Each external system is an adapter.
- Each critical primitive is a newtype or value object.
- Each mutable lifecycle is a state machine.
- Each public contract has a DTO and mapper.
- Each multi-write operation uses an explicit transaction boundary.
- Each exception to these rules requires an ADR.

## Forbidden Names

The following file or folder names are blocked by `npm run validate:conventions`:

- `utils`
- `helpers`
- `misc`
- `common`
- `manager`
- `service`

Use specific names instead: `checksum_calculator`, `bbox_validation`, `metrics_mapper`, `api_client`, `storage_assertions`, `dataset_materializer`.

## Review Checklist

- Does this prove ML infrastructure rather than only model inference?
- Does the use case depend on ports instead of adapters?
- Can the Python worker contract be tested without running a full training job?
- Are dataset versions immutable after creation?
- Are state transitions done through domain methods?
- Is SQL absent from handlers?
- Are `torch` imports isolated to worker training/inference adapters?
- Are public JSON contracts represented by DTOs and schemas?
- Can a recruiter launch or understand the demo in under ten minutes?
