# Roadmap

## V1 - Core ML Pipeline

- Create dataset.
- Upload images.
- Add annotations.
- Launch training job.
- View job status.
- View metrics.
- Register model.
- Run inference.
- Export ONNX.

V1 must be demonstrable locally with `docker compose up`, curl commands, and optionally a simple CLI.

## V2 - Dataset Quality And Advanced Registry

- Dataset versions.
- Configurable train/validation/test split.
- YOLO import and export.
- Model comparison.
- Model promotion.
- CoreML export.
- Visual overlays.
- Class-level metrics.

## V3 - Real-Time Perception Demo

- Mobile or web camera client.
- Frame-by-frame streaming.
- Real-time inference.
- Bounding-box overlay.
- Optional depth metadata.
- Simple tracking.
- Latency benchmark.

## Five-Week MVP Roadmap

| Week | Objective | Deliverables |
| --- | --- | --- |
| 1 | API foundation | Axum server, PostgreSQL connection, migrations, healthcheck, dataset CRUD, sample upload, storage abstraction, initial Docker Compose. |
| 2 | Annotation and versioning | Annotation endpoints, bbox validation, dataset stats, dataset versions, YOLO import, seed dataset, main API tests. |
| 3 | Training pipeline | Training jobs, queue, Python worker, dataset materialization, training script, metrics writing, job status lifecycle. |
| 4 | Model registry and inference | Model registry, artifact storage, inference endpoint, detection JSON, overlay generation, initial ONNX export. |
| 5 | Portfolio polish | Premium README, architecture diagram, demo image/GIF, curl examples, benchmark, model card, roadmap, technical decisions. |
