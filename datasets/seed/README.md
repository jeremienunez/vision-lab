# Seed Dataset

Demo dataset:

- Name: `desk-objects-v1`.
- Classes: `cup`, `book`, `phone`, `keyboard`, `mouse`.
- Image: `images/desk-objects.png`, generated from `images/desk-objects.svg`.
- Manifest: `manifest.json`.
- YOLO labels: `labels/desk-objects.txt`.
- Annotations: cup, book, and phone boxes to demonstrate ingestion, training, inference, and overlay generation.

Keep this dataset small so the local demo stays fast.

Local absolute path after running `npm run bootstrap:env`:

```text
/home/jerem/vision-lab/datasets/seed
```

The generated `.env.local` file exposes this path as `PERCEPTIONLAB_SEED_DATASET_ROOT`.

Validate the assets:

```sh
npm run validate:seed
```

Seed a running local API:

```sh
sh scripts/seed_demo_dataset.sh
```
