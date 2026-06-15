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

### Changed

- Reframed the project from generic Vision Lab foundation to PerceptionLab, a Rust + PyTorch ML infrastructure platform.
- Aligned product specs, roadmap, sprints, and validation criteria with the Product Owning reference PDF.
- Aligned QA documentation and feature coverage with the QA & BDD reference PDF.
- Aligned architecture documentation and implementation roots with the Design Patterns & Conventions reference PDF.
- Removed the obsolete root `src/` foundation layout so P0 starts from `api/crates/` and `worker/perception_worker/`.
- Extended local quality gates to validate P0 bootstrap files, Rust workspace checks, and worker package tests.
- Excluded internal agent planning docs from Git tracking and future adds before publishing.
