# Worker Workspace

Python/PyTorch worker code lives under `worker/perception_worker/`.

The worker is a typed application component, not a loose script folder.

Hugging Face ingestion command:

```bash
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker ingest-hf owner/desk-objects \
  --target-name desk-objects-hf \
  --classes cup,book \
  --max-samples 10
```

`HF_TOKEN` is read from the environment and must not be committed.

Local YOLO directory ingestion command for Roboflow, Open Images, or manually merged
exports:

```bash
PERCEPTIONLAB_DATA_ROOT=/media/jerem/ubuntu1/perceptionlab/datasets \
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker ingest-yolo \
  /media/jerem/ubuntu1/perceptionlab/raw/phone-remote-yolo \
  --target-name phone-remote-mix \
  --classes phone,remote,person,laptop,mouse,keyboard \
  --split train
```

The source directory must contain `data.yaml`, `images/<split>`, and matching
`labels/<split>/<image-stem>.txt` YOLO label files. Images with empty labels are
preserved as hard negatives.

Process one PostgreSQL-backed training job:

```bash
PERCEPTIONLAB_DATABASE_URL=postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab \
PERCEPTIONLAB_ARTIFACT_ROOT=/media/jerem/ubuntu1/perceptionlab/artifacts \
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker process-once \
  --repository-backend postgres \
  --trainer tiny_torch
```

`process-once` leases one queued job, marks it running, writes metrics, creates a candidate model, and completes or fails the queue entry.

Process one queued YOLO fine-tuning job:

```bash
PERCEPTIONLAB_DATABASE_URL=postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab \
PERCEPTIONLAB_ARTIFACT_ROOT=/media/jerem/ubuntu1/perceptionlab/artifacts \
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run --extra ml perception-worker process-once \
  --repository-backend postgres \
  --trainer yolo_finetune
```

The `yolo_finetune` trainer materializes the queued job's dataset version into an
Ultralytics `data.yaml` layout, trains from `base_model` or `.perceptionlab/models/yolo11n.pt`,
and registers the produced `best.pt` as a candidate model through the same repository path.
