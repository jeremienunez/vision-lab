# Product Owning Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align repository documentation, sprint planning, and local validation with the PerceptionLab Product Owning PDF.

**Architecture:** Keep the current Node tooling for documentation and hook validation. Reframe product planning around a Rust API service, Python/PyTorch worker, PostgreSQL, storage abstraction, queue-backed training jobs, model registry, inference API, and ONNX export.

**Tech Stack:** Markdown docs, Node.js policy scripts, native `node:test`, Dependency Cruiser, planned Rust API, planned Python/PyTorch worker.

---

### Task 1: Product Reframe

**Files:**
- Modify: `README.md`
- Modify: `TODO.md`
- Modify: `CHANGELOG.md`
- Modify: `doc/README.md`
- Modify: `doc/specs/product-spec.md`

- [x] **Step 1: Replace generic foundation wording**

Use `PerceptionLab` as the product name and describe the core flow as `Upload data -> build dataset -> launch training -> track metrics -> export model -> run inference`.

- [x] **Step 2: Update MVP scope**

Document datasets, samples, annotations, dataset versions, training jobs, metrics, model registry, inference, ONNX export, Docker Compose, PostgreSQL, and storage.

- [x] **Step 3: Verify documentation index**

Run: `find doc -maxdepth 3 -type f | sort`
Expected: product, architecture, API, domain, roadmap, demo, sprints, quality docs, and references are listed.

### Task 2: Sprint Readiness Validation

**Files:**
- Modify: `tests/unit/sprint-document-policy.test.mjs`
- Modify: `scripts/sprint-document-policy.mjs`
- Modify: `doc/quality/sprint-validation-bdd.md`

- [x] **Step 1: Add failing test**

Run: `node --test tests/unit/sprint-document-policy.test.mjs`
Expected: FAIL when a sprint lacks `Priority` and `Dependencies`.

- [x] **Step 2: Implement policy**

Require `## Priority` and `## Dependencies` in every sprint document.

- [x] **Step 3: Verify policy test**

Run: `node --test tests/unit/sprint-document-policy.test.mjs`
Expected: PASS.

### Task 3: Roadmap And Sprint Rewrite

**Files:**
- Modify: `doc/sprints/sprint-00-foundation.md`
- Create: `doc/sprints/sprint-01-api-foundation.md`
- Create: `doc/sprints/sprint-02-annotation-versioning.md`
- Create: `doc/sprints/sprint-03-training-pipeline.md`
- Create: `doc/sprints/sprint-04-registry-inference.md`
- Create: `doc/sprints/sprint-05-portfolio-polish.md`
- Delete: `doc/sprints/sprint-01-mvp-vision-pipeline.md`
- Delete: `doc/sprints/sprint-02-quality-automation.md`

- [x] **Step 1: Replace generic sprints**

Use the five-week Product Owning roadmap as the sprint sequence.

- [x] **Step 2: Validate BDD readiness**

Run: `npm run validate:docs`
Expected: all sprint files pass goal, priority, dependencies, scope, BDD, and done checks.

### Task 4: Final Quality Gate

**Files:**
- Modify: `package.json`
- Create: `api/README.md`
- Create: `worker/README.md`
- Create: `infra/README.md`
- Create: `datasets/seed/README.md`
- Create: `doc/demo/README.md`

- [x] **Step 1: Add planned implementation folders**

Create placeholder README files that describe the Rust API, PyTorch worker, infrastructure, seed dataset, and demo artifacts.

- [x] **Step 2: Run full quality gate**

Run: `npm run quality`
Expected: tests pass, sprint docs validate, and Dependency Cruiser reports no violations.
