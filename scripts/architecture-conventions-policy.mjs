export const requiredArchitecturePaths = [
  'api/crates/perception_domain/src',
  'api/crates/perception_app/src/ports',
  'api/crates/perception_app/src/use_cases',
  'api/crates/perception_infra/src/postgres',
  'api/crates/perception_infra/src/storage',
  'api/crates/perception_infra/src/queue',
  'api/crates/perception_infra/src/config',
  'api/crates/perception_http/src/routes',
  'api/crates/perception_http/src/dto',
  'api/crates/perception_http/src/mappers',
  'api/crates/perception_api/src',
  'worker/perception_worker/domain',
  'worker/perception_worker/contracts',
  'worker/perception_worker/app',
  'worker/perception_worker/ports',
  'worker/perception_worker/adapters/db',
  'worker/perception_worker/adapters/storage',
  'worker/perception_worker/adapters/training',
  'worker/perception_worker/adapters/inference',
  'worker/perception_worker/adapters/export',
  'worker/perception_worker/entrypoints',
  'qa/features',
  'qa/steps',
  'qa/support',
  'qa/fixtures',
  'contracts',
  'doc/architecture/design-patterns.md',
  'doc/architecture/review-checklist.md',
  'doc/architecture/adr/0001-use-hexagonal-architecture.md',
  'doc/architecture/adr/0002-use-postgresql-backed-queue-for-mvp.md',
  'doc/architecture/adr/0003-use-strategy-for-training-and-inference-modes.md',
];

const forbiddenLegacyPaths = ['apps/api-rust', 'workers/pytorch-trainer', 'src'];
const forbiddenBaseNames = new Set(['utils', 'helpers', 'misc', 'common', 'manager', 'service']);

function normalizePath(path) {
  return String(path ?? '').replaceAll('\\', '/').replace(/^\.\//, '').replace(/\/+$/, '');
}

function baseNameWithoutExtension(path) {
  const normalized = normalizePath(path);
  const baseName = normalized.split('/').at(-1) ?? '';
  return baseName.replace(/\.[^.]+$/, '');
}

function pathExists(pathSet, requiredPath) {
  const normalizedRequiredPath = normalizePath(requiredPath);
  return [...pathSet].some(
    (path) => path === normalizedRequiredPath || path.startsWith(`${normalizedRequiredPath}/`),
  );
}

export function validateArchitectureConventions(paths) {
  const normalizedPaths = paths.map(normalizePath).filter(Boolean);
  const pathSet = new Set(normalizedPaths);
  const errors = [];

  for (const requiredPath of requiredArchitecturePaths) {
    if (!pathExists(pathSet, requiredPath)) {
      errors.push(`Missing required architecture path: ${requiredPath}.`);
    }
  }

  for (const path of normalizedPaths) {
    for (const forbiddenLegacyPath of forbiddenLegacyPaths) {
      if (path === forbiddenLegacyPath || path.startsWith(`${forbiddenLegacyPath}/`)) {
        errors.push(`Forbidden legacy architecture path "${forbiddenLegacyPath}" in path: ${path}.`);
      }
    }

    const segments = path.split('/');
    for (const segment of segments) {
      const segmentBaseName = segment.replace(/\.[^.]+$/, '');
      if (forbiddenBaseNames.has(segmentBaseName)) {
        errors.push(`Forbidden vague architecture name "${segment}" in path: ${path}.`);
      }
    }

    const baseName = baseNameWithoutExtension(path);
    if (forbiddenBaseNames.has(baseName)) {
      errors.push(`Forbidden vague architecture file name in path: ${path}.`);
    }
  }

  return {
    valid: errors.length === 0,
    errors: [...new Set(errors)],
  };
}
