# Foundation Quality Gates Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the repository foundation, documentation structure, sprint criteria, and local Git quality gates.

**Architecture:** The repository uses documentation-first planning and a layered source layout. Local policy scripts validate commit message shape and sprint BDD completeness, while Dependency Cruiser enforces source dependency direction.

**Tech Stack:** Node.js 24, native `node:test`, Dependency Cruiser, Git hooks, Markdown documentation.

---

## File Structure

- Create: `TODO.md` for tracked backlog.
- Create: `CHANGELOG.md` for release history.
- Create: `doc/` for specs, sprints, architecture notes, quality rules, and PDF references.
- Create: `src/domain`, `src/application`, `src/infrastructure`, `src/presentation` for future implementation.
- Create: `tests/unit`, `tests/integration`, `tests/contract` for test levels.
- Create: `scripts/commit-message-policy.mjs` for Conventional Commit validation.
- Create: `scripts/sprint-document-policy.mjs` for BDD sprint validation.
- Create: `.githooks/pre-commit` and `.githooks/commit-msg` for local checks.
- Create: `.dependency-cruiser.cjs` for architecture boundaries.

### Task 1: Documentation Foundation

**Files:**
- Create: `TODO.md`
- Create: `CHANGELOG.md`
- Create: `doc/README.md`
- Create: `doc/specs/product-spec.md`
- Create: `doc/specs/architecture-spec.md`
- Create: `doc/specs/qa-bdd-spec.md`

- [ ] **Step 1: Create documentation entry points**

Add root project documentation and a docs index that points to specs, sprints, architecture, quality, and references.

- [ ] **Step 2: Move references**

Run:

```bash
mv perceptionlab_*.pdf doc/references/
```

Expected: the three PDF files are under `doc/references/`.

- [ ] **Step 3: Verify docs exist**

Run:

```bash
find doc -maxdepth 3 -type f | sort
```

Expected: specs, quality docs, sprint docs, ADR, and reference PDFs are listed.

### Task 2: Policy Scripts With TDD

**Files:**
- Create: `tests/unit/commit-message-policy.test.mjs`
- Create: `tests/unit/sprint-document-policy.test.mjs`
- Create: `scripts/commit-message-policy.mjs`
- Create: `scripts/sprint-document-policy.mjs`
- Create: `scripts/validate-commit-msg.mjs`
- Create: `scripts/validate-sprint-docs.mjs`

- [ ] **Step 1: Write failing commit policy test**

```js
import { validateCommitMessage } from '../../scripts/commit-message-policy.mjs';
```

Run:

```bash
node --test tests/unit/*.test.mjs
```

Expected: FAIL with `ERR_MODULE_NOT_FOUND`.

- [ ] **Step 2: Implement commit policy**

Create a validator that accepts `type(scope): subject`, rejects unsupported types, and keeps the first line within 72 characters.

- [ ] **Step 3: Implement sprint policy**

Create a validator that requires `Goal`, `Scope`, `BDD Validation Criteria`, `Given`, `When`, `Then`, and `Definition of Done`.

- [ ] **Step 4: Verify tests pass**

Run:

```bash
node --test tests/unit/*.test.mjs
```

Expected: PASS.

### Task 3: Local Quality Gates

**Files:**
- Create: `package.json`
- Create: `.dependency-cruiser.cjs`
- Create: `.githooks/pre-commit`
- Create: `.githooks/commit-msg`

- [ ] **Step 1: Add package scripts**

Add scripts for `test`, `validate:docs`, `lint:architecture`, `quality`, and `prepare:hooks`.

- [ ] **Step 2: Add Dependency Cruiser rules**

Forbid domain imports from higher layers, application imports from adapters, presentation imports from infrastructure, and circular dependencies.

- [ ] **Step 3: Add hooks**

`pre-commit` runs `npm run quality`. `commit-msg` runs `node scripts/validate-commit-msg.mjs "$1"`.

- [ ] **Step 4: Verify local gates**

Run:

```bash
npm run quality
```

Expected: tests, sprint validation, and architecture validation pass.
