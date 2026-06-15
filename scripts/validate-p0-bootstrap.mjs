import { readdirSync } from 'node:fs';
import { join, relative } from 'node:path';

import { validateP0BootstrapPaths } from './p0-bootstrap-policy.mjs';

const ignoredDirectories = new Set([
  '.git',
  '.mypy_cache',
  '.perceptionlab',
  '.pytest_cache',
  '.ruff_cache',
  '.venv',
  'api/target',
  'node_modules',
  'worker/.venv',
]);

function collectPaths(directory, rootDirectory = directory) {
  const entries = readdirSync(directory, { withFileTypes: true });
  const paths = [];

  for (const entry of entries) {
    const absolutePath = join(directory, entry.name);
    const relativePath = relative(rootDirectory, absolutePath).replaceAll('\\', '/');

    if (entry.isDirectory() && ignoredDirectories.has(relativePath)) {
      continue;
    }

    paths.push(relativePath);

    if (entry.isDirectory()) {
      paths.push(...collectPaths(absolutePath, rootDirectory));
    }
  }

  return paths;
}

const result = validateP0BootstrapPaths(collectPaths(process.cwd()));

if (!result.valid) {
  console.error('P0 bootstrap validation failed:');
  for (const error of result.errors) {
    console.error(`- ${error}`);
  }
  process.exit(1);
}

console.log('P0 bootstrap files validated.');
