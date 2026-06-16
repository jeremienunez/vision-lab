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
