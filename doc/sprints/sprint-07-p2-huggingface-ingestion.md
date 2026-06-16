# Sprint 07 - P2 Hugging Face Dataset Ingestion

## Goal

Add a testable Hugging Face dataset ingestion path that materializes object-detection samples into local dataset files.

## Priority

P2

## Dependencies

- P0/P1 dataset, annotation, versioning, worker, and seed dataset foundations are complete.
- P2A registry controls are complete.
- `.env.local` can provide `HF_TOKEN` and `PERCEPTIONLAB_DATA_ROOT`.
- The local Ubuntu data disk can be mounted at `/media/jerem/ubuntu1`.

## Scope

- Add strict worker contracts for dataset ingestion commands.
- Add worker domain objects for image samples, annotations, ingestion results, and ingestion errors.
- Add dataset source and ingestion store ports.
- Add a Hugging Face dataset adapter backed by `datasets.load_dataset`.
- Add local filesystem materialization into `images/`, `labels/`, and `manifest.json`.
- Add a `perception-worker ingest-hf` command.
- Keep tests independent from real Hugging Face credentials and network calls.

## BDD Validation Criteria

### Scenario: Hugging Face dataset ingestion is materialized locally
Given `HF_TOKEN` is configured and `PERCEPTIONLAB_DATA_ROOT` points to local storage
When I ingest dataset "owner/desk-objects" with classes cup and book
Then the worker writes images, YOLO labels, and a manifest under the target dataset folder

### Scenario: Missing Hugging Face token is rejected
Given `HF_TOKEN` is not configured
When I run the Hugging Face ingestion command
Then the command fails before loading the external dataset

### Scenario: Hugging Face token is never leaked
Given a Hugging Face loader fails while using a token
When the worker reports the ingestion failure
Then the error message and exception cause do not contain the token

## Definition of Done

- Dataset ingestion service is covered by tests before implementation.
- Hugging Face adapter token handling is covered by tests before implementation.
- CLI command is covered by tests before implementation.
- Worker tests, `ruff`, and strict `mypy` pass.
- `npm run quality` and `cargo test --manifest-path api/Cargo.toml --workspace` pass.
- TODO P2 status is updated only after the slice is implemented.
