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
- ONNX/CoreML export status and artifact availability.
- Overlay generation from detections.
- ML pipeline consistency across dataset version classes, model metadata, metrics, and inference classes.
- Basic API security for filenames, content types, and non-leaking internal errors.

## Quality Gates

- Unit tests cover policy scripts, domain value objects, and application rules.
- Integration tests cover API, PostgreSQL, storage, and queue behavior.
- Contract tests cover Rust API to Python worker payloads.
- BDD feature files live in `qa/features/` and use English Gherkin for a GitHub technical audience.
- `npm run validate:bdd` statically validates BDD feature structure.
- `@cucumber/cucumber` is the final executable BDD runner, with steps in `qa/steps/**/*.mjs`, support in `qa/support/**/*.mjs`, and reports in `qa/reports/cucumber-report.json`.
- `npm run bdd:smoke` runs the current executable `@smoke` subset without requiring the full local stack.
- Sprint documents include goal, priority, dependencies, scope, BDD criteria, and definition of done.
- Architecture checks run locally and later in CI.

## Criticality Rule

- 100% of P0 scenarios must pass before MVP validation.
- At least 80% of P1 scenarios must pass before portfolio polish is accepted.
- No critical bug may remain on ingestion, versioning, training jobs, or model registry.

## Training Test Modes

- `fake_training`: fast, deterministic, required for CI and BDD acceptance.
- `tiny_training`: real PyTorch on a small seed dataset, required locally or nightly once the worker exists.

## Defect Rule

Every confirmed defect gets a failing test before the fix.
