import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

export const initialSchemaMigrationPath = 'api/migrations/0001_initial_schema.sql';

export const requiredInitialSchemaTables = [
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
];

export const requiredInitialSchemaMarkers = [
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
];

function normalizePath(path) {
  return String(path ?? '').replaceAll('\\', '/').replace(/^\.\//, '');
}

function createTableRegex(tableName) {
  return new RegExp(`\\bCREATE\\s+TABLE\\s+${tableName}\\b`, 'i');
}

function markerRegex(marker) {
  const escaped = marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  return new RegExp(escaped.replaceAll('\\ ', '\\s+'), 'i');
}

export function loadInitialSchemaMigration(projectRoot = process.cwd()) {
  const absolutePath = join(projectRoot, initialSchemaMigrationPath);

  if (!existsSync(absolutePath)) {
    return [];
  }

  return [
    {
      path: initialSchemaMigrationPath,
      content: readFileSync(absolutePath, 'utf8'),
    },
  ];
}

export function validateInitialSchemaMigration(files) {
  const migration = files.find(
    (file) => normalizePath(file.path) === initialSchemaMigrationPath,
  );
  const errors = [];

  if (!migration) {
    return {
      valid: false,
      errors: [`Missing required initial schema migration: ${initialSchemaMigrationPath}.`],
    };
  }

  for (const tableName of requiredInitialSchemaTables) {
    if (!createTableRegex(tableName).test(migration.content)) {
      errors.push(`Missing required initial schema table: ${tableName}.`);
    }
  }

  for (const marker of requiredInitialSchemaMarkers) {
    if (!markerRegex(marker).test(migration.content)) {
      errors.push(`Missing required initial schema marker: ${marker}.`);
    }
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
