import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import {
  buildInferenceRequest,
  buildInferenceResultView,
  validateInferenceImageFile,
} from '../../web/src/dashboard/features/inference/inference-lab.js';

describe('single-image inference lab helpers', () => {
  it('validates supported image files for inference', () => {
    assert.equal(
      validateInferenceImageFile(new File(['jpg'], 'desk.jpg', { type: 'image/jpeg' })),
      '',
    );
    assert.equal(
      validateInferenceImageFile(new File(['png'], 'desk.png', { type: 'image/png' })),
      '',
    );
    assert.equal(
      validateInferenceImageFile(new File(['webp'], 'desk.webp', { type: 'image/webp' })),
      '',
    );
    assert.equal(
      validateInferenceImageFile(new File(['txt'], 'notes.txt', { type: 'text/plain' })),
      'Choose a JPG, PNG, or WebP image.',
    );
  });

  it('builds the model inference request from form state', () => {
    const imageFile = new File(['fake-image'], 'capture.jpg', { type: 'image/jpeg' });

    assert.deepEqual(
      buildInferenceRequest({
        modelId: 'model_01',
        imageFile,
        confidenceThreshold: 0.4,
      }),
      {
        modelId: 'model_01',
        imageBlob: imageFile,
        filename: 'capture.jpg',
        confidenceThreshold: 0.4,
      },
    );
  });

  it('formats inference results for display', () => {
    const view = buildInferenceResultView({
      run_id: 'run_01',
      latency_ms: 42,
      detections: [
        {
          class_name: 'cup',
          confidence: 0.914,
          bbox: { x: 0.1, y: 0.2, width: 0.3, height: 0.4 },
        },
      ],
    });

    assert.deepEqual(view, {
      runId: 'run_01',
      latencyLabel: '42 ms',
      detectionCountLabel: '1 detection',
      detections: [
        {
          id: 'cup-0',
          className: 'cup',
          confidenceLabel: '91%',
          bboxLabel: 'x 0.10 / y 0.20 / w 0.30 / h 0.40',
        },
      ],
    });
  });
});
