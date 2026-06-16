import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { describe, it } from 'node:test';

import { fireDemoProduct, parseFireDemoOptions } from '../../scripts/fire-demo-product.mjs';

function response(body, status = 200) {
  return {
    status,
    async text() {
      return JSON.stringify(body);
    },
  };
}

describe('fire demo product script', () => {
  it('runs the full product smoke flow through inference and overlay', async () => {
    const calls = [];
    let output = '';

    const code = await fireDemoProduct({
      baseUrl: 'http://api.local/',
      fetchImpl: async (url, options) => {
        calls.push([url, options]);

        if (url === 'http://api.local/health') {
          return response({ status: 'ok' });
        }
        if (url === 'http://api.local/datasets') {
          return response({ id: 'ds_01' }, 201);
        }
        if (url === 'http://api.local/datasets/ds_01/samples') {
          return response({ id: 'smp_01' }, 201);
        }
        if (url === 'http://api.local/samples/smp_01/annotations') {
          return response({ id: `ann_${calls.length}` }, 201);
        }
        if (url === 'http://api.local/datasets/ds_01/versions') {
          return response({ id: 'dsv_01', version_name: 'v1' }, 201);
        }
        if (url === 'http://api.local/training-jobs') {
          return response({ id: 'job_01', dataset_version_id: 'dsv_01', status: 'queued' }, 201);
        }
        if (url === 'http://api.local/training-jobs/job_01/status') {
          const payload = JSON.parse(options.body);
          return response({ id: 'job_01', status: payload.next_status });
        }
        if (url === 'http://api.local/models') {
          return response(
            {
              id: 'mdl_01',
              training_job_id: 'job_01',
              status: 'candidate',
              metrics_summary: { classes: 'cup,book,phone' },
            },
            201,
          );
        }
        if (url === 'http://api.local/models/mdl_01/infer') {
          assert.equal(options.body.get('confidence_threshold'), '0.25');
          return response({
            run_id: 'irun_01',
            model_id: 'mdl_01',
            latency_ms: 1,
            detections: [
              { class_name: 'cup', confidence: 0.91 },
              { class_name: 'book', confidence: 0.88 },
            ],
          });
        }
        if (url === 'http://api.local/inference-runs/irun_01/overlay') {
          return response(
            {
              inference_run_id: 'irun_01',
              artifact_uri: 'file:///tmp/perceptionlab/overlays/irun_01.svg',
              labels: ['cup 91%', 'book 88%'],
            },
            201,
          );
        }

        throw new Error(`Unexpected request: ${url}`);
      },
      stdout: (value) => {
        output += value;
      },
    });

    assert.equal(code, 0);
    assert.deepEqual(
      calls.map(([url]) => url),
      [
        'http://api.local/health',
        'http://api.local/datasets',
        'http://api.local/datasets/ds_01/samples',
        'http://api.local/samples/smp_01/annotations',
        'http://api.local/samples/smp_01/annotations',
        'http://api.local/samples/smp_01/annotations',
        'http://api.local/datasets/ds_01/versions',
        'http://api.local/training-jobs',
        'http://api.local/training-jobs/job_01/status',
        'http://api.local/training-jobs/job_01/status',
        'http://api.local/models',
        'http://api.local/models/mdl_01/infer',
        'http://api.local/inference-runs/irun_01/overlay',
      ],
    );
    assert.deepEqual(JSON.parse(output), {
      dataset_id: 'ds_01',
      dataset_version_id: 'dsv_01',
      training_job_id: 'job_01',
      model_id: 'mdl_01',
      inference_run_id: 'irun_01',
      detected_classes: ['cup', 'book'],
      overlay_artifact_uri: 'file:///tmp/perceptionlab/overlays/irun_01.svg',
      status: 'object_recognition_smoke_passed',
    });
  });

  it('fails when inference returns no detections', async () => {
    await assert.rejects(
      fireDemoProduct({
        baseUrl: 'http://api.local/',
        modelArtifactUri: 'file:///models/yolo11n.pt',
        fetchImpl: async (url) => {
          if (url === 'http://api.local/health') return response({ status: 'ok' });
          if (url === 'http://api.local/datasets') return response({ id: 'ds_01' }, 201);
          if (url === 'http://api.local/datasets/ds_01/samples') {
            return response({ id: 'smp_01' }, 201);
          }
          if (url === 'http://api.local/samples/smp_01/annotations') {
            return response({ id: 'ann_01' }, 201);
          }
          if (url === 'http://api.local/datasets/ds_01/versions') {
            return response({ id: 'dsv_01', version_name: 'v1' }, 201);
          }
          if (url === 'http://api.local/training-jobs') {
            return response({ id: 'job_01', dataset_version_id: 'dsv_01', status: 'queued' }, 201);
          }
          if (url === 'http://api.local/training-jobs/job_01/status') {
            return response({ id: 'job_01', status: 'succeeded' });
          }
          if (url === 'http://api.local/models') return response({ id: 'mdl_01' }, 201);
          if (url === 'http://api.local/models/mdl_01/infer') {
            return response({ run_id: 'irun_01', detections: [] });
          }
          throw new Error(`Unexpected request: ${url}`);
        },
        stdout: () => {},
      }),
      /Object recognition smoke failed: no detections returned/,
    );
  });

  it('runs inference with a custom local image path', async () => {
    const tmpDir = await fs.mkdtemp(path.join(os.tmpdir(), 'perceptionlab-fire-'));
    const imagePath = path.join(tmpDir, 'phone-capture.png');
    await fs.writeFile(imagePath, Buffer.from([137, 80, 78, 71]));
    let inferredFilename = '';

    await fireDemoProduct({
      baseUrl: 'http://api.local/',
      imagePath,
      fetchImpl: async (url, options) => {
        if (url === 'http://api.local/health') return response({ status: 'ok' });
        if (url === 'http://api.local/datasets') return response({ id: 'ds_01' }, 201);
        if (url === 'http://api.local/datasets/ds_01/samples') {
          return response({ id: 'smp_01' }, 201);
        }
        if (url === 'http://api.local/samples/smp_01/annotations') {
          return response({ id: 'ann_01' }, 201);
        }
        if (url === 'http://api.local/datasets/ds_01/versions') {
          return response({ id: 'dsv_01', version_name: 'v1' }, 201);
        }
        if (url === 'http://api.local/training-jobs') {
          return response({ id: 'job_01', dataset_version_id: 'dsv_01', status: 'queued' }, 201);
        }
        if (url === 'http://api.local/training-jobs/job_01/status') {
          return response({ id: 'job_01', status: 'succeeded' });
        }
        if (url === 'http://api.local/models') return response({ id: 'mdl_01' }, 201);
        if (url === 'http://api.local/models/mdl_01/infer') {
          inferredFilename = options.body.get('image').name;
          return response({
            run_id: 'irun_01',
            detections: [{ class_name: 'cup', confidence: 0.91 }],
          });
        }
        if (url === 'http://api.local/inference-runs/irun_01/overlay') {
          return response({ artifact_uri: 'file:///tmp/overlay.svg' }, 201);
        }
        throw new Error(`Unexpected request: ${url}`);
      },
      stdout: () => {},
    });

    assert.equal(inferredFilename, 'phone-capture.png');
  });

  it('registers a model with the configured artifact URI', async () => {
    let registeredModelPayload = null;

    await fireDemoProduct({
      baseUrl: 'http://api.local/',
      modelArtifactUri: 'file:///models/yolo11n.pt',
      fetchImpl: async (url, options = {}) => {
        if (url === 'http://api.local/health') return response({ status: 'ok' });
        if (url === 'http://api.local/datasets') return response({ id: 'ds_01' }, 201);
        if (url === 'http://api.local/datasets/ds_01/samples') {
          return response({ id: 'smp_01' }, 201);
        }
        if (url === 'http://api.local/samples/smp_01/annotations') {
          return response({ id: 'ann_01' }, 201);
        }
        if (url === 'http://api.local/datasets/ds_01/versions') {
          return response({ id: 'dsv_01', version_name: 'v1' }, 201);
        }
        if (url === 'http://api.local/training-jobs') {
          return response({ id: 'job_01', dataset_version_id: 'dsv_01', status: 'queued' }, 201);
        }
        if (url === 'http://api.local/training-jobs/job_01/status') {
          return response({ id: 'job_01', status: 'succeeded' });
        }
        if (url === 'http://api.local/models') {
          registeredModelPayload = JSON.parse(options.body);
          return response({ id: 'mdl_01' }, 201);
        }
        if (url === 'http://api.local/models/mdl_01/infer') {
          return response({ run_id: 'irun_01', detections: [{ class_name: 'person' }] });
        }
        if (url === 'http://api.local/inference-runs/irun_01/overlay') {
          return response({ artifact_uri: 'file:///tmp/overlay.svg' }, 201);
        }
        throw new Error(`Unexpected request: ${url}`);
      },
      stdout: () => {},
    });

    assert.equal(registeredModelPayload.artifact_uri, 'file:///models/yolo11n.pt');
  });

  it('rejects a missing custom image before calling the API', async () => {
    let calls = 0;

    await assert.rejects(
      fireDemoProduct({
        baseUrl: 'http://api.local/',
        imagePath: '/tmp/perceptionlab-missing-image.png',
        fetchImpl: async () => {
          calls += 1;
          return response({ status: 'ok' });
        },
        stdout: () => {},
      }),
      /Input image not found/,
    );
    assert.equal(calls, 0);
  });

  it('parses the custom image CLI option', () => {
    assert.deepEqual(parseFireDemoOptions(['--image', '/tmp/capture.jpg']), {
      imagePath: '/tmp/capture.jpg',
    });
  });

  it('parses the confidence threshold CLI option', () => {
    assert.deepEqual(parseFireDemoOptions(['--confidence-threshold', '0.40']), {
      confidenceThreshold: '0.40',
    });
  });
});
