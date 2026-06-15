# Test Strategy

## Test Pyramid

```text
Unit tests
Integration tests
API contract tests
BDD acceptance tests
End-to-end demo tests
Performance smoke tests
```

## Unit Tests

Validate isolated behavior:

- Bounding-box validation.
- Payload validation.
- Annotation conversion.
- Checksum calculation.
- Status mapping.
- Policy scripts.

## Integration Tests

Validate collaborations:

- API plus PostgreSQL.
- API plus storage.
- Worker plus queue.
- Worker plus database.
- Worker plus artifact storage.

## BDD Acceptance Tests

Validate user-observable behavior through Gherkin scenarios in `qa/features/`.

## End-To-End Demo Tests

Validate the main product story: dataset creation, image upload, annotation, dataset versioning, training job, metrics, model registry, inference, and overlay.

## Performance Smoke Tests

Minimal local targets:

- Dataset creation below 500 ms.
- Training job creation below 500 ms.
- Concurrent upload smoke with 20 images.
- Inference below 1000 ms for MVP local tiny model.

## Quality Gates

### PR Gate

- `cargo fmt`
- `cargo clippy`
- `cargo test`
- Python lint
- Python unit tests
- Database migration check
- API contract smoke
- `npm run quality`

### Integration Gate

- Docker Compose starts.
- PostgreSQL migrations pass.
- Storage is available.
- Queue is available.
- API health is OK.
- Worker health is OK.
- P0 BDD scenarios for the module pass.

### MVP Gate

- All P0 scenarios pass.
- Main end-to-end scenario passes.
- README reproduces the demo.
- Overlay image is generated.
- Model is referenced in registry.
- ONNX export is available or explicitly mocked if P1 is not complete.

## ML Test Modes

- `fake_training`: deterministic and fast, used in CI BDD.
- `tiny_training`: real PyTorch on a tiny dataset, used locally or nightly.

ML QA proves pipeline correctness, not state-of-the-art model quality.
