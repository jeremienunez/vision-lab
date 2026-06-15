# Sprint 04 - Model Registry And Inference

## Goal

Add model registry, artifact lookup, inference endpoint, detection JSON response, overlay generation, and initial ONNX export.

## Priority

P0

## Dependencies

- Sprint 03 training pipeline complete.
- Successful training jobs produce artifact URIs.
- Inference service ownership is decided in the technical pass.

## Scope

- Create model record after successful training.
- Add model list and detail endpoints.
- Add inference endpoint for selected model.
- Return detections with class, confidence, bbox, and latency.
- Add overlay generation from inference detections.
- Add initial ONNX export request and status tracking.

## BDD Validation Criteria

### Scenario: Model is registered from successful job
Given a training job has completed successfully
When the model artifact is stored
Then the model registry contains a candidate model linked to the job and dataset version

### Scenario: Inference returns detections
Given a registered model is available
When a client calls `POST /models/{model_id}/infer` with an image
Then the API returns detections with class name, confidence, bbox, and latency

### Scenario: Missing model is rejected
Given no model exists for a requested id
When a client calls the inference endpoint
Then the API returns a readable not found error and no inference run is stored

### Scenario: ONNX export is tracked
Given a registered model exists
When a client calls `POST /models/{model_id}/exports` for `onnx`
Then an export record is created with status and artifact URI or readable error

## Definition of Done

- Model records are created only from successful jobs.
- Inference response follows the API spec.
- Latency is included in inference responses.
- Overlay output can be generated from detections.
- ONNX export has persisted status.
- Archived models are not used by default.
