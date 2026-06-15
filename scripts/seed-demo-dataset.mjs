#!/usr/bin/env node
import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { loadSeedManifest } from './seed-dataset-policy.mjs';

const defaultBaseUrl = process.env.PERCEPTIONLAB_API_BASE_URL ?? 'http://127.0.0.1:8080';
const defaultSeedRoot = process.env.PERCEPTIONLAB_SEED_DATASET_ROOT ?? 'datasets/seed';

export async function seedDemoDataset(dependencies = {}) {
  const {
    baseUrl = defaultBaseUrl,
    seedRoot = defaultSeedRoot,
    fetchImpl = globalThis.fetch,
    stdout = (value) => process.stdout.write(value),
  } = dependencies;

  const apiBaseUrl = baseUrl.replace(/\/+$/, '');
  const manifest = loadSeedManifest(seedRoot);
  const dataset = await postJson(fetchImpl, `${apiBaseUrl}/datasets`, manifest.dataset);
  const samples = [];

  for (const sample of manifest.samples) {
    const uploadedSample = await postSample(fetchImpl, apiBaseUrl, seedRoot, dataset.id, sample);
    samples.push(uploadedSample);

    for (const annotation of sample.annotations) {
      await postJson(fetchImpl, `${apiBaseUrl}/samples/${uploadedSample.id}/annotations`, {
        class_name: annotation.class_name,
        bbox: annotation.bbox,
        confidence: annotation.confidence,
      });
    }
  }

  const version = await postJson(
    fetchImpl,
    `${apiBaseUrl}/datasets/${dataset.id}/versions`,
    manifest.version,
  );

  stdout(
    `${JSON.stringify(
      {
        dataset_id: dataset.id,
        dataset_name: manifest.dataset.name,
        version_id: version.id,
        version_name: version.version_name,
        sample_count: samples.length,
        annotation_count: manifest.samples.reduce(
          (count, sample) => count + sample.annotations.length,
          0,
        ),
      },
      null,
      2,
    )}\n`,
  );

  return 0;
}

async function postJson(fetchImpl, url, payload) {
  const response = await fetchImpl(url, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  });
  return parseResponse(response, url);
}

async function postSample(fetchImpl, apiBaseUrl, seedRoot, datasetId, sample) {
  const bytes = await fs.readFile(path.join(seedRoot, sample.path));
  const form = new FormData();
  form.append('width', String(sample.width));
  form.append('height', String(sample.height));
  form.append('file', new Blob([bytes], { type: sample.mime_type }), sample.filename);

  const response = await fetchImpl(`${apiBaseUrl}/datasets/${datasetId}/samples`, {
    method: 'POST',
    body: form,
  });
  return parseResponse(response, sample.path);
}

async function parseResponse(response, context) {
  const text = await response.text();
  if (response.status < 200 || response.status >= 300) {
    throw new Error(`Seed request failed for ${context}: ${response.status} ${text}`);
  }

  return JSON.parse(text);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    process.exitCode = await seedDemoDataset();
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
