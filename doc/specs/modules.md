# Product Modules

## Module A - Dataset Management

Purpose: create, list, update, inspect, and version datasets.

MVP task type: `object_detection`.

Acceptance signals:

- Dataset can be created with name, description, task type, and classes.
- Stable dataset id is returned.
- List, detail, stats, samples, and annotations are available.
- Training cannot start without defined classes.

## Module B - Sample Ingestion

Purpose: upload images and persist metadata.

MVP formats: `jpg`, `jpeg`, `png`, `webp`.

Acceptance signals:

- Upload targets an existing dataset.
- File is stored.
- Metadata is persisted: filename, MIME type, width, height, size, checksum.
- Non-image or oversized files are rejected.
- Response includes `sample_id`.

## Module C - Annotation Management

Purpose: manage bounding-box annotations.

Internal bbox format: normalized `x`, `y`, `width`, `height` between `0` and `1`.

Acceptance signals:

- Annotation belongs to a sample.
- Class exists in the dataset.
- Bbox is inside valid bounds.
- YOLO conversion path is possible.
- Simple YOLO import is available by P1.

## Module D - Dataset Versioning

Purpose: freeze a dataset state for reproducible training.

Acceptance signals:

- Training jobs reference `dataset_version_id`.
- Version captures classes, samples, and annotations.
- Later dataset edits do not mutate existing versions.

## Module E - Training Jobs

Purpose: create and track PyTorch training work.

Statuses: `queued`, `running`, `succeeded`, `failed`, `cancelled`.

Acceptance signals:

- Job is created from a dataset version.
- Worker moves status from queued to running.
- Final state is succeeded or failed.
- Metrics are saved.
- Successful job creates a model artifact.

## Module F - Metrics Tracking

Purpose: store and read training metrics.

Expected metrics:

- `train_loss`
- `val_loss`
- `precision`
- `recall`
- `mAP50`
- `mAP50_95`
- `epoch`
- `learning_rate`
- `duration_seconds`

Acceptance signals:

- Successful job has final metrics.
- Metrics are linked to `job_id`.
- Metrics are available through API.
- Summary exposes the best epoch.

## Module G - Model Registry

Purpose: store trained models and artifacts.

Statuses: `candidate`, `validated`, `promoted`, `archived`.

Acceptance signals:

- Model is created only from a successful job.
- Model references dataset version and training job.
- `artifact_uri` is stored.
- Main metrics are exposed.
- V2 can enforce one promoted model per dataset and family.

## Module H - Inference API

Purpose: test a model on an image.

Acceptance signals:

- Image is sent to an existing model.
- Response includes detections.
- Each detection has class, confidence, and bbox.
- Response includes latency.
- Missing or unavailable model ids are rejected.

## Module I - Model Export

Purpose: convert a model to deployable formats.

MVP format: `onnx`.

Future formats: `coreml`, `torchscript`.

Acceptance signals:

- ONNX export can be requested.
- Export artifact is saved.
- Export status is available.
- Failure stores readable error details.

## Module J - Visual Overlay Demo

Purpose: generate a visual proof from detections.

Acceptance signals:

- Overlay is generated from inference results.
- Boxes are visible.
- Labels include class name and confidence.
- Optional `distance_m` appears when depth metadata exists.
