# PerceptionLab

Rust-powered ingestion and training platform for real-time computer vision models.

## What Is It?

PerceptionLab is a Rust + PyTorch ML infrastructure project. It turns raw visual data into trainable, versioned, exportable computer vision models.

Core flow:

```text
Upload data -> build dataset -> launch training -> track metrics -> export model -> run inference
```

## Why This Project?

Most computer vision demos stop at model inference. PerceptionLab focuses on the infrastructure required before and after the model: dataset ingestion, annotation storage, dataset versioning, async training jobs, metrics, model registry, inference API, visual overlays, and ONNX/CoreML export.

The portfolio signal is explicit: this is not a model demo, it is ML infrastructure.

## Architecture

- Rust API service for ingestion, orchestration, dataset/version/model endpoints, and health checks.
- Python/PyTorch worker for training, metrics writing, inference, and export jobs.
- PostgreSQL for datasets, samples, annotations, versions, jobs, metrics, and models; exports and inference run persistence are the remaining adapter gaps.
- Object storage or filesystem storage behind an adapter for images, model artifacts, exports, and overlays.
- Queue-backed asynchronous training so HTTP requests never block on ML work.
- Docker Compose local stack for the final MVP demo.

## P0 MVP Surface

- Dataset creation and listing.
- Image upload with validation and metadata storage.
- Bounding-box annotation management.
- Immutable dataset versions.
- Configurable train/validation/test splits for dataset versions.
- Async training job creation and queueing.
- Training job lifecycle transitions.
- Training metrics persistence and `GET /training-jobs/{job_id}/metrics`.
- Model registry use cases and `GET /models` / `GET /models/{model_id}`.
- Multipart model inference contract at `POST /models/{model_id}/infer`.
- Model comparison, promotion, and ONNX/CoreML export endpoints.
- Hugging Face dataset ingestion into local `images/`, `labels/`, and `manifest.json`.
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
- `PERCEPTIONLAB_DATA_ROOT=/media/jerem/ubuntu1/perceptionlab/datasets`
- `PERCEPTIONLAB_STORAGE_ROOT=/media/jerem/ubuntu1/perceptionlab/storage`
- `PERCEPTIONLAB_ARTIFACT_ROOT=/media/jerem/ubuntu1/perceptionlab/artifacts`

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

The Compose stack starts PostgreSQL and runs the API with `PERCEPTIONLAB_REPOSITORY_BACKEND=postgres` for datasets, samples, annotations, dataset versions, training jobs, the training job queue, training metrics, and models. The API applies `api/migrations/` at startup through SQLx. Model exports and inference runs still use transient adapters in the current tranche. The Postgres host port defaults to `55432` to avoid local `5432` conflicts; override with `PERCEPTIONLAB_POSTGRES_PORT=5432` if needed.

Run the API directly against a local PostgreSQL database:

```bash
PERCEPTIONLAB_REPOSITORY_BACKEND=postgres \
PERCEPTIONLAB_DATABASE_URL=postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab \
PERCEPTIONLAB_MIGRATIONS_ROOT=/home/jerem/vision-lab/api/migrations \
PERCEPTIONLAB_API_ADDR=127.0.0.1:8080 \
cargo run --manifest-path api/Cargo.toml -p perception_api
```

## Object Recognition Fire Smoke

Run the full local smoke that proves the product inference path can execute an object-detection response on the seed image:

```bash
npm run demo:fire
```

The default smoke uses the deterministic local inference adapter so the product path stays fast and reproducible in CI.

Use a captured phone or webcam image instead of the bundled seed image:

```bash
npm run demo:fire -- --image /absolute/path/to/capture.jpg
```

Run the same product path with the real YOLO worker behind `POST /models/{model_id}/infer`:

```bash
PERCEPTIONLAB_INFERENCE_ENGINE=yolo_cli \
npm run demo:fire -- --image /absolute/path/to/capture.jpg --confidence-threshold 0.25
```

The command starts a transient API, seeds `datasets/seed`, creates a succeeded demo training job, registers a model, runs inference on the selected image, generates an overlay, and prints a JSON summary with detected classes and the overlay artifact URI. Supported custom image formats are `.jpg`, `.jpeg`, `.png`, and `.webp`. In real YOLO mode the registered model artifact defaults to `.perceptionlab/models/yolo11n.pt`; override it with `PERCEPTIONLAB_FIRE_MODEL_ARTIFACT_URI=file:///absolute/path/to/model.pt`. Override the port with `PERCEPTIONLAB_API_ADDR=127.0.0.1:18080` if `8080` is already used.

## Real YOLO Smoke

Run a real local YOLO prediction on an existing image:

```bash
npm run detect:image -- image.png --model-path .perceptionlab/models/yolo11n.pt --run-name manual
```

Capture one webcam frame and run the same detector:

```bash
npm run detect:webcam -- --device-index 0 --model-path .perceptionlab/models/yolo11n.pt
```

Both commands write artifacts under `.perceptionlab/` and print JSON with `detection_count`, class names, confidences, and the annotated image path.

## Download A Real Detection Dataset

Download a bounded CPPE-5 object-detection subset from Hugging Face into the configured data disk:

```bash
set -a
. ./.env.local
set +a
export HF_HOME=/media/jerem/ubuntu1/perceptionlab/cache/huggingface
export HF_HUB_CACHE=$HF_HOME/hub
export HF_DATASETS_CACHE=$HF_HOME/datasets

cd worker
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker ingest-hf \
  rishitdagli/cppe-5 \
  --target-name cppe5-mvp-40 \
  --classes Coverall,Face_Shield,Gloves,Goggles,Mask \
  --split train \
  --max-samples 40
```

Push the downloaded manifest into a running API:

```bash
PERCEPTIONLAB_API_BASE_URL=http://127.0.0.1:8080 \
PERCEPTIONLAB_SEED_DATASET_ROOT=/media/jerem/ubuntu1/perceptionlab/datasets/cppe5-mvp-40 \
node scripts/seed-demo-dataset.mjs
```

Consume one queued training job from PostgreSQL with the worker:

```bash
cd worker
PERCEPTIONLAB_DATABASE_URL=postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab \
PERCEPTIONLAB_ARTIFACT_ROOT=/media/jerem/ubuntu1/perceptionlab/artifacts \
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker process-once \
  --repository-backend postgres \
  --trainer tiny_torch
```

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
  -d '{
    "version_name": "v1",
    "split_config": { "train": "70", "validation": "20", "test": "10" },
    "created_by": "local-user"
  }'
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

Process one queued job with the Python worker:

```bash
cd worker
PERCEPTIONLAB_DATABASE_URL=postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab \
PERCEPTIONLAB_ARTIFACT_ROOT=/media/jerem/ubuntu1/perceptionlab/artifacts \
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker process-once \
  --repository-backend postgres \
  --trainer tiny_torch
```

Transition a local demo job before registering a model:

```bash
curl -sS -X PATCH http://127.0.0.1:8080/training-jobs/<job_id>/status \
  -H 'content-type: application/json' \
  -d '{"next_status": "running", "error_message": null}'
curl -sS -X PATCH http://127.0.0.1:8080/training-jobs/<job_id>/status \
  -H 'content-type: application/json' \
  -d '{"next_status": "succeeded", "error_message": null}'
```

Model registry and inference routes are wired for registered models:

```bash
curl -sS -X POST http://127.0.0.1:8080/models \
  -H 'content-type: application/json' \
  -d '{
    "training_job_id": "<job_id>",
    "name": "desk-objects-demo",
    "version": "v1",
    "artifact_uri": "file:///tmp/perceptionlab/demo-model.pt",
    "metrics_summary": { "mAP50": "0.91", "classes": "cup,book" }
  }'
curl -sS http://127.0.0.1:8080/models
curl -sS -X POST http://127.0.0.1:8080/models/<model_id>/exports \
  -H 'content-type: application/json' \
  -d '{"format": "coreml"}'
curl -sS -X POST http://127.0.0.1:8080/models/compare \
  -H 'content-type: application/json' \
  -d '{
    "model_ids": ["<baseline_model_id>", "<challenger_model_id>"],
    "metric_name": "mAP50"
  }'
curl -sS -X POST http://127.0.0.1:8080/models/<model_id>/promote
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

## Hugging Face Dataset Ingestion

Keep the Hugging Face token in local environment only:

```bash
HF_TOKEN=<redacted>
PERCEPTIONLAB_DATA_ROOT=/media/jerem/ubuntu1/perceptionlab/datasets
```

Materialize a small external dataset slice through the worker:

```bash
cd worker
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker ingest-hf owner/desk-objects \
  --target-name desk-objects-hf \
  --classes cup,book \
  --split train \
  --max-samples 10
```

The command writes `images/`, `labels/`, and `manifest.json` under `PERCEPTIONLAB_DATA_ROOT/<target-name>`.

## Seed Demo Dataset

The demo seed lives in `datasets/seed/` and contains one generated PNG sample, matching YOLO labels, and a manifest for `desk-objects-v1`.

Validate the seed assets:

```bash
npm run validate:seed
```

With the API running, seed the local transient API:

```bash
sh scripts/seed_demo_dataset.sh
```

## Benchmarks

Inference latency benchmark harness:

```bash
npm run benchmark:inference -- --model-id <model_id> --iterations 10
```

Benchmark notes live in [doc/benchmarks/inference-latency.md](doc/benchmarks/inference-latency.md).

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
