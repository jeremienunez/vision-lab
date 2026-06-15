# Sprint 06 - P2 Advanced Registry And Dataset Quality

## Goal

Add the first P2 slice: configurable dataset splits, CoreML export, model comparison, and model promotion.

## Priority

P2

## Dependencies

- P1 model export, overlay, OpenAPI, CLI, seed dataset, benchmark, and BDD runner are complete.
- Dataset versions already persist `split_config`.
- Model statuses already include `candidate`, `validated`, `promoted`, and `archived`.
- Model exports already support ONNX through the application and HTTP layers.

## Scope

- Extend dataset version creation with validated train/validation/test split percentages.
- Extend model export format support from `onnx` to `onnx` plus `coreml`.
- Add model comparison use case and HTTP endpoint.
- Add model promotion use case and HTTP endpoint.
- Update OpenAPI, API spec, BDD coverage, README or CLI examples, and TODO tracking.

## BDD Validation Criteria

### Scenario: Dataset version captures split config
Given dataset "desk-objects-v1" contains annotated samples
When I create dataset version "v2" with train 70 validation 20 and test 10
Then the dataset version response contains the split configuration

### Scenario: Invalid split config is rejected
Given dataset "desk-objects-v1" contains annotated samples
When I create dataset version "bad-split" with train 80 validation 20 and test 20
Then the response status should be 400

### Scenario: CoreML export is available
Given a registered model exists and its artifact is available
When I request an export for the model with format "coreml"
Then the export status should be "succeeded" and include a CoreML artifact URI

### Scenario: Models can be compared
Given two registered models have comparable validation metrics
When I compare the models
Then the response ranks the model with the best validation metric first

### Scenario: Model promotion is exclusive
Given two models exist for the same dataset version and model family
When I promote one model
Then that model status becomes "promoted" and no competing model remains promoted

## Definition of Done

- Application use cases are tested before implementation.
- HTTP routes are covered by route tests.
- Transient repositories preserve model promotion invariants.
- OpenAPI contract includes the new request and response schemas.
- `npm run quality` and `cargo test --manifest-path api/Cargo.toml --workspace` pass.
- TODO P2 checkboxes are updated only for completed items.
