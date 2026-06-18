import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { orderCameraModels } from '../../web/src/dashboard/camera-models.js';

describe('camera model selection', () => {
  it('prioritizes real YOLO models for live camera inference', () => {
    const models = orderCameraModels([
      {
        id: 'tiny_01',
        name: 'worker-tiny',
        model_family: 'tiny_torch',
        artifact_uri: 'file:///tmp/model.pt',
        status: 'candidate',
      },
      {
        id: 'yolo_01',
        name: 'final-yolo',
        model_family: 'yolo11n',
        artifact_uri: 'file:///repo/.perceptionlab/models/yolo11n.pt',
        status: 'candidate',
      },
    ]);

    assert.equal(models[0].id, 'yolo_01');
  });

  it('keeps archived models out of the camera selector', () => {
    const models = orderCameraModels([
      {
        id: 'archived_yolo',
        name: 'old-yolo',
        model_family: 'yolo11n',
        artifact_uri: 'file:///repo/.perceptionlab/models/yolo11n.pt',
        status: 'archived',
      },
      {
        id: 'tiny_01',
        name: 'worker-tiny',
        model_family: 'tiny_torch',
        artifact_uri: 'file:///tmp/model.pt',
        status: 'candidate',
      },
    ]);

    assert.deepEqual(models.map((model) => model.id), ['tiny_01']);
  });
});
