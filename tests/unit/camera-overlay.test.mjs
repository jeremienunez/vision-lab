import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { buildDetectionOverlayItems } from '../../web/src/dashboard/camera-overlay.js';

describe('camera detection overlay', () => {
  it('projects model detections into labels positioned on the camera preview', () => {
    const items = buildDetectionOverlayItems([
      {
        class_name: 'cup',
        confidence: 0.914,
        bbox: { x: 0.1, y: 0.2, width: 0.3, height: 0.4 },
      },
    ]);

    assert.deepEqual(items, [
      {
        label: 'cup',
        confidenceLabel: '91%',
        style: {
          left: '10%',
          top: '20%',
          width: '30%',
          height: '40%',
        },
      },
    ]);
  });

  it('mirrors horizontal positions for the user-facing camera preview', () => {
    const items = buildDetectionOverlayItems([
      {
        class_name: 'bottle',
        confidence: 0.71,
        bbox: { x: 0.1, y: 0.2, width: 0.3, height: 0.4 },
      },
    ], { mirrored: true });

    assert.equal(items[0].label, 'bottle');
    assert.deepEqual(items[0].style, {
      left: '60%',
      top: '20%',
      width: '30%',
      height: '40%',
    });
  });

  it('keeps a visible fallback label when a detection has no usable bbox', () => {
    const items = buildDetectionOverlayItems([
      { class_name: 'keyboard', confidence: 0.8, bbox: { x: 5, y: -1, width: 0, height: 2 } },
    ]);

    assert.equal(items[0].label, 'keyboard');
    assert.equal(items[0].confidenceLabel, '80%');
    assert.deepEqual(items[0].style, {
      left: '6%',
      top: '8%',
      width: '88%',
      height: '84%',
    });
  });
});
