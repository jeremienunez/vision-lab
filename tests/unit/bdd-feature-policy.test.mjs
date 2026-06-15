import { describe, it } from 'node:test';
import assert from 'node:assert/strict';

import { validateFeatureDocument } from '../../scripts/bdd-feature-policy.mjs';

const validFeature = `@p0 @api
Feature: Platform healthcheck
  The platform must expose health endpoints.

  Scenario: API healthcheck returns healthy status
    Given the PerceptionLab API is running
    When I call GET "/health"
    Then the response status should be 200
`;

describe('BDD feature policy', () => {
  it('accepts a tagged feature with at least one scenario and Given When Then steps', () => {
    const result = validateFeatureDocument(validFeature);

    assert.equal(result.valid, true);
    assert.deepEqual(result.errors, []);
  });

  it('rejects a feature without priority tags', () => {
    const result = validateFeatureDocument(validFeature.replace('@p0 @api\n', ''));

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /tag/);
  });

  it('rejects a feature without a Then outcome', () => {
    const result = validateFeatureDocument(validFeature.replace('    Then the response status should be 200\n', ''));

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /Then/);
  });

  it('rejects a feature without a scenario', () => {
    const result = validateFeatureDocument(validFeature.replace('  Scenario: API healthcheck returns healthy status\n', ''));

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /Scenario/);
  });
});
