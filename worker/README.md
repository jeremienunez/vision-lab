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

Process one PostgreSQL-backed training job:

```bash
PERCEPTIONLAB_DATABASE_URL=postgres://perceptionlab:perceptionlab@127.0.0.1:55432/perceptionlab \
PERCEPTIONLAB_ARTIFACT_ROOT=/media/jerem/ubuntu1/perceptionlab/artifacts \
UV_CACHE_DIR=../.perceptionlab/cache/uv uv run perception-worker process-once \
  --repository-backend postgres \
  --trainer tiny_torch
```

`process-once` leases one queued job, marks it running, writes metrics, creates a candidate model, and completes or fails the queue entry.
