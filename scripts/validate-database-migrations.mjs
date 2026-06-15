import {
  loadInitialSchemaMigration,
  validateInitialSchemaMigration,
} from './database-migration-policy.mjs';

const result = validateInitialSchemaMigration(loadInitialSchemaMigration(process.cwd()));

if (!result.valid) {
  console.error('Database migration validation failed:');
  for (const error of result.errors) {
    console.error(`- ${error}`);
  }
  process.exit(1);
}

console.log('Database migrations validated.');
