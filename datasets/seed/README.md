# Seed Dataset

Planned demo dataset:

- Name: `desk-objects-v1`.
- Classes: `cup`, `book`, `phone`, `keyboard`, `mouse`.
- Images: small local desk photos.
- Annotations: enough bounding boxes to demonstrate ingestion, training, inference, and overlay generation.

Keep this dataset small so the local demo stays fast.

Local absolute path after running `npm run bootstrap:env`:

```text
/home/jerem/vision-lab/datasets/seed
```

The generated `.env.local` file exposes this path as `PERCEPTIONLAB_SEED_DATASET_ROOT`.
