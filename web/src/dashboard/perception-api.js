import { buildApiHeaders } from './dashboard-data.js';

export function createPerceptionApi({ baseUrl, apiKey, fetchImpl = globalThis.fetch }) {
  const apiBaseUrl = normalizeBaseUrl(baseUrl);

  async function getJson(path) {
    const response = await fetchImpl(`${apiBaseUrl}${path}`, {
      headers: buildApiHeaders(apiKey),
    });
    return readJsonResponse(response);
  }

  async function postMultipart(path, formData) {
    const response = await fetchImpl(`${apiBaseUrl}${path}`, {
      method: 'POST',
      headers: buildApiHeaders(apiKey),
      body: formData,
    });

    return readJsonResponse(response);
  }

  async function postJson(path, payload) {
    const response = await fetchImpl(`${apiBaseUrl}${path}`, {
      method: 'POST',
      headers: {
        ...buildApiHeaders(apiKey),
        'content-type': 'application/json',
      },
      body: JSON.stringify(payload),
    });

    return readJsonResponse(response);
  }

  async function readJsonResponse(response) {
    const text = await response.text();

    if (!response.ok) {
      throw new Error(`${response.status} ${text || response.statusText}`);
    }

    return text ? JSON.parse(text) : {};
  }

  return {
    async loadDashboard() {
      const [health, datasets, trainingJobs, models] = await Promise.all([
        getJson('/health'),
        getJson('/datasets').then((payload) => payload.datasets ?? []),
        getJson('/training-jobs').then((payload) => payload.training_jobs ?? []),
        getJson('/models').then((payload) => payload.models ?? []),
      ]);
      const metricResults = await Promise.allSettled(
        trainingJobs.slice(0, 8).map(async (job) => {
          const payload = await getJson(`/training-jobs/${job.id}/metrics`);
          return [job.id, payload.metrics ?? []];
        }),
      );
      const metricsByJob = Object.fromEntries(
        metricResults
          .filter((result) => result.status === 'fulfilled')
          .map((result) => result.value),
      );

      return {
        health,
        datasets,
        trainingJobs,
        models,
        metricsByJob,
      };
    },

    async runModelInference({
      modelId,
      imageBlob,
      filename = 'webcam-frame.jpg',
      confidenceThreshold = 0.25,
    }) {
      const formData = new FormData();
      formData.set('confidence_threshold', String(confidenceThreshold));
      formData.set('image', imageBlob, filename);

      return postMultipart(`/models/${modelId}/infer`, formData);
    },

    async listDatasetVersions(datasetId) {
      const payload = await getJson(`/datasets/${datasetId}/versions`);
      return payload.dataset_versions ?? [];
    },

    async createTrainingJob({
      datasetVersionId,
      modelFamily,
      baseModel,
      epochs,
      batchSize,
      imageSize,
      learningRate,
    }) {
      return postJson('/training-jobs', {
        dataset_version_id: datasetVersionId,
        model_family: modelFamily,
        base_model: baseModel || null,
        hyperparameters: {
          epochs,
          batch_size: batchSize,
          image_size: imageSize,
          learning_rate: learningRate,
        },
      });
    },
  };
}

function normalizeBaseUrl(baseUrl) {
  const fallbackBaseUrl = '/api';
  const normalized = String(baseUrl || fallbackBaseUrl).trim();

  return normalized.replace(/\/+$/, '');
}
