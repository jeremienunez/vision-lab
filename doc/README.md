# Documentation Index

PerceptionLab documentation is organized around the Product Owning reference: product intent first, then architecture, QA, API contracts, roadmap, sprint execution, frontend product surfaces, experimental ML exploration, and mobile deployment compression.

## References

The initial PDF references are stored in `doc/references/`:

- `perceptionlab_product_owning.pdf`
- `perceptionlab_qa_bdd.pdf`
- `perceptionlab_design_patterns_conventions.pdf`

## Working Documents

- `doc/specs/product-spec.md` - product scope and expected outcomes.
- `doc/specs/architecture-spec.md` - technical boundaries, SOLID, and design pattern rules.
- `doc/specs/api-spec.md` - expected API endpoints and example contracts.
- `doc/specs/domain-model.md` - business objects, statuses, relations, and rules.
- `doc/specs/modules.md` - product modules A-J and acceptance signals.
- `doc/specs/requirements.md` - functional and non-functional requirements.
- `doc/specs/roadmap.md` - MVP and portfolio roadmap.
- `doc/specs/p2-spec.md` - P2 advanced platform scope and execution slices.
- `doc/specs/demo-spec.md` - final demo target and portfolio proof.
- `doc/specs/technical-pass-questions.md` - questions intentionally left for the technical pass.
- `doc/specs/qa-bdd-spec.md` - QA strategy and BDD format.
- `doc/frontend/frontend-architecture.md` - current `web/` implementation root, frontend architecture, route map, API client, live camera integration, and remaining implementation order.
- `doc/frontend/live-camera-spec.md` - browser camera modes, frame capture, overlay mapping, backpressure, and BDD scenarios.
- `doc/design/graphic-tokens.md` - design token reference plus the current `web/src/index.css` Tailwind token implementation note.
- `doc/research/experimental-finetuning-pass.md` - fine-tuning experiment protocol, dataset tracks, experiment matrix, metrics, artifacts, and model card template.
- `doc/research/mobile-quantization-pass.md` - mobile quantization, compression, teacher-student distillation, QAT, calibration, mobile benchmarking, and deployment readiness.
- `doc/sprints/` - sprint plans with validation criteria.
- `doc/quality/` - definition of done and BDD validation checklist.
- `doc/quality/qa-bdd.md` - QA/BDD strategy extracted from the QA reference PDF.
- `doc/quality/test-strategy.md` - test pyramid, gates, and execution modes.
- `doc/quality/acceptance-matrix.md` - product module to feature-file traceability.
- `doc/quality/pre-p0-coverage-audit.md` - preparatory PDF coverage check before P0 implementation.
- `doc/architecture/` - conventions and ADRs.
- `doc/architecture/design-patterns.md` - mandatory patterns and forbidden anti-patterns.
- `doc/architecture/review-checklist.md` - review checklist before P0 implementation.
- `doc/demo/` - planned demo inputs, overlays, and API response examples.
- `qa/features/` - Gherkin acceptance scenarios in English, including the executable `@smoke` subset.
- `qa/fixtures/` - planned test fixtures mirrored from the QA/BDD reference.
- `contracts/` - OpenAPI and JSON schemas for API and worker contracts.
