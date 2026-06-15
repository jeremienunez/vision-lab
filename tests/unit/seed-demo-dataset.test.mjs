import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { seedDemoDataset } from '../../scripts/seed-demo-dataset.mjs';

function jsonResponse(body, status = 200) {
  return {
    status,
    async text() {
      return JSON.stringify(body);
    },
  };
}

describe('seed demo dataset script', () => {
  it('posts dataset, sample, annotations, and dataset version in order', async () => {
    const calls = [];
    let output = '';

    const code = await seedDemoDataset({
      baseUrl: 'http://api.local/',
      fetchImpl: async (url, options) => {
        calls.push([url, options]);

        if (url.endsWith('/datasets')) {
          return jsonResponse({ id: 'ds_01' }, 201);
        }
        if (url.endsWith('/datasets/ds_01/samples')) {
          return jsonResponse({ id: 'smp_01' }, 201);
        }
        if (url.endsWith('/samples/smp_01/annotations')) {
          return jsonResponse({ id: `ann_${calls.length}` }, 201);
        }
        if (url.endsWith('/datasets/ds_01/versions')) {
          return jsonResponse({ id: 'dsv_01', version_name: 'v1' }, 201);
        }

        throw new Error(`Unexpected URL: ${url}`);
      },
      stdout: (value) => {
        output += value;
      },
    });

    assert.equal(code, 0);
    assert.equal(calls[0][0], 'http://api.local/datasets');
    assert.equal(calls[1][0], 'http://api.local/datasets/ds_01/samples');
    assert.equal(
      calls.filter(([url]) => url === 'http://api.local/samples/smp_01/annotations').length,
      3,
    );
    assert.equal(calls.at(-1)[0], 'http://api.local/datasets/ds_01/versions');
    assert.deepEqual(JSON.parse(output), {
      dataset_id: 'ds_01',
      dataset_name: 'desk-objects-v1',
      version_id: 'dsv_01',
      version_name: 'v1',
      sample_count: 1,
      annotation_count: 3,
    });
  });
});
