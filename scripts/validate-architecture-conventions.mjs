import { readdirSync } from 'node:fs';
import { join, relative } from 'node:path';

import { validateArchitectureConventions } from './architecture-conventions-policy.mjs';

const ignoredDirectories = new Set(['.git', 'node_modules', 'coverage', 'dist', 'build']);

function collectPaths(directory, rootDirectory = directory) {
  const entries = readdirSync(directory, { withFileTypes: true });
  const paths = [];

  for (const entry of entries) {
    if (entry.isDirectory() && ignoredDirectories.has(entry.name)) {
      continue;
    }

    const absolutePath = join(directory, entry.name);
    const relativePath = relative(rootDirectory, absolutePath).replaceAll('\\', '/');
    paths.push(relativePath);

    if (entry.isDirectory()) {
      paths.push(...collectPaths(absolutePath, rootDirectory));
    }
  }

  return paths;
}

const result = validateArchitectureConventions(collectPaths(process.cwd()));

if (!result.valid) {
  console.error('Architecture convention violations:');
  for (const error of result.errors) {
    console.error(`- ${error}`);
  }
  process.exit(1);
}

console.log('Architecture conventions validated.');
