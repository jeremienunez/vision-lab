import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { validateBddRunnerDecision } from '../../scripts/bdd-runner-policy.mjs';

describe('BDD runner policy', () => {
  it('accepts the final Cucumber-JS runner decision', () => {
    assert.deepEqual(validateBddRunnerDecision(), []);
  });
});
