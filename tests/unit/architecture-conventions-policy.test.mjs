import { describe, it } from 'node:test';
import assert from 'node:assert/strict';

import {
  requiredArchitecturePaths,
  validateArchitectureConventions,
} from '../../scripts/architecture-conventions-policy.mjs';

describe('architecture conventions policy', () => {
  it('accepts the prescribed Rust, Python worker, QA, and contracts structure', () => {
    const result = validateArchitectureConventions(requiredArchitecturePaths);

    assert.equal(result.valid, true);
    assert.deepEqual(result.errors, []);
  });

  it('rejects vague helper-style files and folders', () => {
    const result = validateArchitectureConventions([
      ...requiredArchitecturePaths,
      'api/crates/perception_domain/src/utils.rs',
      'worker/perception_worker/common',
    ]);

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /utils\.rs/);
    assert.match(result.errors.join('\n'), /common/);
  });

  it('rejects legacy folders that contradict the design-pattern document', () => {
    const result = validateArchitectureConventions([
      ...requiredArchitecturePaths,
      'apps/api-rust/README.md',
      'workers/pytorch-trainer/README.md',
      'src/domain/README.md',
      'doc/superpowers/plans/internal-plan.md',
    ]);

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /apps\/api-rust/);
    assert.match(result.errors.join('\n'), /workers\/pytorch-trainer/);
    assert.match(result.errors.join('\n'), /src\/domain/);
    assert.match(result.errors.join('\n'), /doc\/superpowers/);
  });

  it('rejects missing required architecture folders', () => {
    const result = validateArchitectureConventions(
      requiredArchitecturePaths.filter((path) => path !== 'api/crates/perception_domain/src'),
    );

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /perception_domain/);
  });

  it('ignores generated dependency and local cache folders', () => {
    const result = validateArchitectureConventions([
      ...requiredArchitecturePaths,
      '.perceptionlab/cache/uv/archive-v0/example/ultralytics/utils.py',
      'worker/.venv/lib/python3.12/site-packages/torch/utils/data.py',
      'api/target/debug/build/example/utils.rs',
    ]);

    assert.equal(result.valid, true);
    assert.deepEqual(result.errors, []);
  });
});
