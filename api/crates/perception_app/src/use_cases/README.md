# Use Cases

One file per product intention:

- `create_dataset.rs`
- `upload_sample.rs`
- `add_annotation.rs`
- `create_dataset_version.rs`
- `create_training_job.rs`
- `transition_training_job.rs`
- `record_training_metric.rs`
- `list_training_metrics.rs`
- `run_inference.rs`

HTTP handlers call use cases; they do not implement business orchestration.
