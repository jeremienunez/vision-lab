import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { runInferenceBenchmark } from '../../scripts/benchmark-inference.mjs';

function response(body, status = 200) {
  return {
    status,
    async text() {
      return JSON.stringify(body);
    },
  };
}

describe('inference benchmark script', () => {
  it('runs configured inference iterations and reports latency summary', async () => {
    const calls = [];
    const ticks = [100, 118, 200, 221, 300, 330];
    let output = '';

    const code = await runInferenceBenchmark(
      [
        '--base-url',
        'http://api.local/',
        '--model-id',
        'mdl_01',
        '--image',
        'datasets/seed/images/desk-objects.png',
        '--iterations',
        '3',
      ],
      {
        apiKey: 'local-secret',
        fetchImpl: async (url, options) => {
          calls.push([url, options]);
          return response({ run_id: `run_${calls.length}`, latency_ms: 9, detections: [] });
        },
        now: () => ticks.shift(),
        stdout: (value) => {
          output += value;
        },
      },
    );

    assert.equal(code, 0);
    assert.equal(calls.length, 3);
    assert.equal(calls[0][0], 'http://api.local/models/mdl_01/infer');
    assert.equal(calls[0][1].method, 'POST');
    assert.equal(calls[0][1].headers['x-api-key'], 'local-secret');

    const summary = JSON.parse(output);
    assert.equal(summary.model_id, 'mdl_01');
    assert.equal(summary.iterations, 3);
    assert.equal(summary.client_latency_ms.min, 18);
    assert.equal(summary.client_latency_ms.max, 30);
    assert.equal(summary.client_latency_ms.avg, 23);
    assert.equal(summary.api_latency_ms.avg, 9);
  });

  it('returns a usage error when model id is missing', async () => {
    let errorOutput = '';

    const code = await runInferenceBenchmark(['--image', 'datasets/seed/images/desk-objects.png'], {
      fetchImpl: async () => response({}),
      stderr: (value) => {
        errorOutput += value;
      },
    });

    assert.equal(code, 2);
    assert.match(errorOutput, /--model-id/);
  });
});
