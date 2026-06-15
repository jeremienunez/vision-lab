# Architecture Conventions

## Naming

- Use domain vocabulary: dataset, sample, annotation, dataset version, training job, metric, model, export, inference run, artifact.
- Prefer explicit module names over generic helpers.
- Keep modules small enough to understand in one read.

## API And Worker Boundary

- Rust API owns HTTP, validation, persistence orchestration, and job creation.
- Python worker owns PyTorch execution, dataset materialization, metrics writing, artifact creation, inference, and export.
- Rust and Python communicate through explicit queue payloads, database state, and artifact URIs.
- A failed job must always preserve a readable error message.

## Patterns

- Start with value objects for validated domain concepts.
- Use ports for storage, queue, repositories, artifact registry, trainer, exporter, and inference runtime.
- Add repositories only when persistence behavior exists.
- Add factories only for real branching rules such as model family or export format.
- Keep dashboard work out of the MVP until the API-first flow is credible.

## Review Checklist

- Does this prove ML infrastructure rather than only model inference?
- Does the use case depend on ports instead of adapters?
- Can the Python worker contract be tested without running a full training job?
- Are dataset versions immutable after creation?
- Can a recruiter launch or understand the demo in under ten minutes?
