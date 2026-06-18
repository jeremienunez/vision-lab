# Infrastructure

MVP local stack:

- Rust API service from `api/crates/perception_api`.
- PostgreSQL with `api/migrations/0001_initial_schema.sql` loaded on first boot.
- PostgreSQL-backed queue target through `PERCEPTIONLAB_QUEUE_URL`.
- Local filesystem storage volumes for uploaded samples and model artifacts.
- Loki single-node log store with filesystem persistence for local observability.
- Grafana Alloy Docker log collection through the local Docker socket.
- Docker Compose entrypoint at `compose.yaml`.
- Operator entrypoint at `Makefile`.

The MVP must remain runnable locally and understandable by a technical recruiter in under ten minutes.
