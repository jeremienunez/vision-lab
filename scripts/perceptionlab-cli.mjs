#!/usr/bin/env node
import fs from 'node:fs';
import { pathToFileURL } from 'node:url';

const defaultBaseUrl = process.env.PERCEPTIONLAB_API_BASE_URL ?? 'http://127.0.0.1:8080';
const defaultApiKey = process.env.PERCEPTIONLAB_API_KEY;

const usage = `Usage:
  node scripts/perceptionlab-cli.mjs [--base-url URL] health
  node scripts/perceptionlab-cli.mjs [--base-url URL] datasets
  node scripts/perceptionlab-cli.mjs [--base-url URL] models
  node scripts/perceptionlab-cli.mjs [--base-url URL] create-dataset --name NAME --classes a,b [--description TEXT]
  node scripts/perceptionlab-cli.mjs openapi
`;

export async function runCli(argv, dependencies = {}) {
  const {
    fetchImpl = globalThis.fetch,
    stdout = (value) => process.stdout.write(value),
    stderr = (value) => process.stderr.write(value),
    readFile = (path) => fs.readFileSync(path, 'utf8'),
    apiKey = defaultApiKey,
  } = dependencies;

  const parsed = parseCli(argv);
  if (parsed.error) {
    stderr(`${parsed.error}\n\n${usage}`);
    return 2;
  }

  if (parsed.help) {
    stdout(usage);
    return 0;
  }

  if (parsed.command === 'openapi') {
    stdout(formatJsonText(readFile('contracts/openapi.json')));
    return 0;
  }

  if (typeof fetchImpl !== 'function') {
    stderr('No fetch implementation available.\n');
    return 1;
  }

  const request = buildRequest(parsed, apiKey);
  if (request.error) {
    stderr(`${request.error}\n\n${usage}`);
    return 2;
  }

  const response = await fetchImpl(request.url, request.options);
  const body = await response.text();
  stdout(formatJsonText(body));

  if (response.status < 200 || response.status >= 300) {
    return 1;
  }

  return 0;
}

function parseCli(argv) {
  const args = [...argv];
  const options = { baseUrl: defaultBaseUrl };

  while (args[0]?.startsWith('--')) {
    const flag = args.shift();
    if (flag === '--help' || flag === '-h') {
      return { help: true };
    }
    if (flag === '--base-url') {
      const value = args.shift();
      if (!value) {
        return { error: '--base-url requires a value' };
      }
      options.baseUrl = value;
      continue;
    }
    return { error: `Unknown option: ${flag}` };
  }

  const command = args.shift();
  if (!command) {
    return { error: 'Missing command' };
  }

  return {
    command,
    options,
    commandOptions: parseCommandOptions(args),
  };
}

function parseCommandOptions(args) {
  const options = {};

  for (let index = 0; index < args.length; index += 1) {
    const flag = args[index];
    const value = args[index + 1];
    if (!flag?.startsWith('--') || value === undefined || value.startsWith('--')) {
      return { error: `Invalid option near ${flag ?? 'end of command'}` };
    }
    options[flag.slice(2).replaceAll('-', '_')] = value;
    index += 1;
  }

  return options;
}

function buildRequest(parsed, apiKey) {
  if (parsed.commandOptions.error) {
    return { error: parsed.commandOptions.error };
  }

  const baseUrl = parsed.options.baseUrl.replace(/\/+$/, '');

  switch (parsed.command) {
    case 'health':
      return getRequest(`${baseUrl}/health`);
    case 'datasets':
      return getRequest(`${baseUrl}/datasets`, apiKey);
    case 'models':
      return getRequest(`${baseUrl}/models`, apiKey);
    case 'create-dataset':
      return createDatasetRequest(baseUrl, parsed.commandOptions, apiKey);
    default:
      return { error: `Unknown command: ${parsed.command}` };
  }
}

function getRequest(url, apiKey) {
  return {
    url,
    options: { method: 'GET', headers: authHeaders(apiKey) },
  };
}

function createDatasetRequest(baseUrl, options, apiKey) {
  if (!options.name) {
    return { error: 'create-dataset requires --name' };
  }
  if (!options.classes) {
    return { error: 'create-dataset requires --classes' };
  }

  const classes = options.classes
    .split(',')
    .map((className) => className.trim())
    .filter(Boolean);
  if (classes.length === 0) {
    return { error: 'create-dataset requires at least one class' };
  }

  return {
    url: `${baseUrl}/datasets`,
    options: {
      method: 'POST',
      headers: jsonHeaders(apiKey),
      body: JSON.stringify({
        name: options.name,
        description: options.description ?? null,
        task_type: options.task_type ?? 'object_detection',
        classes,
      }),
    },
  };
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

function formatJsonText(text) {
  if (!text) {
    return '\n';
  }

  try {
    return `${JSON.stringify(JSON.parse(text), null, 2)}\n`;
  } catch {
    return text.endsWith('\n') ? text : `${text}\n`;
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.exitCode = await runCli(process.argv.slice(2));
}
