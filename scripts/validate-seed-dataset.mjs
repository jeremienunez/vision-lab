import { validateSeedDataset } from './seed-dataset-policy.mjs';

const errors = validateSeedDataset();

if (errors.length > 0) {
  console.error(errors.join('\n'));
  process.exit(1);
}

console.log('Seed demo dataset validated.');
