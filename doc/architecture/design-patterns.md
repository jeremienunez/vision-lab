# Design Patterns

This document is the local source of truth for the PerceptionLab design-pattern pass.

## Decision Rule

Use a pattern only when it answers at least one of these questions:

- Which business invariant does it protect?
- Which technical dependency does it isolate?
- Which future evolution does it make cheaper?
- Which test does it make easier to write?
- Which responsibility confusion does it avoid?

Patterns are forbidden when they only make the code look more senior.

## Mandatory Patterns

| Pattern | Role |
| --- | --- |
| Hexagonal Architecture | Separate domain, use cases, ports, and technical adapters. |
| Use Case | Represent each product intention explicitly. |
| Newtype | Prevent ID and primitive confusion. |
| Value Object | Make invalid critical values impossible to construct. |
| Repository | Hide PostgreSQL behind application ports. |
| Unit of Work | Protect multi-write operations with explicit transactions. |
| Strategy | Switch fake, tiny, real training, storage, inference, and export modes. |
| State Machine | Protect job, model, export, and dataset status transitions. |
| DTO + Mapper | Separate HTTP, DB, worker, and public contracts from domain entities. |
| Adapter | Isolate SQLx, storage, PyTorch, ONNX, CoreML, and external errors. |
| Factory / Builder | Build typed config and dependency graphs explicitly. |
| Error Mapping | Convert internal errors into stable public errors. |
| Lightweight Event Log | Trace lifecycle events without full event sourcing in MVP. |

## Mandatory State Machines

Training job:

```text
queued -> running -> succeeded
queued -> running -> failed
queued -> cancelled
```

Model:

```text
candidate -> validated -> promoted -> archived
candidate -> archived
validated -> archived
promoted -> archived
```

Export:

```text
queued -> running -> succeeded
queued -> running -> failed
```

Dataset:

```text
draft -> ready -> archived
```

Dataset versions are immutable immediately after creation.

## Forbidden Anti-Patterns

- God service.
- SQL in handlers.
- Raw Python dict payloads for jobs, inference, or exports.
- `serde_json::Value` in domain code.
- String statuses instead of enums and transitions.
- Mutable global singleton.
- Vague `utils`, `helpers`, `misc`, `common`, `manager`, or `service` files.
- Framework-driven domain.
- Mock-only E2E tests that never exercise API, DB, storage, and worker boundaries.

Any exception requires an ADR.
