import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { validateSeedDataset } from '../../scripts/seed-dataset-policy.mjs';

describe('seed dataset policy', () => {
  it('accepts the published desk object seed dataset', () => {
    assert.deepEqual(validateSeedDataset(), []);
  });

  it('rejects a missing manifest', () => {
    const errors = validateSeedDataset('missing-seed-root');

    assert(errors.some((error) => error.includes('Missing seed manifest')));
  });
});
