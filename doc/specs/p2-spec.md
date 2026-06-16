# P2 Advanced Platform Spec

## Objective

P2 turns the API-first MVP into an advanced ML platform demo. The work must stay incremental: each slice should be independently testable, documented, and useful before the next subsystem starts.

## P2 Slices

### P2A - Advanced Registry And Dataset Quality

Purpose: improve model governance and training reproducibility without changing the product shape.

Scope:

- Add configurable train/validation/test split when creating dataset versions.
- Add CoreML model export alongside ONNX.
- Add model comparison over metrics summaries.
- Add model promotion with the invariant that only one model is promoted for a dataset version and model family.

Primary endpoints:

- `POST /datasets/{dataset_id}/versions` accepts optional `split_config`.
- `POST /models/{model_id}/exports` accepts `onnx` and `coreml`.
- `POST /models/compare` compares two or more model ids.
- `POST /models/{model_id}/promote` promotes a validated or candidate model.

Validation signals:

- Invalid split percentages are rejected.
- CoreML export returns a stored artifact URI with a CoreML extension.
- Model comparison returns metrics per model and identifies the best model when a comparable metric exists.
- Promoting a model demotes or archives competing promoted models according to the selected invariant.

### P2B - API Security

Purpose: add portfolio-grade API key protection without blocking local development ergonomics.

Scope:

- Add optional API key middleware.
- Keep `/health` public.
- Require `x-api-key` for mutating and protected read endpoints when `PERCEPTIONLAB_API_KEY` is configured.
- Document local and CI usage.

Validation signals:

- Missing API key returns `401`.
- Wrong API key returns `403`.
- Correct API key allows protected requests.
- Healthcheck remains public.

### P2E - External Dataset Ingestion

Purpose: make the platform testable with real external datasets while keeping local storage explicit and secrets out of logs.

Scope:

- Add a Hugging Face dataset ingestion adapter in the Python worker.
- Read `HF_TOKEN` from the local environment, never from committed config.
- Materialize image samples and object-detection annotations into a local YOLO-style layout.
- Write ingested datasets under `PERCEPTIONLAB_DATA_ROOT`, which can point to `/media/jerem/ubuntu1/perceptionlab/datasets`.
- Keep tests deterministic through a fake dataset source and injected loader.

Primary command:

- `perception-worker ingest-hf <source_dataset> --target-name <name> --classes cup,book --max-samples 10`

Validation signals:

- Missing `HF_TOKEN` fails before any network call.
- Loader errors are redacted and do not retain the token in exception causes.
- Materialized output contains `images/`, `labels/`, and `manifest.json`.
- Tests run without a real Hugging Face key or network access.

### P2C - Operations UX

Purpose: expose platform state through a minimal dashboard and streamable logs.

Scope:

- Add minimal web dashboard only after API-first endpoints are stable.
- Add training log persistence contract.
- Add log listing or SSE streaming for a training job.

Validation signals:

- Dashboard shows datasets, jobs, models, and latest metrics from API calls.
- Training logs can be read or streamed for a job.
- Missing job ids return standard errors.

### P2D - Media And Metadata Extensions

Purpose: extend perception inputs without destabilizing the object detection core.

Scope:

- Support video samples as a separate media path.
- Support optional depth metadata on samples and detections.
- Preserve current image upload and inference contracts.

Validation signals:

- Video upload is accepted only for supported MIME types.
- Video metadata is stored separately from image dimensions where needed.
- Depth metadata flows into detections and overlays when available.

## Execution Order

1. P2A first: it builds directly on P1 model export, metrics, dataset versions, and model registry.
2. P2E second: make the platform testable with external datasets before broader UX work.
3. P2B third: secure the API once advanced registry and ingestion operations exist.
4. P2C fourth: dashboard and logs depend on stable APIs.
5. P2D last: video and depth introduce new media semantics and should not be mixed with registry work.

## Out Of Scope For The First P2 Slice

- Real mobile client.
- Real-time camera streaming.
- Full production authentication.
- Multi-tenant authorization.
- Non-local object storage hardening.
