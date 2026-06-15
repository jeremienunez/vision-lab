# API Spec

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
| GET | `/training-jobs/{job_id}/metrics` | Read job metrics. |
| GET | `/training-jobs/{job_id}/metrics/by-class` | Read class-level job metrics. |
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

## Model Export Contract

Request:

```json
{
  "format": "onnx"
}
```

## Class Metrics Contract

`GET /training-jobs/{job_id}/metrics/by-class` returns metrics whose metadata includes `class_name`.

Response:

```json
{
  "class_metrics": [
    {
      "training_job_id": "job_01hxyz",
      "class_name": "cup",
      "split_name": "validation",
      "metric_name": "mAP50",
      "metric_value": 0.82,
      "step": null,
      "epoch": 1
    }
  ]
}
```

Response:

```json
{
  "id": "mexp_01hxyz",
  "model_id": "mdl_01hxyz",
  "format": "onnx",
  "artifact_uri": "file:///tmp/model.onnx",
  "status": "succeeded",
  "error_message": null
}
```

`GET /models/{model_id}/exports` returns:

```json
{
  "exports": [
    {
      "id": "mexp_01hxyz",
      "model_id": "mdl_01hxyz",
      "format": "onnx",
      "artifact_uri": "file:///tmp/model.onnx",
      "status": "succeeded",
      "error_message": null
    }
  ]
}
```
