# TODO

## Foundation

- [x] Create base project folders.
- [x] Move source PDFs into `doc/references/`.
- [x] Add changelog and project documentation entry points.
- [x] Add first specs and sprint documents.
- [x] Add BDD sprint validation criteria.
- [x] Add Git hook structure.
- [x] Add commit message shape validation.
- [x] Add Dependency Cruiser architecture rules.
- [x] Align documentation with Product Owning PDF for PerceptionLab.
- [x] Add QA/BDD feature structure and static BDD validation.
- [x] Align folder structure and architecture guard with Design Patterns & Conventions PDF.
- [x] Re-run preparatory coverage audit before P0.

## P0 - MVP Obligatoire

- [x] Bootstrap P0 dependency installation and absolute local paths.
- [x] Create Rust API with healthcheck.
- [x] Create Cargo workspace under `api/`.
- [x] Create `perception_domain` crate with newtype ids, value objects, and state machines.
- [x] Create `perception_app` crate with ports and first use case.
- [x] Create initial PostgreSQL schema.
- [x] Add `POST /datasets` and `GET /datasets`.
- [x] Persist datasets through PostgreSQL when `PERCEPTIONLAB_REPOSITORY_BACKEND=postgres`.
- [x] Add `POST /datasets/{dataset_id}/samples`.
- [x] Persist samples and annotations through PostgreSQL when `PERCEPTIONLAB_REPOSITORY_BACKEND=postgres`.
- [x] Connect abstracted file/object storage.
- [x] Add annotation endpoints.
- [x] Add `GET /datasets/{dataset_id}/stats`.
- [x] Add `POST /datasets/{dataset_id}/versions`.
- [x] Persist dataset versions through PostgreSQL when `PERCEPTIONLAB_REPOSITORY_BACKEND=postgres`.
- [x] Add `POST /training-jobs`.
- [x] Add training job lifecycle persistence and transitions.
- [x] Add queue-backed training orchestration.
- [x] Add minimal Python/PyTorch worker.
- [x] Create installable `worker/perception_worker` package with P0 dependency manifest.
- [x] Add strict worker contracts and strategy ports.
- [x] Add training loop or wrapper model.
- [x] Persist training metrics.
- [x] Add `GET /training-jobs/{job_id}/metrics`.
- [x] Add minimal model registry.
- [x] Add `GET /models`.
- [x] Add `GET /models/{model_id}`.
- [x] Add `POST /models/{model_id}/infer`.
- [x] Add Docker Compose.
- [x] Add product-grade README quickstart.

## P1 - Fortement Valorisant

- [x] Import annotations YOLO.
- [x] Export annotations YOLO.
- [x] Export model to ONNX.
- [x] Generate visual overlay image.
- [x] Track metrics by class.
- [x] Publish OpenAPI/Swagger documentation.
- [x] Add simple CLI.
- [x] Add seed demo dataset.
- [x] Benchmark inference latency.
- [x] Choose final BDD runner and step definition stack during the technical QA pass.

## P2 - Avance

- [x] Add CoreML export.
- [x] Add configurable train/validation/test split.
- [x] Compare models.
- [x] Promote model.
- [x] Add Hugging Face dataset ingestion.
- [x] Add one-command object-recognition fire smoke.
- [x] Add real YOLO image and webcam smoke commands.
- [x] Connect real YOLO inference to the API product fire smoke.
- [ ] Add API key auth.
- [ ] Add minimal web dashboard only after API-first MVP.
- [ ] Stream training logs.
- [ ] Support video.
- [ ] Support depth metadata.

## P3 - Bonus Portfolio

- [ ] Real-time camera client.
- [ ] Mobile iOS prototype.
- [ ] Segmentation model.
- [ ] Depth-aware detections.
- [ ] Model drift report.
- [ ] Human-in-the-loop correction.
- [ ] Auto-labeling with existing model.
