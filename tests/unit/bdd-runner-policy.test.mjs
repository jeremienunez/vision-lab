import assert from 'node:assert/strict';
import fs from 'node:fs';
import { describe, it } from 'node:test';

import { validateBddRunnerDecision } from '../../scripts/bdd-runner-policy.mjs';

describe('BDD runner policy', () => {
  it('accepts the final Cucumber-JS runner decision', () => {
    assert.deepEqual(validateBddRunnerDecision(), []);
  });

  it('exposes an executable BDD smoke subset', () => {
    const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
    const runnerDoc = fs.readFileSync('qa/bdd-runner.md', 'utf8');

    assert.match(packageJson.scripts['bdd:smoke'], /cucumber-js/);
    assert.match(packageJson.scripts['bdd:smoke'], /@smoke/);
    assert.match(runnerDoc, /bdd:smoke/);
    assert.match(runnerDoc, /@smoke/);
  });
});
