# Pre-P0 Coverage Audit

Date: 2026-06-15

## Sources Reviewed

- `doc/references/perceptionlab_product_owning.pdf`
- `doc/references/perceptionlab_qa_bdd.pdf`
- `doc/references/perceptionlab_design_patterns_conventions.pdf`

## Verdict

The preparatory documentation is covered for starting P0. Product scope, QA acceptance, and architecture conventions are represented by persistent docs, folder structure, static validators, and quality scripts.

## Coverage Matrix

| Source | Coverage |
| --- | --- |
| Product Owning | `doc/specs/product-spec.md`, `doc/specs/modules.md`, `doc/specs/api-spec.md`, `doc/specs/domain-model.md`, `doc/specs/requirements.md`, `doc/specs/roadmap.md`, `doc/sprints/`, `TODO.md` |
| QA & BDD | `doc/quality/qa-bdd.md`, `doc/quality/test-strategy.md`, `doc/quality/acceptance-matrix.md`, `qa/features/*.feature`, `qa/fixtures/`, `scripts/run_bdd.sh`, `npm run validate:bdd` |
| Design Patterns & Conventions | `api/crates/`, `worker/perception_worker/`, `contracts/`, `doc/architecture/design-patterns.md`, `doc/architecture/review-checklist.md`, ADRs, `npm run validate:conventions` |

## Final Pre-P0 Gaps Closed

- Added BDD coverage for ML pipeline consistency.
- Added BDD coverage for basic API security.
- Consolidated fixtures under `qa/fixtures/`.
- Removed the obsolete root `src/` foundation notes so `api/crates/` and `worker/perception_worker/` remain the only implementation roots.
- Updated the architecture convention guard to reject reintroduction of the obsolete root `src/` layout.

## Residual Technical Gates For P0

- Cargo workspace checks must be added when Rust crates are created.
- Rust import boundaries must be enforced once crate code exists.
- Python linting, typing, and tests must be added once `worker/perception_worker` becomes executable.
- The final BDD runner and step definitions must be selected during the technical QA pass.
