import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

import { validateFeatureDocument } from './bdd-feature-policy.mjs';

const featureDirectory = join(process.cwd(), 'qa', 'features');

function collectFeatureFiles(directory) {
  const entries = readdirSync(directory, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    const entryPath = join(directory, entry.name);

    if (entry.isDirectory()) {
      if (entry.name === 'fixtures' || entry.name === 'reports') {
        continue;
      }
      files.push(...collectFeatureFiles(entryPath));
      continue;
    }

    if (entry.isFile() && entry.name.endsWith('.feature')) {
      files.push(entryPath);
    }
  }

  return files.sort();
}

const featureFiles = collectFeatureFiles(featureDirectory);

if (featureFiles.length === 0) {
  console.error('No BDD feature files found in qa/features.');
  process.exit(1);
}

const failures = [];

for (const featureFile of featureFiles) {
  const result = validateFeatureDocument(readFileSync(featureFile, 'utf8'));

  if (!result.valid) {
    failures.push({ featureFile, errors: result.errors });
  }
}

if (failures.length > 0) {
  console.error('Invalid BDD feature files:');
  for (const failure of failures) {
    console.error(`- ${failure.featureFile}`);
    for (const error of failure.errors) {
      console.error(`  - ${error}`);
    }
  }
  process.exit(1);
}

console.log(`Validated ${featureFiles.length} BDD feature file(s).`);
