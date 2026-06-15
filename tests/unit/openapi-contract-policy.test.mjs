import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { describe, it } from 'node:test';

import {
  loadOpenApiContract,
  validateOpenApiContract,
} from '../../scripts/openapi-contract-policy.mjs';

describe('OpenAPI contract policy', () => {
  it('accepts the published PerceptionLab API contract', () => {
    const errors = validateOpenApiContract(loadOpenApiContract());

    assert.deepEqual(errors, []);
  });

  it('rejects an empty placeholder contract', () => {
    const errors = validateOpenApiContract({
      openapi: '3.1.0',
      info: { title: 'PerceptionLab API', version: '0.1.0' },
      paths: {},
    });

    assert(errors.some((error) => error.includes('Missing GET /health')));
    assert(errors.some((error) => error.includes('Missing schema InferenceResponse')));
  });

  it('exports the same JSON contract used by validation', () => {
    const exported = execFileSync('sh', ['scripts/export_openapi.sh'], {
      encoding: 'utf8',
    });

    assert.deepEqual(JSON.parse(exported), loadOpenApiContract());
  });
});
