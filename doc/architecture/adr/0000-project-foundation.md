# ADR 0000 - Project Foundation

## Status

Accepted on 2026-06-15.

## Context

The project starts from Product Owning, QA/BDD, and design-pattern reference PDFs. The Product Owning source defines PerceptionLab as a Rust + PyTorch ML infrastructure platform, not a generic image analysis lab.

## Decision

Use a lightweight Node-based toolchain for local validation:

- Native `node:test` for policy script tests.
- Dependency Cruiser for architecture boundaries.
- Versioned Git hooks through `.githooks/`.
- Markdown specs and sprint plans under `doc/`.

For implementation planning, use this product shape:

- Rust API service.
- Python/PyTorch worker.
- PostgreSQL.
- Object storage or local storage adapter.
- Queue-backed training jobs.
- Docker Compose local stack.

## Consequences

- Contributors can run one command: `npm run quality`.
- Architecture boundaries are documented and executable.
- Git hooks are versioned and reproducible.
- The first product implementation can start with explicit sprint criteria.
- Rust and Python validation tooling must be added during the technical pass.
