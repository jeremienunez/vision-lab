# DB Adapters

Database adapters live here.

Torch imports are forbidden in repositories.

`postgres_job_repository.py` implements the worker-side PostgreSQL adapter for the training job queue. It leases queued jobs, updates job lifecycle state, persists worker metrics, creates a candidate model record, and completes or fails the queue row.
