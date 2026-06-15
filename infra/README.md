# Infrastructure

MVP local stack:

- Rust API service from `api/crates/perception_api`.
- PostgreSQL with `api/migrations/0001_initial_schema.sql` loaded on first boot.
- PostgreSQL-backed queue target through `PERCEPTIONLAB_QUEUE_URL`.
- Local filesystem storage volumes for uploaded samples and model artifacts.
- Docker Compose entrypoint at `compose.yaml`.

The MVP must remain runnable locally and understandable by a technical recruiter in under ten minutes.
