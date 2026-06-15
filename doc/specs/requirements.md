# Requirements

## Functional Requirements

| ID | Requirement |
| --- | --- |
| FR-001 | Create a dataset with unique name, task type, and optional class list. |
| FR-002 | Upload an image into an existing dataset. |
| FR-003 | Validate image MIME type, maximum size, and dimensions. |
| FR-004 | Store files through an object-storage or local-filesystem abstraction. |
| FR-005 | Persist filename, width, height, size, and checksum. |
| FR-006 | Add a bounding-box annotation to a sample. |
| FR-007 | Validate bbox coordinates and class membership. |
| FR-008 | Create an immutable dataset version. |
| FR-009 | Create a training job from a dataset version. |
| FR-010 | Place the training job into a worker-consumed queue. |
| FR-011 | Worker loads data, runs training, and updates job status. |
| FR-012 | Persist training metrics. |
| FR-013 | Persist model artifact after successful training. |
| FR-014 | Reference models in an API-readable model registry. |
| FR-015 | Run inference on an image with a selected model. |
| FR-016 | Export at least one model to ONNX. |
| FR-017 | Generate an image visualization of detections. |

## Non-Functional Requirements

### Performance

- Image upload target: under 2 seconds for a standard local image.
- Job creation target: under 500 ms before queue processing.
- Local CPU inference target: under 500 ms for a small model.
- Training must be asynchronous and must not block HTTP requests.

### Robustness

- Worker downtime keeps jobs queued.
- Failed jobs store readable errors.
- Storage failures do not create orphan metadata.
- Job statuses remain coherent.

### Observability

- Structured logs.
- Request id for important requests.
- Minimal job event history.
- API logs and worker logs.
- Job status, error message, and metrics table.

### Security

- MVP can run locally without authentication.
- Uploaded files are strictly validated.
- Upload size is limited.
- User payloads never trigger arbitrary execution.
- Filenames are sanitized.
- V2 adds API key, rate limiting, ownership, and dataset/model permissions.

### Reproducibility

Track:

- `dataset_version_id`
- `model_family`
- `base_model`
- `hyperparameters`
- optional `code_version`
- `created_at`

Perfect reproducibility is not required in the MVP, but visible traceability is required.
