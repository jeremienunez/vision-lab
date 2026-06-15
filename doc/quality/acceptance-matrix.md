# Acceptance Matrix

| Product Module | Feature File | Priority |
| --- | --- | --- |
| Healthcheck | `qa/features/health.feature` | P0 |
| Dataset Management | `qa/features/dataset_management.feature` | P0 |
| Sample Ingestion | `qa/features/sample_ingestion.feature` | P0 |
| Annotation Management | `qa/features/annotation_management.feature` | P0 |
| Dataset Versioning | `qa/features/dataset_versioning.feature` | P0 |
| Training Jobs | `qa/features/training_jobs.feature` | P0 |
| Metrics Tracking | `qa/features/metrics_tracking.feature` | P0 |
| Model Registry | `qa/features/model_registry.feature` | P0 |
| Inference API | `qa/features/inference_api.feature` | P0 |
| Model Export | `qa/features/model_export.feature` | P1 |
| Visual Overlay | `qa/features/visual_overlay.feature` | P1 |
| Artifacts Storage | `qa/features/artifacts_storage.feature` | P0 |
| Database Integrity | `qa/features/database_integrity.feature` | P0 |
| Observability | `qa/features/observability.feature` | P1 |
| Performance Smoke | `qa/features/performance_smoke.feature` | P1 |
| Standard API Errors | `qa/features/standard_api_errors.feature` | P0 |
| Basic API Security | `qa/features/basic_api_security.feature` | P0 |
| Worker Job Locking | `qa/features/worker_job_locking.feature` | P0 |
| ML Pipeline Consistency | `qa/features/ml_pipeline_consistency.feature` | P0 |
| End-to-End Pipeline | `qa/features/end_to_end_pipeline.feature` | P0 |

## Critical Anti-Regression Set

- Create valid dataset.
- Upload valid image.
- Reject invalid file.
- Add valid annotation.
- Reject invalid bbox.
- Create dataset version.
- Guarantee dataset version immutability.
- Create queued training job.
- Worker moves job to running.
- Worker moves job to succeeded.
- Metrics are available.
- Model registry is created.
- Inference returns detections.
- Storage and database stay consistent.
- Model metadata preserves dataset version classes.
- Unsafe filenames and content types are rejected.
