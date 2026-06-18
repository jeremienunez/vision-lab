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
});
