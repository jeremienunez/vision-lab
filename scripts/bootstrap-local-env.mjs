import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { buildLocalEnvContent } from './p0-bootstrap-policy.mjs';

const scriptDirectory = dirname(fileURLToPath(import.meta.url));
const projectRoot = resolve(scriptDirectory, '..');

const localDirectories = [
  join(projectRoot, 'datasets', 'seed'),
  join(projectRoot, '.perceptionlab', 'storage'),
  join(projectRoot, '.perceptionlab', 'artifacts'),
  join(projectRoot, '.perceptionlab', 'tmp'),
  join(projectRoot, '.perceptionlab', 'cache', 'uv'),
];

for (const directory of localDirectories) {
  mkdirSync(directory, { recursive: true });
}

writeFileSync(join(projectRoot, '.env.local'), buildLocalEnvContent(projectRoot), 'utf8');

console.log(`Generated ${join(projectRoot, '.env.local')}`);
