import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import {
  buildDatasetVersionOptions,
  buildTrainingJobCommand,
  localPathFromArtifactUri,
  preferredDatasetVersionId,
} from '../../web/src/dashboard/features/training/training-form.js';

describe('training job form helpers', () => {
  it('builds dataset version options with readable dataset context', () => {
    const options = buildDatasetVersionOptions(
      [
        {
          id: 'dataset_01',
          name: 'openimages-office-objects-v2',
          classes: ['phone', 'remote', 'person'],
        },
      ],
      {
        dataset_01: [
          {
            id: 'version_01',
            dataset_id: 'dataset_01',
            version_name: 'train-2026-06-18',
            sample_count: 1951,
            annotation_count: 4353,
          },
        ],
      },
    );

    assert.deepEqual(options, [
      {
        value: 'version_01',
        label: 'openimages-office-objects-v2 / train-2026-06-18',
        detail: '1951 samples / 4353 annotations / phone, remote, person',
      },
    ]);
  });

  it('builds the training job command from typed form state', () => {
    const command = buildTrainingJobCommand({
      datasetVersionId: 'version_01',
      modelFamily: ' yolo11s_finetune ',
      baseModel: ' file:///media/jerem/ubuntu1/perceptionlab/artifacts/models/best.pt ',
      epochs: '2',
      batchSize: '4',
      imageSize: '640',
      learningRate: '0.001',
    });

    assert.deepEqual(command, {
      datasetVersionId: 'version_01',
      modelFamily: 'yolo11s_finetune',
      baseModel: '/media/jerem/ubuntu1/perceptionlab/artifacts/models/best.pt',
      epochs: 2,
      batchSize: 4,
      imageSize: 640,
      learningRate: 0.001,
    });
  });

  it('returns an empty base model when the operator wants the worker default', () => {
    const command = buildTrainingJobCommand({
      datasetVersionId: 'version_01',
      modelFamily: 'yolo11n',
      baseModel: ' ',
      epochs: '1',
      batchSize: '2',
      imageSize: '640',
      learningRate: '0.001',
    });

    assert.equal(command.baseModel, '');
  });

  it('converts file artifact URIs to local paths for the worker', () => {
    assert.equal(
      localPathFromArtifactUri('file:///media/jerem/ubuntu1/model.pt'),
      '/media/jerem/ubuntu1/model.pt',
    );
  });

  it('prefers the dataset version attached to the strongest YOLO model', () => {
    const selected = preferredDatasetVersionId(
      [
        {
          id: 'old_yolo',
          name: 'final-yolo-model',
          model_family: 'yolo11n',
          dataset_version_id: 'old_version',
          artifact_uri: 'file:///repo/.perceptionlab/models/yolo11n.pt',
          metrics_summary: {},
        },
        {
          id: 'volume_yolo',
          name: 'worker-volume-yolo',
          model_family: 'yolo11s_finetune',
          dataset_version_id: 'volume_version',
          artifact_uri: 'file:///media/model.pt',
          metrics_summary: { mAP50: '0.6154332345571324' },
        },
      ],
      [
        { value: 'old_version' },
        { value: 'volume_version' },
      ],
    );

    assert.equal(selected, 'volume_version');
  });
});
