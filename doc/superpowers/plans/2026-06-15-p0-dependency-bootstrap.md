# P0 Dependency Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create the installable P0 foundation: Rust workspace, Python worker package, local absolute path config, and verification commands.

**Architecture:** Keep this tranche focused on dependency/bootstrap readiness, not business endpoints. Rust crates follow the prescribed hexagonal split under `api/crates/`; the worker becomes an installable Python package under `worker/`; local machine paths are generated into ignored `.env.local` while portable defaults stay in `.env.example`.

**Tech Stack:** Cargo workspace, Rust 2024, Axum, SQLx, Tokio, Utoipa, Python 3.12, uv, Pydantic, Typer, Pytest, local filesystem paths under `/home/jerem/vision-lab`.

---

### Task 1: P0 Bootstrap Policy

**Files:**
- Create: `tests/unit/p0-bootstrap-policy.test.mjs`
- Create: `scripts/p0-bootstrap-policy.mjs`
- Create: `scripts/validate-p0-bootstrap.mjs`
- Modify: `package.json`

- [x] **Step 1: Write the failing test**

```js
import { describe, it } from 'node:test';
import assert from 'node:assert/strict';

import {
  buildLocalEnvContent,
  requiredP0BootstrapPaths,
  validateP0BootstrapPaths,
} from '../../scripts/p0-bootstrap-policy.mjs';

describe('P0 bootstrap policy', () => {
  it('requires installable Rust, worker, and local env bootstrap files', () => {
    const result = validateP0BootstrapPaths(requiredP0BootstrapPaths);

    assert.equal(result.valid, true);
    assert.deepEqual(result.errors, []);
  });

  it('rejects missing Cargo workspace and worker package files', () => {
    const result = validateP0BootstrapPaths(['README.md']);

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /api\/Cargo.toml/);
    assert.match(result.errors.join('\n'), /worker\/pyproject.toml/);
  });

  it('builds absolute local paths for Ubuntu filesystem execution', () => {
    const envContent = buildLocalEnvContent('/home/jerem/vision-lab');

    assert.match(envContent, /PERCEPTIONLAB_PROJECT_ROOT=\/home\/jerem\/vision-lab/);
    assert.match(envContent, /PERCEPTIONLAB_DATA_ROOT=\/home\/jerem\/vision-lab\/datasets/);
    assert.match(envContent, /PERCEPTIONLAB_STORAGE_ROOT=\/home\/jerem\/vision-lab\/\.perceptionlab\/storage/);
    assert.doesNotMatch(envContent, /\.\.\//);
  });
});
```

Run: `node --test tests/unit/p0-bootstrap-policy.test.mjs`
Expected: FAIL with `ERR_MODULE_NOT_FOUND`.

- [x] **Step 2: Implement the policy**

Create `scripts/p0-bootstrap-policy.mjs` with `requiredP0BootstrapPaths`, `validateP0BootstrapPaths(paths)`, and `buildLocalEnvContent(projectRoot)`.

- [x] **Step 3: Add validator command**

Create `scripts/validate-p0-bootstrap.mjs` that scans the repository and fails when required P0 bootstrap files are missing.

Add `validate:p0-bootstrap` to `package.json` and include it in `quality`.

### Task 2: Rust Workspace Dependencies

**Files:**
- Create: `api/Cargo.toml`
- Create: `api/crates/perception_domain/Cargo.toml`
- Create: `api/crates/perception_domain/src/lib.rs`
- Create: `api/crates/perception_app/Cargo.toml`
- Create: `api/crates/perception_app/src/lib.rs`
- Create: `api/crates/perception_infra/Cargo.toml`
- Create: `api/crates/perception_infra/src/lib.rs`
- Create: `api/crates/perception_http/Cargo.toml`
- Create: `api/crates/perception_http/src/lib.rs`
- Create: `api/crates/perception_api/Cargo.toml`
- Create: `api/crates/perception_api/src/main.rs`
- Modify: `.gitignore`
- Modify: `package.json`

- [x] **Step 1: Create manifests**

Use workspace dependencies for `axum`, `tokio`, `serde`, `serde_json`, `sqlx`, `uuid`, `chrono`, `thiserror`, `async-trait`, `tracing`, `tracing-subscriber`, `tower-http`, `utoipa`, `config`, and `dotenvy`.

- [x] **Step 2: Create minimal crate roots**

Each library crate has a `src/lib.rs` with `#![forbid(unsafe_code)]`. The binary crate has `src/main.rs` and initializes tracing.

- [x] **Step 3: Add Rust check**

Add `check:rust` to `package.json`:

```json
"check:rust": "cargo check --manifest-path api/Cargo.toml --workspace"
```

Run: `npm run check:rust`
Expected: Cargo resolves dependencies and workspace check passes.

### Task 3: Python Worker Dependencies

**Files:**
- Create: `worker/pyproject.toml`
- Create: `worker/perception_worker/__init__.py`
- Create: `worker/tests/test_worker_package.py`
- Modify: `.gitignore`
- Modify: `package.json`

- [x] **Step 1: Create worker package manifest**

Declare Python `>=3.12,<3.13`, runtime dependencies `pydantic`, `pydantic-settings`, `typer`, `rich`, `structlog`, `sqlalchemy`, `psycopg[binary]`, `pillow`, and `numpy`. Declare optional ML dependencies `torch`, `torchvision`, and `ultralytics` under the `ml` extra.

- [x] **Step 2: Create import test**

`worker/tests/test_worker_package.py` imports `perception_worker` and asserts `__version__ == "0.1.0"`.

- [x] **Step 3: Add worker check**

Add `check:worker` to `package.json`:

```json
"check:worker": "cd worker && uv run pytest"
```

Run: `npm run check:worker`
Expected: uv creates the worker environment and pytest passes.

### Task 4: Local Absolute Path Bootstrap

**Files:**
- Create: `.env.example`
- Create: `scripts/bootstrap-local-env.mjs`
- Modify: `datasets/seed/README.md`
- Modify: `README.md`
- Modify: `.gitignore`
- Modify: `package.json`

- [x] **Step 1: Add portable env example**

`.env.example` documents all required path variables and service URLs.

- [x] **Step 2: Generate ignored local env**

`scripts/bootstrap-local-env.mjs` writes `.env.local` with absolute paths under `/home/jerem/vision-lab`, and creates `.perceptionlab/storage`, `.perceptionlab/artifacts`, `.perceptionlab/tmp`, and `datasets/seed`.

- [x] **Step 3: Add bootstrap command**

Add `bootstrap:env` to `package.json`:

```json
"bootstrap:env": "node scripts/bootstrap-local-env.mjs"
```

Run: `npm run bootstrap:env`
Expected: `.env.local` exists locally and contains absolute paths.

### Task 5: Verification And Commit

**Files:**
- Modify: `TODO.md`
- Modify: `CHANGELOG.md`

- [x] **Step 1: Verify**

Run:

```bash
npm run quality
npm run check:rust
npm run check:worker
git diff --check
```

Expected: all commands pass.

- [ ] **Step 2: Commit**

Run:

```bash
git add -A
git commit -m "chore(p0): bootstrap dependencies and local paths"
```

Expected: pre-commit quality gate passes and the commit is created.
