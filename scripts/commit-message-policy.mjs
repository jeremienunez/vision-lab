const allowedTypes = new Set([
  'feat',
  'fix',
  'docs',
  'test',
  'refactor',
  'perf',
  'build',
  'ci',
  'chore',
  'revert',
]);

const conventionalShape = /^(?<type>[a-z]+)(?:\((?<scope>[a-z0-9-]+)\))?: (?<subject>.+)$/;

export function validateCommitMessage(rawMessage) {
  const message = String(rawMessage ?? '').replaceAll('\r', '').trim();
  const firstLine = message.split('\n')[0] ?? '';
  const errors = [];

  if (firstLine.length === 0) {
    errors.push('Commit message is empty.');
    return { valid: false, errors };
  }

  if (firstLine.length > 72) {
    errors.push('Commit first line must stay within 72 characters.');
  }

  const match = firstLine.match(conventionalShape);

  if (!match?.groups) {
    errors.push('Use conventional commit shape: type(scope): subject.');
    return { valid: false, errors };
  }

  const { type, subject } = match.groups;

  if (!allowedTypes.has(type)) {
    errors.push(`unsupported type "${type}". Allowed types: ${Array.from(allowedTypes).join(', ')}.`);
  }

  if (subject.trim().length === 0) {
    errors.push('Commit subject is required.');
  }

  if (subject.endsWith('.')) {
    errors.push('Commit subject should not end with a period.');
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
