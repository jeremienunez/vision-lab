import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
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
      apiKey: 'local-secret',
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
    assert.equal(
      calls.every(([, options]) => options.headers['x-api-key'] === 'local-secret'),
      true,
    );
    assert.deepEqual(JSON.parse(output), {
      dataset_id: 'ds_01',
      dataset_name: 'desk-objects-v1',
      version_id: 'dsv_01',
      version_name: 'v1',
      sample_count: 1,
      annotation_count: 3,
    });
  });

  it('posts a downloaded HF-style manifest through the same API flow', async () => {
    const seedRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'perceptionlab-hf-seed-'));
    fs.mkdirSync(path.join(seedRoot, 'images'));
    fs.writeFileSync(path.join(seedRoot, 'images', 'sample.jpg'), 'fake-jpeg-bytes');
    fs.writeFileSync(
      path.join(seedRoot, 'manifest.json'),
      JSON.stringify({
        dataset: {
          name: 'cppe5-smoke',
          description: 'Ingested from Hugging Face dataset cppe-5 split train.',
          task_type: 'object_detection',
          classes: ['Coverall', 'Mask'],
        },
        version: {
          version_name: 'v1',
          created_by: 'hf-ingest',
        },
        samples: [
          {
            filename: 'sample.jpg',
            path: 'images/sample.jpg',
            mime_type: 'image/jpeg',
            width: 320,
            height: 240,
            annotations: [
              {
                class_name: 'Mask',
                class_id: 1,
                bbox: { x: 0.1, y: 0.2, width: 0.3, height: 0.4 },
                confidence: null,
              },
            ],
          },
        ],
      }),
    );
    const calls = [];
    let output = '';

    const code = await seedDemoDataset({
      baseUrl: 'http://api.local',
      seedRoot,
      fetchImpl: async (url, options) => {
        calls.push([url, options]);

        if (url.endsWith('/datasets')) {
          assert.equal(JSON.parse(options.body).name, 'cppe5-smoke');
          return jsonResponse({ id: 'ds_hf' }, 201);
        }
        if (url.endsWith('/datasets/ds_hf/samples')) {
          return jsonResponse({ id: 'smp_hf' }, 201);
        }
        if (url.endsWith('/samples/smp_hf/annotations')) {
          assert.equal(JSON.parse(options.body).class_name, 'Mask');
          return jsonResponse({ id: 'ann_hf' }, 201);
        }
        if (url.endsWith('/datasets/ds_hf/versions')) {
          assert.equal(JSON.parse(options.body).created_by, 'hf-ingest');
          return jsonResponse({ id: 'dsv_hf', version_name: 'v1' }, 201);
        }

        throw new Error(`Unexpected URL: ${url}`);
      },
      stdout: (value) => {
        output += value;
      },
    });

    assert.equal(code, 0);
    assert.equal(calls.length, 4);
    assert.deepEqual(JSON.parse(output), {
      dataset_id: 'ds_hf',
      dataset_name: 'cppe5-smoke',
      version_id: 'dsv_hf',
      version_name: 'v1',
      sample_count: 1,
      annotation_count: 1,
    });
  });
});
