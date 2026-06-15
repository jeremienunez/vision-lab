# ADR 0002 - Use PostgreSQL Backed Queue For MVP

## Status

Proposed on 2026-06-15.

## Context

The MVP needs asynchronous training jobs without overbuilding distributed infrastructure.

## Decision

Prefer a PostgreSQL-backed queue for the first MVP unless the technical pass proves a separate queue is necessary.

## Consequences

- Fewer moving parts for the local Docker Compose demo.
- Worker locking and job state transitions must be tested carefully.
- The design can evolve to a dedicated queue later through the queue port.
