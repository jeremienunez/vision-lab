# Changelog

All notable changes to this project will be documented in this file.

The format follows Keep a Changelog principles, and versioning should follow SemVer once releases start.

## [0.1.0] - 2026-06-15

### Added

- Initial project structure.
- Documentation workspace under `doc/`.
- Reference PDFs moved to `doc/references/`.
- Product, architecture, QA, and sprint specifications.
- BDD sprint validation rules.
- Versioned Git hooks in `.githooks/`.
- Commit message validation for Conventional Commits.
- Dependency Cruiser architecture boundaries.
- Node test suite for local policy scripts.
- QA/BDD feature structure, fixtures, traceability docs, and static BDD validation command.
- Design-pattern convention guard, normative Rust/Python folder structure, contracts folder, architecture ADRs, and review checklist.
- Pre-P0 coverage audit across Product Owning, QA/BDD, and Design Patterns references.
- BDD coverage for ML pipeline consistency and basic API security.
- P0 dependency bootstrap with Cargo workspace, Python worker package, CPU PyTorch/Ultralytics sync, local absolute path generation, and validation commands.
- Rust API healthcheck route with Axum router, JSON dependency status, and executable local server entrypoint.
- Pure Rust domain foundation with typed IDs, normalized bbox validation, image dimensions, training hyperparameters, and lifecycle state machines.
- Application-layer dataset creation use case behind a repository port.
- Initial PostgreSQL schema for datasets, annotations, immutable dataset versions, training jobs, queue rows, metrics, model registry, exports, inference runs, and artifacts.
- Dataset creation and listing HTTP routes backed by the application repository port and a local transient repository adapter.
- Sample upload use case and multipart HTTP route with abstracted sample repository and local file storage adapters.
- Annotation add/list use cases and HTTP routes with dataset class and normalized bbox validation.
- Dataset stats use case and HTTP route reporting sample count, annotation count, and annotations by class.
- Dataset version creation use case and HTTP route capturing immutable dataset snapshot metadata.
- Training job creation use case and HTTP route queuing jobs from immutable dataset versions.
- Training job lifecycle transition use case with persisted status updates and guarded state transitions.
- Queue-backed training orchestration port with transient queue adapter, enqueue-on-create, and lease-next behavior.
- Minimal Python worker processor with strict Pydantic training-job contracts, repository/trainer ports, and a fake training adapter for deterministic orchestration tests.
- Tiny deterministic PyTorch trainer strategy that runs a CPU training loop, writes a local model artifact, and returns training metrics.
- Training metric application use cases, repository port, domain id, and transient repository for persisted epoch metric records.
- Training job metrics HTTP route returning persisted metric records ordered by epoch and step.
- Minimal model registry use cases and transient repository for registering candidate models from succeeded training jobs.
- Model registry HTTP routes for listing registered models and reading model detail.
- Inference application use case, strategy port, and deterministic fake inference engine for local MVP execution.
- Multipart model inference HTTP route with confidence filtering and unsupported media type handling.
- Docker Compose local stack for the Rust API and PostgreSQL schema bootstrap.
- Product-grade README quickstart covering installation, quality gates, direct API run, Docker Compose, and P0 curl examples.
- YOLO annotation export application use case that emits `classes.txt` and per-sample label files with normalized center-based boxes.

### Changed

- Reframed the project from generic Vision Lab foundation to PerceptionLab, a Rust + PyTorch ML infrastructure platform.
- Aligned product specs, roadmap, sprints, and validation criteria with the Product Owning reference PDF.
- Aligned QA documentation and feature coverage with the QA & BDD reference PDF.
- Aligned architecture documentation and implementation roots with the Design Patterns & Conventions reference PDF.
- Removed the obsolete root `src/` foundation layout so P0 starts from `api/crates/` and `worker/perception_worker/`.
- Extended local quality gates to validate P0 bootstrap files, Rust workspace checks, and worker package tests.
- Excluded internal agent planning docs from Git tracking and future adds before publishing.
