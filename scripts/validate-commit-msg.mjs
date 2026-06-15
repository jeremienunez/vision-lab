import { readFileSync } from 'node:fs';

import { validateCommitMessage } from './commit-message-policy.mjs';

const commitMessageFile = process.argv[2];

if (!commitMessageFile) {
  console.error('Missing commit message file path.');
  process.exit(2);
}

const message = readFileSync(commitMessageFile, 'utf8');
const result = validateCommitMessage(message);

if (!result.valid) {
  console.error('Invalid commit message:');
  for (const error of result.errors) {
    console.error(`- ${error}`);
  }
  process.exit(1);
}
