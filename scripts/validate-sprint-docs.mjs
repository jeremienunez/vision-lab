import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

import { validateSprintDocument } from './sprint-document-policy.mjs';

const sprintDirectory = join(process.cwd(), 'doc', 'sprints');
const sprintFiles = readdirSync(sprintDirectory)
  .filter((fileName) => fileName.endsWith('.md'))
  .sort();

if (sprintFiles.length === 0) {
  console.error('No sprint documents found in doc/sprints.');
  process.exit(1);
}

const failures = [];

for (const fileName of sprintFiles) {
  const filePath = join(sprintDirectory, fileName);
  const result = validateSprintDocument(readFileSync(filePath, 'utf8'));

  if (!result.valid) {
    failures.push({ fileName, errors: result.errors });
  }
}

if (failures.length > 0) {
  console.error('Invalid sprint documents:');
  for (const failure of failures) {
    console.error(`- ${failure.fileName}`);
    for (const error of failure.errors) {
      console.error(`  - ${error}`);
    }
  }
  process.exit(1);
}

console.log(`Validated ${sprintFiles.length} sprint document(s).`);
