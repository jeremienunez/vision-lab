import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { createPerceptionApi } from '../../web/src/dashboard/perception-api.js';

describe('PerceptionLab dashboard API client', () => {
  it('posts webcam frames to model inference as multipart form data', async () => {
    let request = null;
    const api = createPerceptionApi({
      baseUrl: 'http://api.local',
      apiKey: 'local-secret',
      fetchImpl: async (url, options) => {
        request = { url, options };

        return new Response(JSON.stringify({
          run_id: 'irun_01',
          model_id: 'mdl_01',
          latency_ms: 42,
          detections: [{ class_name: 'cup', confidence: 0.91 }],
        }), {
          status: 200,
          headers: { 'content-type': 'application/json' },
        });
      },
    });

    const result = await api.runModelInference({
      modelId: 'mdl_01',
      imageBlob: new Blob(['fake-jpeg'], { type: 'image/jpeg' }),
      filename: 'webcam-frame.jpg',
      confidenceThreshold: 0.4,
    });

    assert.equal(request.url, 'http://api.local/models/mdl_01/infer');
    assert.equal(request.options.method, 'POST');
    assert.equal(request.options.headers['x-api-key'], 'local-secret');
    assert.equal(request.options.body.get('confidence_threshold'), '0.4');
    assert.equal(request.options.body.get('image').name, 'webcam-frame.jpg');
    assert.equal(result.detections[0].class_name, 'cup');
  });

  it('posts training job creation as JSON', async () => {
    let request = null;
    const api = createPerceptionApi({
      baseUrl: 'http://api.local',
      apiKey: 'local-secret',
      fetchImpl: async (url, options) => {
        request = { url, options };

        return new Response(JSON.stringify({
          id: 'job_01',
          dataset_version_id: 'version_01',
          model_family: 'yolo11s_finetune',
          base_model: '/media/models/best.pt',
          status: 'queued',
        }), {
          status: 201,
          headers: { 'content-type': 'application/json' },
        });
      },
    });

    const result = await api.createTrainingJob({
      datasetVersionId: 'version_01',
      modelFamily: 'yolo11s_finetune',
      baseModel: '/media/models/best.pt',
      epochs: 2,
      batchSize: 4,
      imageSize: 640,
      learningRate: 0.001,
    });

    assert.equal(request.url, 'http://api.local/training-jobs');
    assert.equal(request.options.method, 'POST');
    assert.equal(request.options.headers['content-type'], 'application/json');
    assert.equal(request.options.headers['x-api-key'], 'local-secret');
    assert.deepEqual(JSON.parse(request.options.body), {
      dataset_version_id: 'version_01',
      model_family: 'yolo11s_finetune',
      base_model: '/media/models/best.pt',
      hyperparameters: {
        epochs: 2,
        batch_size: 4,
        image_size: 640,
        learning_rate: 0.001,
      },
    });
    assert.equal(result.status, 'queued');
  });
});
