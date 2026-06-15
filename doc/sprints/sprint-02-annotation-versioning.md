# Sprint 02 - Annotation And Versioning

## Goal

Add annotation management, dataset stats, immutable dataset versions, YOLO import support, and a minimal seed dataset.

## Priority

P0

## Dependencies

- Sprint 01 API foundation complete.
- Dataset and sample persistence available.
- Class list stored on datasets.

## Scope

- Add bounding-box annotation endpoints.
- Validate normalized bbox coordinates.
- Validate annotation class membership.
- Add dataset stats endpoint.
- Add immutable dataset version creation and listing.
- Add basic YOLO import path if storage and parsing are ready.
- Add seed dataset structure for `desk-objects-v1`.

## BDD Validation Criteria

### Scenario: Annotation is accepted
Given an existing sample and a class defined on its dataset
When a client calls `POST /samples/{sample_id}/annotations` with a valid normalized bbox
Then the annotation is persisted and can be listed for the sample

### Scenario: Annotation class is rejected
Given an existing sample in a dataset with classes `cup` and `book`
When a client submits an annotation for class `phone`
Then the API rejects the annotation because the class does not exist in the dataset

### Scenario: Dataset version is immutable
Given a dataset has samples and annotations
When a client calls `POST /datasets/{dataset_id}/versions`
Then the created version captures samples, annotations, and classes without changing when later dataset edits happen

### Scenario: Dataset stats are visible
Given a dataset contains samples and annotations
When a client calls `GET /datasets/{dataset_id}/stats`
Then the API returns sample count, annotation count, and class coverage

## Definition of Done

- Annotation endpoints create and list bbox annotations.
- Bbox validation rejects coordinates outside `[0, 1]`.
- Dataset version stores classes, sample count, annotation count, and split config.
- Training jobs cannot target mutable datasets directly.
- Seed dataset instructions exist for `desk-objects-v1`.
