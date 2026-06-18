import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

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
    assert.match(envContent, /PERCEPTIONLAB_API_ADDR=127\.0\.0\.1:8080/);
    assert.match(envContent, /PERCEPTIONLAB_DATA_ROOT=\/home\/jerem\/vision-lab\/datasets/);
    assert.match(
      envContent,
      /PERCEPTIONLAB_STORAGE_ROOT=\/home\/jerem\/vision-lab\/\.perceptionlab\/storage/,
    );
    assert.doesNotMatch(envContent, /\.\.\//);
  });

  it('keeps uv cache inside the project for worker checks', () => {
    const packageJson = JSON.parse(readFileSync('package.json', 'utf8'));

    assert.match(
      packageJson.scripts['check:worker'],
      /UV_CACHE_DIR=\.\.\/\.perceptionlab\/cache\/uv/,
    );
  });

  it('routes PyTorch worker dependencies to the CPU wheel index', () => {
    const pyproject = readFileSync('worker/pyproject.toml', 'utf8');

    assert.match(pyproject, /\[tool\.uv\.sources\]/);
    assert.match(pyproject, /torch = \{ index = "pytorch-cpu" \}/);
    assert.match(pyproject, /torchvision = \{ index = "pytorch-cpu" \}/);
    assert.match(pyproject, /url = "https:\/\/download\.pytorch\.org\/whl\/cpu"/);
  });

  it('exposes explicit dependency installation commands', () => {
    const packageJson = JSON.parse(readFileSync('package.json', 'utf8'));

    assert.equal(
      packageJson.scripts['install:rust'],
      'cargo fetch --manifest-path api/Cargo.toml',
    );
    assert.match(packageJson.scripts['install:worker:ml'], /uv sync --extra ml --group dev/);
    assert.match(packageJson.scripts['install:deps'], /npm run bootstrap:env/);
    assert.match(packageJson.scripts['install:deps'], /npm run install:rust/);
    assert.match(packageJson.scripts['install:deps'], /npm run install:worker:ml/);
  });

  it('requires Docker Compose services for the local P0 stack', () => {
    const compose = readFileSync('compose.yaml', 'utf8');

    assert.match(compose, /^services:/m);
    assert.match(compose, /^  postgres:/m);
    assert.match(compose, /^  api:/m);
    assert.match(compose, /PERCEPTIONLAB_API_ADDR=0\.0\.0\.0:8080/);
  });

  it('requires local Loki observability bootstrap files', () => {
    assert.ok(requiredP0BootstrapPaths.includes('Makefile'));
    assert.ok(requiredP0BootstrapPaths.includes('infra/loki/loki-config.yaml'));
    assert.ok(requiredP0BootstrapPaths.includes('infra/loki/alloy-config.alloy'));
  });

  it('requires Docker Compose services for local log collection', () => {
    const compose = readFileSync('compose.yaml', 'utf8');

    assert.match(compose, /^  loki:/m);
    assert.match(compose, /grafana\/loki:3\.7\.0/);
    assert.match(compose, /^  alloy:/m);
    assert.match(compose, /grafana\/alloy:/);
    assert.match(compose, /--storage\.path=\/var\/lib\/alloy\/data/);
    assert.match(compose, /\/var\/run\/docker\.sock:\/var\/run\/docker\.sock:ro/);
    assert.match(compose, /alloy-data:\/var\/lib\/alloy/);
    assert.match(compose, /^  alloy-data:/m);
  });

  it('keeps local Docker log collection scoped to the PerceptionLab Compose project', () => {
    const alloyConfig = readFileSync('infra/loki/alloy-config.alloy', 'utf8');

    assert.match(alloyConfig, /targets = discovery\.docker\.perceptionlab\.targets/);
    assert.match(alloyConfig, /targets\s+= discovery\.relabel\.perceptionlab_logs\.output/);
    assert.match(alloyConfig, /source_labels = \["__meta_docker_container_label_com_docker_compose_project"\]/);
    assert.match(alloyConfig, /regex\s+= "perceptionlab"/);
    assert.match(alloyConfig, /action\s+= "keep"/);
  });

  it('exposes Make targets for the full local stack and Loki checks', () => {
    const makefile = readFileSync('Makefile', 'utf8');

    assert.match(makefile, /^up:/m);
    assert.match(makefile, /^infra:/m);
    assert.match(makefile, /^loki-ready:/m);
    assert.match(makefile, /^loki-query:/m);
    assert.match(makefile, /docker compose/);
    assert.match(makefile, /npm run web:dev/);
  });

  it('exposes a host YOLO API target for real camera inference', () => {
    const makefile = readFileSync('Makefile', 'utf8');

    assert.match(makefile, /^api-real:/m);
    assert.match(makefile, /\$\(COMPOSE\) stop api/);
    assert.match(makefile, /PERCEPTIONLAB_INFERENCE_ENGINE=yolo_cli/);
    assert.match(
      makefile,
      /PERCEPTIONLAB_DATABASE_URL=\$\$\{PERCEPTIONLAB_DATABASE_URL:-postgres:\/\/perceptionlab:perceptionlab@127\.0\.0\.1:55432\/perceptionlab\}/,
    );
    assert.match(makefile, /cargo run --manifest-path api\/Cargo\.toml -p perception_api/);
  });

  it('documents the product-grade P0 quickstart commands', () => {
    const readme = readFileSync('README.md', 'utf8');

    assert.match(readme, /npm run quality/);
    assert.match(readme, /make up/);
    assert.match(readme, /make loki-ready/);
    assert.match(readme, /docker compose up api/);
    assert.match(readme, /POST http:\/\/127\.0\.0\.1:8080\/training-jobs/);
    assert.match(readme, /POST http:\/\/127\.0\.0\.1:8080\/models\/<model_id>\/infer/);
  });

  it('keeps internal agent planning docs out of git', () => {
    const gitignore = readFileSync('.gitignore', 'utf8');

    assert.match(gitignore, /^doc\/superpowers\/$/m);
  });
});
