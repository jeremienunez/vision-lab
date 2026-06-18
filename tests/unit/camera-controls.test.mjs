import assert from 'node:assert/strict';
import { test } from 'node:test';

import { normalizeConfidenceThreshold } from '../../web/src/dashboard/camera-controls.js';

test('clamps values into the [0, 1] range', () => {
  assert.equal(normalizeConfidenceThreshold(1.5), 1);
  assert.equal(normalizeConfidenceThreshold(-0.2), 0);
  assert.equal(normalizeConfidenceThreshold(0.4), 0.4);
});

test('falls back to the default for non-finite input', () => {
  assert.equal(normalizeConfidenceThreshold('abc'), 0.25);
  assert.equal(normalizeConfidenceThreshold(Number.NaN), 0.25);
});
