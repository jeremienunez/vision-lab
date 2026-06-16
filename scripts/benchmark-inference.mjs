#!/usr/bin/env node
import fs from 'node:fs/promises';
import { performance } from 'node:perf_hooks';
import { pathToFileURL } from 'node:url';

const defaultBaseUrl = process.env.PERCEPTIONLAB_API_BASE_URL ?? 'http://127.0.0.1:8080';
const defaultImagePath =
  process.env.PERCEPTIONLAB_BENCHMARK_IMAGE ?? 'datasets/seed/images/desk-objects.png';
const defaultApiKey = process.env.PERCEPTIONLAB_API_KEY;

const usage = `Usage:
  node scripts/benchmark-inference.mjs --model-id MODEL_ID [--image PATH] [--iterations N] [--base-url URL] [--confidence-threshold N]
`;

export async function runInferenceBenchmark(argv, dependencies = {}) {
  const {
    fetchImpl = globalThis.fetch,
    now = () => performance.now(),
    stdout = (value) => process.stdout.write(value),
    stderr = (value) => process.stderr.write(value),
    apiKey = defaultApiKey,
  } = dependencies;

  const options = parseOptions(argv);
  if (options.error) {
    stderr(`${options.error}\n\n${usage}`);
    return 2;
  }

  if (typeof fetchImpl !== 'function') {
    stderr('No fetch implementation available.\n');
    return 1;
  }

  const imageBytes = await fs.readFile(options.image);
  const clientLatencies = [];
  const apiLatencies = [];
  const url = `${options.baseUrl.replace(/\/+$/, '')}/models/${options.modelId}/infer`;

  for (let iteration = 0; iteration < options.iterations; iteration += 1) {
    const form = new FormData();
    form.append('confidence_threshold', String(options.confidenceThreshold));
    form.append('image', new Blob([imageBytes], { type: 'image/png' }), 'benchmark.png');

    const startedAt = now();
    const response = await fetchImpl(url, {
      method: 'POST',
      headers: authHeaders(apiKey),
      body: form,
    });
    const elapsedMs = now() - startedAt;
    const body = await parseResponse(response, url);

    clientLatencies.push(elapsedMs);
    if (typeof body.latency_ms === 'number') {
      apiLatencies.push(body.latency_ms);
    }
  }

  stdout(
    `${JSON.stringify(
      {
        model_id: options.modelId,
        image: options.image,
        iterations: options.iterations,
        confidence_threshold: options.confidenceThreshold,
        client_latency_ms: summarize(clientLatencies),
        api_latency_ms: summarize(apiLatencies),
      },
      null,
      2,
    )}\n`,
  );

  return 0;
}

function parseOptions(argv) {
  const flags = {};

  for (let index = 0; index < argv.length; index += 1) {
    const flag = argv[index];
    const value = argv[index + 1];
    if (!flag?.startsWith('--') || value === undefined || value.startsWith('--')) {
      return { error: `Invalid option near ${flag ?? 'end of command'}` };
    }
    flags[flag.slice(2).replaceAll('-', '_')] = value;
    index += 1;
  }

  if (!flags.model_id) {
    return { error: '--model-id is required' };
  }

  const iterations = Number.parseInt(flags.iterations ?? '10', 10);
  if (!Number.isInteger(iterations) || iterations <= 0) {
    return { error: '--iterations must be a positive integer' };
  }

  const confidenceThreshold = Number.parseFloat(flags.confidence_threshold ?? '0.25');
  if (!Number.isFinite(confidenceThreshold) || confidenceThreshold < 0 || confidenceThreshold > 1) {
    return { error: '--confidence-threshold must be between 0 and 1' };
  }

  return {
    baseUrl: flags.base_url ?? defaultBaseUrl,
    modelId: flags.model_id,
    image: flags.image ?? defaultImagePath,
    iterations,
    confidenceThreshold,
  };
}

async function parseResponse(response, context) {
  const text = await response.text();
  if (response.status < 200 || response.status >= 300) {
    throw new Error(`Benchmark request failed for ${context}: ${response.status} ${text}`);
  }

  return JSON.parse(text);
}

function summarize(values) {
  if (values.length === 0) {
    return { min: null, max: null, avg: null, p95: null };
  }

  const sorted = [...values].sort((left, right) => left - right);
  const total = sorted.reduce((sum, value) => sum + value, 0);
  const p95Index = Math.max(0, Math.ceil(sorted.length * 0.95) - 1);

  return {
    min: round(sorted[0]),
    max: round(sorted.at(-1)),
    avg: round(total / sorted.length),
    p95: round(sorted[p95Index]),
  };
}

function round(value) {
  return Math.round(value * 100) / 100;
}

function authHeaders(apiKey) {
  const normalizedApiKey = apiKey?.trim();
  return normalizedApiKey ? { 'x-api-key': normalizedApiKey } : {};
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    process.exitCode = await runInferenceBenchmark(process.argv.slice(2));
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
