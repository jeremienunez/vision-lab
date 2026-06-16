#!/usr/bin/env node
import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { loadSeedManifest } from './seed-dataset-policy.mjs';
import { seedDemoDataset } from './seed-demo-dataset.mjs';

const defaultBaseUrl = process.env.PERCEPTIONLAB_API_BASE_URL ?? 'http://127.0.0.1:8080';
const defaultSeedRoot = process.env.PERCEPTIONLAB_SEED_DATASET_ROOT ?? 'datasets/seed';
const defaultImagePath = process.env.PERCEPTIONLAB_FIRE_IMAGE_PATH;
const defaultModelArtifactUri =
  process.env.PERCEPTIONLAB_FIRE_MODEL_ARTIFACT_URI ?? 'file:///tmp/perceptionlab/demo-model.pt';
const defaultConfidenceThreshold = process.env.PERCEPTIONLAB_FIRE_CONFIDENCE_THRESHOLD ?? '0.25';
const defaultApiKey = process.env.PERCEPTIONLAB_API_KEY;

export async function fireDemoProduct(dependencies = {}) {
  const {
    baseUrl = defaultBaseUrl,
    seedRoot = defaultSeedRoot,
    imagePath = defaultImagePath,
    modelArtifactUri = defaultModelArtifactUri,
    confidenceThreshold = defaultConfidenceThreshold,
    apiKey = defaultApiKey,
    fetchImpl = globalThis.fetch,
    stdout = (value) => process.stdout.write(value),
  } = dependencies;

  const apiBaseUrl = baseUrl.replace(/\/+$/, '');
  const manifest = loadSeedManifest(seedRoot);
  const inferenceImage = await resolveInferenceImage(imagePath, seedRoot, manifest);

  await getJson(fetchImpl, `${apiBaseUrl}/health`);

  let seedOutput = '';
  await seedDemoDataset({
    baseUrl: apiBaseUrl,
    seedRoot,
    apiKey,
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
  }, apiKey);

  await patchJson(fetchImpl, `${apiBaseUrl}/training-jobs/${trainingJob.id}/status`, {
    next_status: 'running',
    error_message: null,
  }, apiKey);
  await patchJson(fetchImpl, `${apiBaseUrl}/training-jobs/${trainingJob.id}/status`, {
    next_status: 'succeeded',
    error_message: null,
  }, apiKey);

  const model = await postJson(fetchImpl, `${apiBaseUrl}/models`, {
    training_job_id: trainingJob.id,
    name: 'desk-objects-demo',
    version: 'v1',
    artifact_uri: modelArtifactUri,
    metrics_summary: {
      mAP50: '0.91',
      classes: manifest.dataset.classes.join(','),
    },
  }, apiKey);
  const inference = await postMultipart(
    fetchImpl,
    `${apiBaseUrl}/models/${model.id}/infer`,
    inferenceImage,
    confidenceThreshold,
    apiKey,
  );

  if (!Array.isArray(inference.detections) || inference.detections.length === 0) {
    throw new Error('Object recognition smoke failed: no detections returned');
  }

  const overlay = await postJson(
    fetchImpl,
    `${apiBaseUrl}/inference-runs/${inference.run_id}/overlay`,
    {},
    apiKey,
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

export function parseFireDemoOptions(argv) {
  const options = {};

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];

    if (arg === '--image') {
      const imagePath = argv[index + 1];
      if (!imagePath || imagePath.startsWith('--')) {
        throw new Error('Missing value for --image');
      }
      options.imagePath = imagePath;
      index += 1;
      continue;
    }

    if (arg.startsWith('--image=')) {
      const imagePath = arg.slice('--image='.length);
      if (!imagePath) {
        throw new Error('Missing value for --image');
      }
      options.imagePath = imagePath;
      continue;
    }

    if (arg === '--confidence-threshold') {
      const confidenceThreshold = argv[index + 1];
      if (!confidenceThreshold || confidenceThreshold.startsWith('--')) {
        throw new Error('Missing value for --confidence-threshold');
      }
      options.confidenceThreshold = confidenceThreshold;
      index += 1;
      continue;
    }

    if (arg.startsWith('--confidence-threshold=')) {
      const confidenceThreshold = arg.slice('--confidence-threshold='.length);
      if (!confidenceThreshold) {
        throw new Error('Missing value for --confidence-threshold');
      }
      options.confidenceThreshold = confidenceThreshold;
      continue;
    }

    throw new Error(`Unknown fire demo option: ${arg}`);
  }

  return options;
}

async function getJson(fetchImpl, url) {
  const response = await fetchImpl(url);
  return parseResponse(response, url);
}

async function postJson(fetchImpl, url, payload, apiKey) {
  const response = await fetchImpl(url, {
    method: 'POST',
    headers: jsonHeaders(apiKey),
    body: JSON.stringify(payload),
  });
  return parseResponse(response, url);
}

async function patchJson(fetchImpl, url, payload, apiKey) {
  const response = await fetchImpl(url, {
    method: 'PATCH',
    headers: jsonHeaders(apiKey),
    body: JSON.stringify(payload),
  });
  return parseResponse(response, url);
}

async function postMultipart(fetchImpl, url, image, confidenceThreshold, apiKey) {
  const bytes = await fs.readFile(image.path);
  const form = new FormData();
  form.append('confidence_threshold', String(confidenceThreshold));
  form.append('image', new Blob([bytes], { type: image.mime_type }), image.filename);
  const response = await fetchImpl(url, {
    method: 'POST',
    headers: authHeaders(apiKey),
    body: form,
  });

  return parseResponse(response, image.path);
}

function jsonHeaders(apiKey) {
  return {
    'content-type': 'application/json',
    ...authHeaders(apiKey),
  };
}

function authHeaders(apiKey) {
  const normalizedApiKey = apiKey?.trim();
  return normalizedApiKey ? { 'x-api-key': normalizedApiKey } : {};
}

async function resolveInferenceImage(imagePath, seedRoot, manifest) {
  if (!imagePath) {
    const sample = manifest.samples[0];
    return {
      path: path.join(seedRoot, sample.path),
      filename: sample.filename,
      mime_type: sample.mime_type,
    };
  }

  const absolutePath = path.resolve(imagePath);
  let stats;
  try {
    stats = await fs.stat(absolutePath);
  } catch (error) {
    if (error.code === 'ENOENT') {
      throw new Error(`Input image not found: ${absolutePath}`);
    }
    throw error;
  }

  if (!stats.isFile()) {
    throw new Error(`Input image is not a file: ${absolutePath}`);
  }

  return {
    path: absolutePath,
    filename: path.basename(absolutePath),
    mime_type: mimeTypeForImagePath(absolutePath),
  };
}

function mimeTypeForImagePath(imagePath) {
  const extension = path.extname(imagePath).toLowerCase();
  if (extension === '.jpg' || extension === '.jpeg') return 'image/jpeg';
  if (extension === '.png') return 'image/png';
  if (extension === '.webp') return 'image/webp';

  throw new Error('Input image must be a .jpg, .jpeg, .png, or .webp file');
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
    process.exitCode = await fireDemoProduct(parseFireDemoOptions(process.argv.slice(2)));
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
