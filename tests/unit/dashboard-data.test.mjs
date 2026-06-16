import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import {
  buildApiHeaders,
  buildDashboardViewModel,
} from '../../web/src/dashboard/dashboard-data.js';

describe('dashboard data view model', () => {
  it('summarizes platform state from API payloads', () => {
    const viewModel = buildDashboardViewModel({
      health: {
        status: 'healthy',
        dependencies: {
          database: 'ready',
          storage: 'ready',
          queue: 'ready',
        },
      },
      datasets: [
        { id: 'ds_01', name: 'desk-objects', classes: ['cup', 'book'], status: 'draft' },
        { id: 'ds_02', name: 'ppe-smoke', classes: ['Mask'], status: 'draft' },
      ],
      trainingJobs: [
        { id: 'job_01', model_family: 'yolo', status: 'running' },
        { id: 'job_02', model_family: 'tiny_torch', status: 'succeeded' },
      ],
      models: [
        {
          id: 'mdl_01',
          name: 'desk-objects-demo',
          version: 'v1',
          status: 'promoted',
          metrics_summary: { mAP50: '0.91', classes: 'cup,book' },
        },
      ],
      metricsByJob: {
        job_01: [
          { metric_name: 'loss', metric_value: 0.32, epoch: 2 },
          { metric_name: 'mAP50', metric_value: 0.88, epoch: 2 },
        ],
      },
    });

    assert.deepEqual(
      viewModel.kpis.map((kpi) => [kpi.label, kpi.value]),
      [
        ['Datasets', '2'],
        ['Jobs active', '1'],
        ['Models', '1'],
        ['Latest metric', 'mAP50 0.88'],
      ],
    );
    assert.deepEqual(viewModel.jobStatusCounts, { running: 1, succeeded: 1 });
    assert.equal(viewModel.healthLabel, 'API healthy');
    assert.equal(viewModel.promotedModelCount, 1);
  });

  it('adds the API key header only when a local key is configured', () => {
    assert.deepEqual(buildApiHeaders('  local-secret  '), { 'x-api-key': 'local-secret' });
    assert.deepEqual(buildApiHeaders('   '), {});
    assert.deepEqual(buildApiHeaders(undefined), {});
  });
});
