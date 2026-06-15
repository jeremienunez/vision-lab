# Design Pattern Conventions Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align the repository with the Design Patterns & Conventions PDF before starting P0 implementation.

**Architecture:** Make the folder structure express the hexagonal architecture directly. Add a static convention validator so legacy folders, vague filenames, and missing required architecture paths fail before P0 work begins.

**Tech Stack:** Markdown docs, Node.js policy scripts, shell scripts, planned Rust crates, planned Python/PyTorch worker.

---

### Task 1: Architecture Convention Guard

**Files:**
- Create: `tests/unit/architecture-conventions-policy.test.mjs`
- Create: `scripts/architecture-conventions-policy.mjs`
- Create: `scripts/validate-architecture-conventions.mjs`
- Modify: `package.json`

- [x] **Step 1: Write failing test**

Run: `node --test tests/unit/architecture-conventions-policy.test.mjs`
Expected: FAIL with `ERR_MODULE_NOT_FOUND`.

- [x] **Step 2: Implement validator**

Validate required Rust, worker, QA, contracts, design-pattern docs, and ADR paths. Reject legacy folders and vague names.

- [x] **Step 3: Add quality script**

Add `validate:conventions` to `npm run quality`.

### Task 2: Normative Folder Structure

**Files:**
- Create: `api/crates/perception_domain/src/README.md`
- Create: `api/crates/perception_app/src/ports/README.md`
- Create: `api/crates/perception_app/src/use_cases/README.md`
- Create: `api/crates/perception_infra/src/postgres/README.md`
- Create: `api/crates/perception_infra/src/storage/README.md`
- Create: `api/crates/perception_infra/src/queue/README.md`
- Create: `api/crates/perception_http/src/routes/README.md`
- Create: `api/crates/perception_http/src/dto/README.md`
- Create: `api/crates/perception_http/src/mappers/README.md`
- Create: `worker/perception_worker/README.md`
- Create: `contracts/README.md`
- Delete: `apps/api-rust/README.md`
- Delete: `workers/pytorch-trainer/README.md`

- [x] **Step 1: Create prescribed structure**

Use `api/crates/perception_*` and `worker/perception_worker` as the implementation roots.

- [x] **Step 2: Remove legacy structure**

Remove `apps/api-rust` and `workers/pytorch-trainer`.

### Task 3: Architecture Documentation

**Files:**
- Create: `doc/architecture/design-patterns.md`
- Create: `doc/architecture/review-checklist.md`
- Create: `doc/architecture/adr/0001-use-hexagonal-architecture.md`
- Create: `doc/architecture/adr/0002-use-postgresql-backed-queue-for-mvp.md`
- Create: `doc/architecture/adr/0003-use-strategy-for-training-and-inference-modes.md`
- Modify: `doc/specs/architecture-spec.md`
- Modify: `doc/architecture/conventions.md`
- Modify: `README.md`

- [x] **Step 1: Document mandatory patterns**

Capture hexagonal architecture, use cases, newtypes, value objects, repositories, unit of work, strategy, state machines, DTO/mappers, adapters, factories, error mapping, and lightweight event logs.

- [x] **Step 2: Document forbidden anti-patterns**

Block god services, SQL in handlers, raw worker dicts, string statuses, `serde_json::Value` in domain, mutable singletons, and vague file names.

### Task 4: Verification

**Files:**
- Modify: `CHANGELOG.md`
- Modify: `TODO.md`

- [x] **Step 1: Run convention validation**

Run: `npm run validate:conventions`
Expected: PASS.

- [x] **Step 2: Run full quality**

Run: `npm run quality`
Expected: PASS.
