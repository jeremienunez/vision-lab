# ADR 0003 - Use Strategy For Training And Inference Modes

## Status

Accepted on 2026-06-15.

## Context

PerceptionLab needs fast deterministic CI behavior and credible local ML execution.

## Decision

Use Strategy pattern for:

- `FakeTrainer`
- `TinyTrainer`
- `YoloTrainer`
- `FakeInferenceEngine`
- `TorchInferenceEngine`
- `OnnxInferenceEngine`
- local and S3 storage
- ONNX and CoreML exporters

## Consequences

- CI can use `fake_training` without changing use cases.
- Local or nightly tests can use `tiny_training`.
- Runtime mode selection happens through typed config and bootstrap, not scattered conditionals.
