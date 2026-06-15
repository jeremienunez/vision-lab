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

## P0 MVP Surface

- Dataset creation and listing.
- Image upload with validation and metadata storage.
- Bounding-box annotation management.
- Immutable dataset versions.
- Async training job creation and queueing.
- Training job lifecycle transitions.
- Training metrics persistence and `GET /training-jobs/{job_id}/metrics`.
- Model registry use cases and `GET /models` / `GET /models/{model_id}`.
- Multipart model inference contract at `POST /models/{model_id}/infer`.
- Python worker contracts, fake trainer, and tiny deterministic PyTorch trainer.
- Docker Compose stack for the Rust API and PostgreSQL schema bootstrap.

## Project Layout

- `api/crates/` - Rust hexagonal workspace with `perception_domain`, `perception_app`, `perception_infra`, `perception_http`, and `perception_api`.
- `worker/perception_worker/` - typed Python/PyTorch worker with domain, contracts, app, ports, adapters, and entrypoints.
- `contracts/` - OpenAPI and JSON schemas for public and cross-component contracts.
- `infra/` - local infrastructure notes and Docker Compose target.
- `datasets/seed/` - planned minimal demo dataset.
- `doc/` - product, architecture, QA, sprint, demo, and reference documentation.
- `qa/` - Gherkin features, future step definitions, support utilities, and fixtures.
- `scripts/` - local automation used by hooks and quality gates.
- `tests/` - policy, unit, integration, and contract tests.
- `.githooks/` - versioned Git hooks.

## Quickstart

Prerequisites:

- Node.js 22 or newer.
- Rust toolchain compatible with `api/Cargo.toml`.
- Python 3.12 and `uv`.
- Docker with Compose v2 for the containerized stack.

Install dependencies and local config:

```bash
npm install
npm run install:deps
npm run prepare:hooks
```

`npm run prepare:hooks` configures Git to use `.githooks/`.

`npm run install:deps` generates ignored local path config in `.env.local`, fetches Rust
workspace dependencies, and syncs the Python worker with CPU PyTorch and Ultralytics.

Run the full local quality gate:

```bash
npm run quality
cargo test --manifest-path api/Cargo.toml --workspace
cd worker && UV_CACHE_DIR=../.perceptionlab/cache/uv uv run ruff check .
cd worker && UV_CACHE_DIR=../.perceptionlab/cache/uv uv run mypy perception_worker tests
```

Current generated local paths on this Ubuntu filesystem are:

- `PERCEPTIONLAB_PROJECT_ROOT=/home/jerem/vision-lab`
- `PERCEPTIONLAB_DATA_ROOT=/home/jerem/vision-lab/datasets`
- `PERCEPTIONLAB_STORAGE_ROOT=/home/jerem/vision-lab/.perceptionlab/storage`
- `PERCEPTIONLAB_ARTIFACT_ROOT=/home/jerem/vision-lab/.perceptionlab/artifacts`

Run the Rust API directly:

```bash
PERCEPTIONLAB_API_ADDR=127.0.0.1:8080 cargo run --manifest-path api/Cargo.toml -p perception_api
```

In another terminal:

```bash
curl http://127.0.0.1:8080/health
```

Run the containerized local stack:

```bash
docker compose up api
curl http://127.0.0.1:8080/health
docker compose down
```

The Compose stack starts PostgreSQL and loads `api/migrations/0001_initial_schema.sql` on first boot. The current P0 HTTP process uses local transient adapters for fast demo feedback while the schema and repository ports define the database boundary.

## API Smoke Flow

Create a dataset:

```bash
curl -sS -X POST http://127.0.0.1:8080/datasets \
  -H 'content-type: application/json' \
  -d '{
    "name": "desk-objects-v1",
    "description": "Desk object detection demo",
    "task_type": "object_detection",
    "classes": ["cup", "book"]
  }'
```

Upload a sample after replacing `<dataset_id>`:

```bash
curl -sS -X POST http://127.0.0.1:8080/datasets/<dataset_id>/samples \
  -F 'width=640' \
  -F 'height=480' \
  -F 'file=@<image.jpg>;type=image/jpeg'
```

Create a dataset version after at least one sample and annotation exist:

```bash
curl -sS -X POST http://127.0.0.1:8080/datasets/<dataset_id>/versions \
  -H 'content-type: application/json' \
  -d '{"version_name": "v1", "created_by": "local-user"}'
```

Create a queued training job after replacing `<dataset_version_id>`:

```bash
curl -sS -X POST http://127.0.0.1:8080/training-jobs \
  -H 'content-type: application/json' \
  -d '{
    "dataset_version_id": "<dataset_version_id>",
    "model_family": "tiny_torch",
    "base_model": null,
    "hyperparameters": {
      "epochs": 2,
      "batch_size": 1,
      "image_size": 64,
      "learning_rate": 0.01
    }
  }'
```

Model registry and inference routes are wired for registered models:

```bash
curl -sS http://127.0.0.1:8080/models
curl -sS -X POST http://127.0.0.1:8080/models/<model_id>/infer \
  -F 'confidence_threshold=0.25' \
  -F 'image=@<image.jpg>;type=image/jpeg'
```

## Simple CLI

The local CLI wraps the most common smoke-check calls:

```bash
npm run cli -- health
npm run cli -- datasets
npm run cli -- models
npm run cli -- create-dataset --name desk-objects-v1 --classes cup,book --description "Desk demo"
npm run cli -- openapi
```

Use `--base-url` before the command to target another API:

```bash
npm run cli -- --base-url http://127.0.0.1:8080 health
```

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
- [Design patterns](doc/architecture/design-patterns.md)
- [Architecture review checklist](doc/architecture/review-checklist.md)
