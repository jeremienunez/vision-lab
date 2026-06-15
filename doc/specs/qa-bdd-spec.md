# QA And BDD Spec

## QA Strategy

PerceptionLab must prove a complete ML infrastructure flow, not only isolated endpoints. QA therefore validates business behavior, traceability, async job lifecycle, artifacts, and portfolio demo readiness.

Each sprint must define acceptance criteria before implementation starts. Criteria must be executable by humans and convertible into automated tests.

## BDD Format

Use this structure:

```gherkin
Scenario: Short behavior name
Given a precise starting context
When one observable action happens
Then one observable outcome is true
```

## Minimum BDD Areas

- Dataset creation and validation.
- Sample upload, file validation, metadata extraction, and storage failure behavior.
- Annotation bbox validation and class membership.
- Immutable dataset versions.
- Training job queue, status lifecycle, metrics, and failure message.
- Model registry creation from successful jobs only.
- Inference response with detections and latency.
- ONNX export status and artifact availability.
- Overlay generation from detections.

## Quality Gates

- Unit tests cover policy scripts, domain value objects, and application rules.
- Integration tests cover API, PostgreSQL, storage, and queue behavior.
- Contract tests cover Rust API to Python worker payloads.
- Sprint documents include goal, priority, dependencies, scope, BDD criteria, and definition of done.
- Architecture checks run locally and later in CI.

## Defect Rule

Every confirmed defect gets a failing test before the fix.
