# Sprint 03 - Training Pipeline

## Goal

Create the asynchronous PyTorch training pipeline with queued jobs, status transitions, dataset materialization, metrics persistence, and model artifact creation.

## Priority

P0

## Dependencies

- Sprint 02 annotation and versioning complete.
- Dataset versions are immutable and queryable.
- Technical pass has selected queue implementation and Rust/Python contract.

## Scope

- Add training job creation endpoint.
- Add `training_jobs` table and lifecycle states.
- Add queue producer from Rust API.
- Add Python/PyTorch worker consumer.
- Materialize dataset version for the worker.
- Run tiny training or controlled mock training for local demo speed.
- Persist training metrics.
- Store model artifact on successful job.

## BDD Validation Criteria

### Scenario: Training job is queued
Given an immutable dataset version exists
When a client calls `POST /training-jobs`
Then the API creates a job with status `queued` and includes the dataset version id

### Scenario: Worker updates job status
Given a queued training job exists
When the Python worker picks up the job
Then the job moves to `running` and finally to `succeeded` or `failed`

### Scenario: Metrics are persisted
Given a training job completes successfully
When a client calls `GET /training-jobs/{job_id}/metrics`
Then the API returns metrics linked to that job, including loss or detection metrics

### Scenario: Failed job keeps error message
Given the worker cannot materialize the dataset
When the training job fails
Then the job status is `failed` and the error message is persisted for inspection

## Definition of Done

- Job creation uses dataset version id only.
- Queue producer and worker consumer contract is documented.
- Worker can run against the seed dataset path.
- Metrics are written with job id and epoch when applicable.
- Successful job writes a model artifact URI.
- Failed job preserves readable error details.
