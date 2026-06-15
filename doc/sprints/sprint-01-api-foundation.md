# Sprint 01 - API Foundation

## Goal

Create the local Rust API foundation for datasets, sample upload, storage abstraction, database persistence, and Docker Compose startup.

## Priority

P0

## Dependencies

- Sprint 00 foundation complete.
- Technical pass chooses the Rust API framework, database migration tool, storage adapter, and local queue.

## Scope

- Create Rust API service skeleton.
- Add healthcheck endpoint.
- Add PostgreSQL connection and initial migrations.
- Add dataset create/list/detail endpoints.
- Add sample upload endpoint with image validation.
- Add storage abstraction for local filesystem or MinIO-compatible storage.
- Add initial Docker Compose for API, PostgreSQL, storage, and queue if selected.

## BDD Validation Criteria

### Scenario: API healthcheck is available
Given the local stack is running
When a client calls `GET /health`
Then the API returns a healthy status without touching the training worker

### Scenario: Dataset can be created
Given a valid dataset payload with `name`, `task_type`, and `classes`
When a client calls `POST /datasets`
Then the API persists the dataset and returns `dataset_id`, `status`, and `created_at`

### Scenario: Image sample is uploaded
Given an existing dataset and a supported image file
When a client calls `POST /datasets/{dataset_id}/samples`
Then the file is stored and metadata including filename, dimensions, size, and checksum is persisted

### Scenario: Invalid image is rejected
Given an unsupported file type or oversized upload
When a client uploads the file as a dataset sample
Then the API rejects the upload with a readable validation error and creates no orphan metadata

## Definition of Done

- API service starts locally.
- PostgreSQL migration creates dataset and sample tables.
- Image validation covers MIME type, max size, and dimensions.
- Storage failure does not leave orphan sample metadata.
- API tests cover healthcheck, dataset creation, and sample upload.
- README quickstart documents startup and curl examples.
