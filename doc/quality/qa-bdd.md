# QA BDD

## Objective

PerceptionLab QA must prove the full product behavior:

```text
Create dataset
-> ingest images
-> add annotations
-> freeze dataset version
-> launch training job
-> track metrics
-> create model
-> run inference
-> export model
-> generate visual overlay
```

The goal is broader than endpoint testing. QA must validate API behavior, business rules, database integrity, storage consistency, training lifecycle, worker behavior, model registry correctness, inference contracts, export artifacts, error handling, observability, and minimal performance.

## Acceptance Sentence

```text
I can ingest visual data, annotate it, version it, train a model, register it, run inference, and generate a visual result - all through a clean Rust + PyTorch platform.
```

## BDD Conventions

- Feature files are written in English.
- Feature files live in `qa/features/`.
- Fixtures live in `qa/fixtures/`.
- Runner is `@cucumber/cucumber`.
- Step definitions live in `qa/steps/**/*.mjs`.
- Support files live in `qa/support/**/*.mjs`.
- Reports are written to `qa/reports/cucumber-report.json`.
- Tags use priority and domain labels: `@p0`, `@p1`, `@p2`, `@api`, `@database`, `@storage`, `@worker`, `@ml`, `@inference`, `@export`, `@nonfunctional`, `@security`, `@e2e`.

## Criticality Levels

- P0: mandatory for MVP.
- P1: important for GitHub credibility.
- P2: advanced or V2.
- P3: bonus.

MVP validation requires all P0 scenarios to pass, at least 80% of P1 scenarios to pass, and no critical defect on ingestion, versioning, training jobs, or model registry.

## Runner

The final BDD runner is Cucumber-JS via `@cucumber/cucumber`.

```bash
npm run validate:bdd
npm run bdd:dry-run
npm run bdd:smoke
./scripts/run_bdd.sh
```

`npm run validate:bdd` performs static validation of feature-file structure. `npm run bdd:dry-run` validates runner wiring across the full suite and still lists undefined full-stack scenarios by design. `npm run bdd:smoke` executes the current stack-free smoke subset for health, API key auth, dashboard API client behavior, and product fire smoke.
