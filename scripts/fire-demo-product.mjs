#!/usr/bin/env node
import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { loadSeedManifest } from './seed-dataset-policy.mjs';
import { seedDemoDataset } from './seed-demo-dataset.mjs';

const defaultBaseUrl = process.env.PERCEPTIONLAB_API_BASE_URL ?? 'http://127.0.0.1:8080';
const defaultSeedRoot = process.env.PERCEPTIONLAB_SEED_DATASET_ROOT ?? 'datasets/seed';

export async function fireDemoProduct(dependencies = {}) {
  const {
    baseUrl = defaultBaseUrl,
    seedRoot = defaultSeedRoot,
    fetchImpl = globalThis.fetch,
    stdout = (value) => process.stdout.write(value),
  } = dependencies;

  const apiBaseUrl = baseUrl.replace(/\/+$/, '');
  const manifest = loadSeedManifest(seedRoot);

  await getJson(fetchImpl, `${apiBaseUrl}/health`);

  let seedOutput = '';
  await seedDemoDataset({
    baseUrl: apiBaseUrl,
    seedRoot,
    fetchImpl,
    stdout: (value) => {
      seedOutput += value;
    },
  });
  const seed = JSON.parse(seedOutput);
  const trainingJob = await postJson(fetchImpl, `${apiBaseUrl}/training-jobs`, {
    dataset_version_id: seed.version_id,
    model_family: 'tiny_torch',
    base_model: null,
    hyperparameters: {
      epochs: 1,
      batch_size: 1,
      image_size: 64,
      learning_rate: 0.01,
    },
  });

  await patchJson(fetchImpl, `${apiBaseUrl}/training-jobs/${trainingJob.id}/status`, {
    next_status: 'running',
    error_message: null,
  });
  await patchJson(fetchImpl, `${apiBaseUrl}/training-jobs/${trainingJob.id}/status`, {
    next_status: 'succeeded',
    error_message: null,
  });

  const model = await postJson(fetchImpl, `${apiBaseUrl}/models`, {
    training_job_id: trainingJob.id,
    name: 'desk-objects-demo',
    version: 'v1',
    artifact_uri: 'file:///tmp/perceptionlab/demo-model.pt',
    metrics_summary: {
      mAP50: '0.91',
      classes: manifest.dataset.classes.join(','),
    },
  });
  const sample = manifest.samples[0];
  const inference = await postMultipart(
    fetchImpl,
    `${apiBaseUrl}/models/${model.id}/infer`,
    seedRoot,
    sample,
  );

  if (!Array.isArray(inference.detections) || inference.detections.length === 0) {
    throw new Error('Object recognition smoke failed: no detections returned');
  }

  const overlay = await postJson(
    fetchImpl,
    `${apiBaseUrl}/inference-runs/${inference.run_id}/overlay`,
    {},
  );
  const summary = {
    dataset_id: seed.dataset_id,
    dataset_version_id: seed.version_id,
    training_job_id: trainingJob.id,
    model_id: model.id,
    inference_run_id: inference.run_id,
    detected_classes: inference.detections.map((detection) => detection.class_name),
    overlay_artifact_uri: overlay.artifact_uri,
    status: 'object_recognition_smoke_passed',
  };

  stdout(`${JSON.stringify(summary, null, 2)}\n`);

  return 0;
}

async function getJson(fetchImpl, url) {
  const response = await fetchImpl(url);
  return parseResponse(response, url);
}

async function postJson(fetchImpl, url, payload) {
  const response = await fetchImpl(url, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  });
  return parseResponse(response, url);
}

async function patchJson(fetchImpl, url, payload) {
  const response = await fetchImpl(url, {
    method: 'PATCH',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  });
  return parseResponse(response, url);
}

async function postMultipart(fetchImpl, url, seedRoot, sample) {
  const bytes = await fs.readFile(path.join(seedRoot, sample.path));
  const form = new FormData();
  form.append('confidence_threshold', '0.50');
  form.append('image', new Blob([bytes], { type: sample.mime_type }), sample.filename);
  const response = await fetchImpl(url, {
    method: 'POST',
    body: form,
  });

  return parseResponse(response, sample.path);
}

async function parseResponse(response, context) {
  const text = await response.text();
  if (response.status < 200 || response.status >= 300) {
    throw new Error(`Fire demo request failed for ${context}: ${response.status} ${text}`);
  }

  return JSON.parse(text);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    process.exitCode = await fireDemoProduct();
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
