# Definition Of Ready And Done

## Definition Of Ready

A task is ready when it has:

- Clear objective.
- Target endpoint or component.
- Expected input.
- Expected output.
- Acceptance criteria.
- Priority.
- Identified dependency.

Example for `POST /datasets`:

- Objective: create an `object_detection` dataset.
- Input: `name`, `description`, `task_type`, `classes`.
- Output: `dataset_id`, `status`, `created_at`.
- Acceptance criteria: returns `201` for valid payload, rejects empty name, rejects unknown task type, persists in PostgreSQL, has an integration test.
- Priority: P0.
- Dependency: API foundation and database migration available.

## Definition Of Done

A sprint is done when:

- BDD validation criteria are written and reviewed.
- Tests cover new behavior before implementation.
- Main errors are handled with readable messages.
- Endpoint or component documentation is updated.
- Useful logs exist for important operations.
- Acceptance criteria are respected.
- `npm test` passes.
- `npm run validate:docs` passes.
- `npm run lint:architecture` passes.
- `CHANGELOG.md` includes user-visible changes.
- SOLID and design pattern constraints are respected.
- Architecture dependency direction remains clear.

## Definition Of Done For ML Tasks

- Script works on the seed dataset.
- Artifacts are saved.
- Metrics are produced.
- Errors are explicit.
- Dependencies are documented.
