# ADR 0001 - Use Hexagonal Architecture

## Status

Accepted on 2026-06-15.

## Context

PerceptionLab must keep business rules independent from HTTP, SQL, storage, queue, PyTorch, and export tooling.

## Decision

Use lightweight hexagonal architecture:

```text
HTTP / CLI / BDD
-> application use cases
-> domain model and value objects
-> ports
-> adapters
```

Rust implementation is split into `perception_domain`, `perception_app`, `perception_infra`, `perception_http`, and `perception_api` crates.

## Consequences

- Domain code cannot import framework or infrastructure crates.
- Use cases are testable with fake ports.
- Adapters can be tested separately with local PostgreSQL, storage, queue, and worker dependencies.
