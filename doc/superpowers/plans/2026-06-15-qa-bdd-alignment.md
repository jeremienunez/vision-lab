# QA BDD Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align the repository with the PerceptionLab QA & BDD PDF by adding a QA folder, Gherkin feature files, fixtures, traceability docs, and a static BDD validation command.

**Architecture:** Keep executable checks lightweight until the technical pass chooses the final BDD runner. Use Node policy scripts to validate feature-file structure now, while documenting future Docker Compose, Rust, Python, and E2E gates.

**Tech Stack:** Markdown, Gherkin `.feature` files, shell scripts, Node.js native `node:test`.

---

### Task 1: Static BDD Feature Validation

**Files:**
- Create: `tests/unit/bdd-feature-policy.test.mjs`
- Create: `scripts/bdd-feature-policy.mjs`
- Create: `scripts/validate-bdd-features.mjs`
- Modify: `package.json`

- [x] **Step 1: Write the failing test**

```js
import { validateFeatureDocument } from '../../scripts/bdd-feature-policy.mjs';
```

Run:

```bash
node --test tests/unit/bdd-feature-policy.test.mjs
```

Expected: FAIL with `ERR_MODULE_NOT_FOUND`.

- [x] **Step 2: Implement the validator**

Validate that a feature document has at least one tag, one `Feature:`, one `Scenario:`, and at least one `Given`, `When`, and `Then`.

- [x] **Step 3: Add package script**

Add `validate:bdd` and include it in `quality`.

- [x] **Step 4: Verify policy**

Run:

```bash
npm test
```

Expected: PASS.

### Task 2: QA Structure And Feature Files

**Files:**
- Create: `qa/README.md`
- Create: `qa/features/*.feature`
- Create: `qa/features/fixtures/README.md`
- Create: `qa/features/reports/.gitkeep`

- [x] **Step 1: Create feature files**

Create feature files for health, dataset management, sample ingestion, annotation management, dataset versioning, training jobs, metrics tracking, model registry, inference API, model export, visual overlay, artifacts storage, database integrity, observability, performance smoke, standard API errors, worker job locking, and the end-to-end pipeline.

- [x] **Step 2: Validate feature files**

Run:

```bash
npm run validate:bdd
```

Expected: all feature files pass static validation.

### Task 3: QA Documentation

**Files:**
- Create: `doc/quality/qa-bdd.md`
- Create: `doc/quality/test-strategy.md`
- Create: `doc/quality/acceptance-matrix.md`
- Modify: `doc/specs/qa-bdd-spec.md`
- Modify: `doc/README.md`

- [x] **Step 1: Document QA strategy**

Capture test pyramid, criticality levels, quality gates, fake/tiny training modes, ML controls, DB/storage/worker/security checks, and Definition of Done QA.

- [x] **Step 2: Document traceability matrix**

Map product modules to feature files and priorities.

### Task 4: Local Scripts

**Files:**
- Create: `scripts/run_bdd.sh`
- Create: `scripts/seed_demo_dataset.sh`

- [x] **Step 1: Add BDD validation script**

`scripts/run_bdd.sh` runs the static BDD validator now and becomes the future entrypoint for the real BDD runner.

- [x] **Step 2: Add seed script placeholder**

`scripts/seed_demo_dataset.sh` documents the expected future seed command and exits successfully until the API exists.

- [x] **Step 3: Run full quality**

Run:

```bash
npm run quality
```

Expected: tests, sprint docs, BDD feature validation, and architecture validation pass.
