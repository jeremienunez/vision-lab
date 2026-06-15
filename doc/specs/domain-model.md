# Domain Model

## Business Objects

- Dataset
- Sample
- Annotation
- DatasetVersion
- TrainingJob
- TrainingMetric
- Model
- ModelExport
- InferenceRun
- Artifact

## Relationships

- Dataset has many Samples.
- Sample has many Annotations.
- Dataset has many DatasetVersions.
- DatasetVersion has many TrainingJobs.
- TrainingJob has zero or one Model.
- TrainingJob has many TrainingMetrics.
- Model has many ModelExports.
- Model has many InferenceRuns.

## Statuses

| Object | Statuses |
| --- | --- |
| Dataset | `draft`, `ready`, `archived` |
| TrainingJob | `queued`, `running`, `succeeded`, `failed`, `cancelled` |
| Model | `candidate`, `validated`, `promoted`, `archived` |
| Export | `queued`, `running`, `succeeded`, `failed` |

## Business Rules

- A dataset can be modified while it is not archived.
- A dataset version is immutable.
- A training job must use a dataset version, not a mutable dataset directly.
- A sample must belong to a dataset.
- An annotation must belong to a sample.
- An annotation must reference an existing dataset class.
- A model can only be created from a successful training job.
- An export can only be created from an existing model.
- An archived model must not be used by default for inference.
- A failed job must keep its error message.

## MVP Field Expectations

- Dataset: `id`, `name`, `description`, `task_type`, `classes`, `created_at`, `updated_at`, `status`.
- Sample: `id`, `dataset_id`, `storage_uri`, `filename`, `mime_type`, `width`, `height`, `size_bytes`, `checksum`, `source`, `metadata`, `created_at`.
- Annotation: `id`, `sample_id`, `dataset_id`, `class_name`, `class_id`, `bbox`, `format`, `confidence`, `source`, `created_at`.
- DatasetVersion: `id`, `dataset_id`, `version_name`, `sample_count`, `annotation_count`, `classes_snapshot`, `split_config`, `created_at`, `created_by`.
- TrainingJob: `id`, `dataset_version_id`, `model_family`, `base_model`, `status`, `hyperparameters`, `created_at`, `started_at`, `finished_at`, `error_message`, `output_model_id`.
- Model: `id`, `name`, `version`, `training_job_id`, `dataset_version_id`, `model_family`, `artifact_uri`, `metrics_summary`, `status`, `created_at`.
- ModelExport: `id`, `model_id`, `format`, `artifact_uri`, `status`, `created_at`, `error_message`.
