# PerceptionLab

Rust-powered ingestion and training platform for real-time computer vision models.

## What Is It?

PerceptionLab is a Rust + PyTorch ML infrastructure project. It turns raw visual data into trainable, versioned, exportable computer vision models.

Core flow:

```text
Upload data -> build dataset -> launch training -> track metrics -> export model -> run inference
```

## Why This Project?

Most computer vision demos stop at model inference. PerceptionLab focuses on the infrastructure required before and after the model: dataset ingestion, annotation storage, dataset versioning, async training jobs, metrics, model registry, inference API, visual overlays, and ONNX export.

The portfolio signal is explicit: this is not a model demo, it is ML infrastructure.

## Architecture

- Rust API service for ingestion, orchestration, dataset/version/model endpoints, and health checks.
- Python/PyTorch worker for training, metrics writing, inference, and export jobs.
- PostgreSQL for datasets, samples, annotations, versions, jobs, metrics, models, exports, and inference runs.
- Object storage or filesystem storage behind an adapter for images, model artifacts, exports, and overlays.
- Queue-backed asynchronous training so HTTP requests never block on ML work.
- Docker Compose local stack for the final MVP demo.

## Features Planned For MVP

- Dataset creation and listing.
- Image upload with validation and metadata extraction.
- Bounding-box annotation management.
- Immutable dataset versions.
- Async PyTorch training jobs.
- Training metrics tracking.
- Model registry.
- Model inference endpoint.
- ONNX export.
- Overlay generation for visual detections.

## Project Layout

- `apps/api-rust/` - planned Rust API service.
- `workers/pytorch-trainer/` - planned Python/PyTorch worker.
- `infra/` - local infrastructure notes and Docker Compose target.
- `datasets/seed/` - planned minimal demo dataset.
- `doc/` - product, architecture, QA, sprint, demo, and reference documentation.
- `scripts/` - local automation used by hooks and quality gates.
- `tests/` - policy, unit, integration, and contract tests.
- `.githooks/` - versioned Git hooks.

## Quickstart For Current Foundation

```bash
npm install
npm test
npm run validate:docs
npm run lint:architecture
npm run quality
npm run prepare:hooks
```

`npm run prepare:hooks` configures Git to use `.githooks/`.

## Product References

- [Product spec](doc/specs/product-spec.md)
- [Architecture spec](doc/specs/architecture-spec.md)
- [API spec](doc/specs/api-spec.md)
- [Domain model](doc/specs/domain-model.md)
- [Product modules](doc/specs/modules.md)
- [Requirements](doc/specs/requirements.md)
- [Roadmap](doc/specs/roadmap.md)
- [Demo spec](doc/specs/demo-spec.md)
- [QA BDD](doc/quality/qa-bdd.md)
- [Acceptance matrix](doc/quality/acceptance-matrix.md)
