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

## P0 - MVP Obligatoire

- [ ] Create Rust API with healthcheck.
- [ ] Create initial PostgreSQL schema.
- [ ] Add `POST /datasets` and `GET /datasets`.
- [ ] Add `POST /datasets/{dataset_id}/samples`.
- [ ] Connect abstracted file/object storage.
- [ ] Add annotation endpoints.
- [ ] Add `GET /datasets/{dataset_id}/stats`.
- [ ] Add `POST /datasets/{dataset_id}/versions`.
- [ ] Add `POST /training-jobs`.
- [ ] Add `training_jobs` table and job lifecycle.
- [ ] Add queue-backed training orchestration.
- [ ] Add minimal Python/PyTorch worker.
- [ ] Add training loop or wrapper model.
- [ ] Persist training metrics.
- [ ] Add minimal model registry.
- [ ] Add `GET /models`.
- [ ] Add `POST /models/{model_id}/infer`.
- [ ] Add Docker Compose.
- [ ] Add product-grade README quickstart.

## P1 - Fortement Valorisant

- [ ] Import annotations YOLO.
- [ ] Export annotations YOLO.
- [ ] Export model to ONNX.
- [ ] Generate visual overlay image.
- [ ] Track metrics by class.
- [ ] Add `GET /training-jobs/{job_id}/metrics`.
- [ ] Publish OpenAPI/Swagger documentation.
- [ ] Add simple CLI.
- [ ] Add seed demo dataset.
- [ ] Benchmark inference latency.

## P2 - Avance

- [ ] Add CoreML export.
- [ ] Add configurable train/validation/test split.
- [ ] Compare models.
- [ ] Promote model.
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
