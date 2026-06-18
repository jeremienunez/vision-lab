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
  };
}

function normalizeBaseUrl(baseUrl) {
  const fallbackBaseUrl = '/api';
  const normalized = String(baseUrl || fallbackBaseUrl).trim();

  return normalized.replace(/\/+$/, '');
}
