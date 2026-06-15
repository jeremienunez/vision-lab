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
- Fixtures live in `qa/features/fixtures/` and top-level `fixtures/`.
- Reports are expected in `qa/features/reports/`.
- Tags use priority and domain labels: `@p0`, `@p1`, `@api`, `@database`, `@storage`, `@worker`, `@ml`, `@inference`, `@export`, `@nonfunctional`.

## Criticality Levels

- P0: mandatory for MVP.
- P1: important for GitHub credibility.
- P2: advanced or V2.
- P3: bonus.

MVP validation requires all P0 scenarios to pass, at least 80% of P1 scenarios to pass, and no critical defect on ingestion, versioning, training jobs, or model registry.

## Current Runner Status

The final BDD runner is intentionally left for the technical QA pass. Until then:

```bash
npm run validate:bdd
./scripts/run_bdd.sh
```

These commands perform static validation of feature-file structure and keep the acceptance suite visible in CI-ready form.
