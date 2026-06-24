# API Spec

The published executable OpenAPI contract lives in `contracts/openapi.json`.
Validate it with `npm run validate:openapi` or export it with `sh scripts/export_openapi.sh`.

## Optional API Key Auth

`PERCEPTIONLAB_API_KEY` enables portfolio-grade API key protection for local and CI runs.
When it is unset or blank, the API remains open for local MVP ergonomics.

- `GET /health` is always public.
- Protected read and mutating routes require `x-api-key` when the key is configured.
- Missing API keys return `401`.
- Wrong API keys return `403`.
- Local scripts read `PERCEPTIONLAB_API_KEY` and send `x-api-key` automatically.

## MVP Endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| GET | `/health` | Check API readiness. |
| POST | `/datasets` | Create a dataset. |
| GET | `/datasets` | List datasets. |
| GET | `/datasets/{dataset_id}` | Read dataset detail. |
| GET | `/datasets/{dataset_id}/stats` | Read dataset sample and annotation stats. |
| POST | `/datasets/{dataset_id}/samples` | Upload an image sample. |
| GET | `/datasets/{dataset_id}/samples` | List dataset samples. |
| POST | `/samples/{sample_id}/annotations` | Add a bounding-box annotation. |
| GET | `/samples/{sample_id}/annotations` | List sample annotations. |
| POST | `/datasets/{dataset_id}/versions` | Create immutable dataset version. |
| GET | `/datasets/{dataset_id}/versions` | List dataset versions. |
| POST | `/training-jobs` | Create async training job. |
| GET | `/training-jobs` | List training jobs. |
| GET | `/training-jobs/{job_id}` | Read job status. |
| PATCH | `/training-jobs/{job_id}/status` | Transition job status for local orchestration and smoke validation. |
| GET | `/training-jobs/{job_id}/metrics` | Read job metrics. |
| GET | `/training-jobs/{job_id}/metrics/by-class` | Read class-level job metrics. |
| POST | `/models` | Register a candidate model from a succeeded training job. |
| GET | `/models` | List registered models. |
| GET | `/models/{model_id}` | Read model detail. |
| POST | `/models/{model_id}/infer` | Run inference on an image. |
| POST | `/models/{model_id}/exports` | Create model export. |
| GET | `/models/{model_id}/exports` | List model exports. |

## P1 Endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| POST | `/datasets/{dataset_id}/import/yolo` | Import YOLO annotations. |
| GET | `/datasets/{dataset_id}/export/yolo` | Export annotations in YOLO format. |
| POST | `/inference-runs/{run_id}/overlay` | Generate visual overlay. |
| GET | `/artifacts/{artifact_id}/download` | Download stored artifact. |

## P2A Endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| POST | `/models/compare` | Compare registered models by a shared numeric metric. |
| POST | `/models/{model_id}/promote` | Promote a model and demote competing promoted models for the same dataset version and family. |

## P2B API Key Auth Contract

API key protection is optional and controlled by `PERCEPTIONLAB_API_KEY`.

When `PERCEPTIONLAB_API_KEY` is unset or blank, the API keeps the local development behavior: all routes remain reachable without an API key.

When `PERCEPTIONLAB_API_KEY` is configured, `/health` stays public and every other route requires the `x-api-key` header.

Protected request:

```bash
curl -H 'x-api-key: dev-secret' http://127.0.0.1:8080/datasets
```

Missing key response:

```json
{
  "error": {
    "code": "missing_api_key",
    "message": "Missing x-api-key header"
  }
}
```

Wrong key response:

```json
{
  "error": {
    "code": "invalid_api_key",
    "message": "Invalid x-api-key header"
  }
}
```

Expected status codes:

| Condition | Status |
| --- | --- |
| `/health` without key | `200` |
| Protected route without key | `401` |
| Protected route with wrong key | `403` |
| Protected route with matching key | Route-specific success/error |

## Dataset Creation Contract

Request:

```json
{
  "name": "desk-objects-v1",
  "description": "Dataset for detecting desk objects from iPhone captures",
  "task_type": "object_detection",
  "classes": ["cup", "book", "phone", "keyboard", "mouse"]
}
```

Response:

```json
{
  "id": "ds_01hxyz",
  "name": "desk-objects-v1",
  "task_type": "object_detection",
  "classes": ["cup", "book", "phone", "keyboard", "mouse"],
  "status": "draft",
  "created_at": "2026-06-15T12:00:00Z"
}
```

## Dataset Version Contract

Request:

```json
{
  "version_name": "v2",
  "split_config": {
    "train": "70",
    "validation": "20",
    "test": "10"
  },
  "created_by": "local-user"
}
```

`split_config` is optional for backward compatibility. When present, it must contain `train`, `validation`, and `test` percentages that sum to 100.

Response:

```json
{
  "id": "dsv_01hxyz",
  "dataset_id": "ds_01hxyz",
  "version_name": "v2",
  "sample_count": 42,
  "annotation_count": 128,
  "classes_snapshot": ["cup", "book"],
  "split_config": {
    "train": "70",
    "validation": "20",
    "test": "10"
  },
  "created_by": "local-user"
}
```

## Training Job Contract

Request:

```json
{
  "dataset_version_id": "dsv_01hxyz",
  "model_family": "yolo",
  "base_model": "yolo11n",
  "hyperparameters": {
    "epochs": 50,
    "batch_size": 16,
    "image_size": 640,
    "learning_rate": 0.001
  }
}
```

Response:

```json
{
  "id": "job_01hxyz",
  "status": "queued",
  "dataset_version_id": "dsv_01hxyz",
  "model_family": "yolo",
  "created_at": "2026-06-15T12:05:00Z"
}
```

List response:

```json
{
  "training_jobs": [
    {
      "id": "job_01hxyz",
      "status": "queued",
      "dataset_version_id": "dsv_01hxyz",
      "model_family": "yolo",
      "base_model": "yolo11n"
    }
  ]
}
```

Transition request:

```json
{
  "next_status": "succeeded",
  "error_message": null
}
```

`PATCH /training-jobs/{job_id}/status` applies the domain lifecycle rules. The local product smoke uses `queued -> running -> succeeded` before registering a candidate model.

## Model Registration Contract

Request:

```json
{
  "training_job_id": "job_01hxyz",
  "name": "desk-objects-demo",
  "version": "v1",
  "artifact_uri": "file:///tmp/perceptionlab/demo-model.pt",
  "metrics_summary": {
    "mAP50": "0.91",
    "classes": "cup,book,phone,keyboard,mouse"
  }
}
```

`POST /models` requires a succeeded training job and returns the created `ModelResponse`. For the local fake inference adapter, the optional `classes` metric drives deterministic demo detections.

## YOLO Annotation Import Contract

Request:

```json
{
  "files": [
    {
      "sample_filename": "cup.jpg",
      "content": "1 0.250000 0.400000 0.300000 0.400000\n"
    }
  ]
}
```

Response:

```json
{
  "dataset_id": "ds_01hxyz",
  "imported_count": 1
}
```

The YOLO line format is `class_id x_center y_center width height` with normalized values.
The API stores annotations as normalized top-left `x`, `y`, `width`, `height`.

## Inference Contract

Request:

```text
multipart/form-data
image=@test.jpg
confidence_threshold=0.25
```

Response:

```json
{
  "run_id": "irun_01hxyz",
  "model_id": "mdl_01hxyz",
  "latency_ms": 48,
  "detections": [
    {
      "class_id": 0,
      "class_name": "cup",
      "confidence": 0.89,
      "bbox": { "x": 0.36, "y": 0.48, "width": 0.28, "height": 0.31 },
      "distance_m": 0.4
    }
  ]
}
```

The `run_id` is persisted and can be used to generate downstream visual artifacts.

## Visual Overlay Contract

`POST /inference-runs/{run_id}/overlay` generates an SVG overlay from stored detections.

Response:

```json
{
  "inference_run_id": "irun_01hxyz",
  "artifact_uri": "file:///tmp/perceptionlab/artifacts/overlays/irun_01hxyz.svg",
  "labels": ["cup 89%"]
}
```

Unknown inference run ids return `404`.

## Product Fire Smoke

Run the local object-recognition smoke from the repository root:

```bash
npm run demo:fire
```

Use a custom image captured from a phone or webcam:

```bash
npm run demo:fire -- --image /absolute/path/to/capture.jpg
```
