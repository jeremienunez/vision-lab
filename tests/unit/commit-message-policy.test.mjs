import { describe, it } from 'node:test';
import assert from 'node:assert/strict';

import { validateCommitMessage } from '../../scripts/commit-message-policy.mjs';

describe('commit message policy', () => {
  it('accepts a conventional commit with type, scope, and subject', () => {
    const result = validateCommitMessage('feat(vision): add ingestion pipeline');

    assert.equal(result.valid, true);
    assert.deepEqual(result.errors, []);
  });

  it('rejects a commit message without conventional type shape', () => {
    const result = validateCommitMessage('add ingestion pipeline');

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /type\(scope\): subject/);
  });

  it('rejects unsupported commit types', () => {
    const result = validateCommitMessage('feature(vision): add ingestion pipeline');

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /unsupported type/);
  });

  it('rejects subjects that are too long for readable history', () => {
    const result = validateCommitMessage(
      'docs(sprints): add a very long sprint validation title that cannot stay readable in git history',
    );

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /72 characters/);
  });
});
