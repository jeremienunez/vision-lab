export function response(body, status = 200) {
  return {
    status,
    async text() {
      return JSON.stringify(body);
    },
  };
}

export function apiResponseFor({ path, configuredApiKey, providedApiKey }) {
  if (path === '/health') {
    return response({
      status: 'healthy',
      dependencies: {
        database: 'ready',
        storage: 'ready',
        queue: 'ready',
      },
    });
  }

  if (configuredApiKey) {
    if (!providedApiKey) {
      return response({ error: { code: 'missing_api_key', message: 'Missing x-api-key header' } }, 401);
    }

    if (providedApiKey !== configuredApiKey) {
      return response({ error: { code: 'invalid_api_key', message: 'Invalid x-api-key header' } }, 403);
    }
  }

  if (path === '/datasets') return response({ datasets: [] });

  return response({ error: { code: 'not_found', message: 'Not found' } }, 404);
}

export function fireDemoFetch() {
  const calls = [];

  return {
    calls,
    async fetch(url, options = {}) {
      calls.push([url, options]);

      if (url === 'http://api.local/health') return response({ status: 'ok' });
      if (url === 'http://api.local/datasets') return response({ id: 'ds_01' }, 201);
      if (url === 'http://api.local/datasets/ds_01/samples') return response({ id: 'smp_01' }, 201);
      if (url === 'http://api.local/samples/smp_01/annotations') return response({ id: `ann_${calls.length}` }, 201);
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
        return response({
          id: 'mdl_01',
          training_job_id: 'job_01',
          status: 'candidate',
          metrics_summary: { classes: 'cup,book,phone' },
        }, 201);
      }
      if (url === 'http://api.local/models/mdl_01/infer') {
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
        return response({
          inference_run_id: 'irun_01',
          artifact_uri: 'file:///tmp/perceptionlab/overlays/irun_01.svg',
          labels: ['cup 91%', 'book 88%'],
        }, 201);
      }

      throw new Error(`Unexpected BDD smoke request: ${url}`);
    },
  };
}
