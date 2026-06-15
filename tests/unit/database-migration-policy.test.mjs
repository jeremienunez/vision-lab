import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';

const initialMigrationPath = 'api/migrations/0001_initial_schema.sql';
const policyScriptPath = 'scripts/database-migration-policy.mjs';

function readInitialMigration() {
  assert.equal(existsSync(initialMigrationPath), true, `${initialMigrationPath} must exist`);
  return readFileSync(initialMigrationPath, 'utf8');
}

describe('database migration policy', () => {
  it('exposes an initial schema validator for quality gates', async () => {
    assert.equal(existsSync(policyScriptPath), true, `${policyScriptPath} must exist`);

    const policy = await import('../../scripts/database-migration-policy.mjs');
    const result = policy.validateInitialSchemaMigration([
      {
        path: initialMigrationPath,
        content: 'SELECT 1;',
      },
    ]);

    assert.equal(policy.initialSchemaMigrationPath, initialMigrationPath);
    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /Missing required initial schema table/);
  });

  it('creates all initial PostgreSQL tables needed by the MVP flow', () => {
    const sql = readInitialMigration();

    for (const tableName of [
      'datasets',
      'dataset_classes',
      'samples',
      'annotations',
      'dataset_versions',
      'dataset_version_samples',
      'dataset_version_annotations',
      'training_jobs',
      'training_job_queue',
      'job_events',
      'training_metrics',
      'models',
      'model_exports',
      'inference_runs',
      'artifacts',
    ]) {
      assert.match(sql, new RegExp(`CREATE TABLE ${tableName}\\b`, 'i'));
    }
  });

  it('keeps lifecycle, versioning, annotation, and artifact invariants in the schema', () => {
    const sql = readInitialMigration();

    for (const marker of [
      'CREATE EXTENSION IF NOT EXISTS pgcrypto',
      'CONSTRAINT datasets_name_key',
      'CONSTRAINT dataset_status_check',
      'CONSTRAINT dataset_task_type_check',
      'CONSTRAINT sample_dataset_checksum_key',
      'CONSTRAINT annotation_bbox_bounds_check',
      'CONSTRAINT annotation_class_fk',
      'dataset_version_immutable',
      'CONSTRAINT training_job_status_check',
      'CONSTRAINT training_job_timestamps_check',
      'CONSTRAINT training_job_queue_status_check',
      'CONSTRAINT model_status_check',
      'CONSTRAINT model_training_job_key',
      'CONSTRAINT export_status_check',
      'CONSTRAINT artifact_owner_type_check',
    ]) {
      assert.match(sql, new RegExp(marker, 'i'));
    }
  });

  it('runs migration validation as part of the local quality gate', () => {
    const packageJson = JSON.parse(readFileSync('package.json', 'utf8'));

    assert.equal(
      packageJson.scripts['validate:migrations'],
      'node scripts/validate-database-migrations.mjs',
    );
    assert.match(packageJson.scripts.quality, /npm run validate:migrations/);
  });
});
