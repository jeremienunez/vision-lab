# Sprint 05 - Portfolio Polish

## Goal

Make the repository understandable and compelling as a technical portfolio asset in under ten minutes.

## Priority

P1

## Dependencies

- Sprint 04 model registry and inference complete.
- End-to-end local demo works with seed data.
- Demo artifacts can be generated.

## Scope

- Write a premium README as a product-technical page.
- Add architecture diagram.
- Add curl examples for the full demo path.
- Add demo image or GIF.
- Add benchmark notes for upload, training mode, inference latency, and ONNX export.
- Add model card for the demo model.
- Document technical decisions and roadmap.

## BDD Validation Criteria

### Scenario: Recruiter understands the project quickly
Given a technical recruiter opens the repository
When they read the README for less than ten minutes
Then they can identify the Rust API, PyTorch worker, dataset versioning, training jobs, model registry, inference API, and ONNX export

### Scenario: Demo path is reproducible
Given a developer has Docker installed
When they follow the README quickstart
Then they can start the stack and execute the documented curl flow

### Scenario: Visual result is visible
Given the demo inference has run
When the overlay generation step completes
Then `doc/demo/output_overlay.svg` shows boxes, class labels, confidence scores, and optional distance metadata

### Scenario: Technical choices are explainable
Given an interviewer asks why Rust and Python are both used
When they read the technical decisions document
Then they see Rust assigned to API/orchestration and Python/PyTorch assigned to ML execution

## Definition of Done

- README follows the expected product-technical structure.
- Architecture diagram is present or generated from source.
- Curl examples cover dataset creation through inference.
- Demo artifact paths are documented.
- Benchmarks include inference latency.
- Roadmap and technical decisions are current.
