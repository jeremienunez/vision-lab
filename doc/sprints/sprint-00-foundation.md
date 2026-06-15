# Sprint 00 - Foundation

## Goal

Create the repository foundation, documentation structure, and local quality gates.

## Priority

P0

## Dependencies

- Product Owning PDF available in `doc/references/`.

## Scope

- Create `TODO.md` and `CHANGELOG.md`.
- Move PDF references to `doc/references/`.
- Create base folders for source, tests, docs, scripts, and hooks.
- Add specs for product, architecture, QA, and BDD.
- Add Git hooks for commit message and pre-commit quality validation.
- Align the foundation with PerceptionLab as Rust + PyTorch ML infrastructure.

## BDD Validation Criteria

### Scenario: Repository foundation is ready
Given a new contributor opens the repository
When they inspect the root folders
Then they can find docs, source folders, tests, scripts, and hooks

### Scenario: References are grouped
Given the initial PDF references exist
When the project structure is created
Then the PDF files are stored under `doc/references/`

### Scenario: Local quality gate runs
Given dependencies are installed
When `npm run quality` is executed
Then tests, sprint document validation, and architecture validation pass

### Scenario: Product owning direction is visible
Given a technical recruiter opens the repository
When they read the README and product spec
Then they understand this is ML infrastructure, not a simple computer vision model demo

## Definition of Done

- `npm test` passes.
- `npm run validate:docs` passes.
- `npm run lint:architecture` passes.
- `.githooks/pre-commit` and `.githooks/commit-msg` are executable.
- `git config core.hooksPath` is set to `.githooks`.
- Product docs use `PerceptionLab` and the Rust + PyTorch platform promise.
