import { describe, it } from 'node:test';
import assert from 'node:assert/strict';

import { validateSprintDocument } from '../../scripts/sprint-document-policy.mjs';

const validSprint = `# Sprint 01 - MVP Vision Pipeline

## Goal
Deliver a first usable image ingestion and analysis workflow.

## Priority
P0

## Dependencies
- Foundation sprint complete.

## Scope
- Upload one image.
- Extract deterministic metadata.

## BDD Validation Criteria
### Scenario: Image metadata extraction
Given a supported image file is submitted
When the ingestion workflow runs
Then metadata is attached to the analysis result

## Definition of Done
- Tests are green.
- Architecture rules are green.
`;

describe('sprint document policy', () => {
  it('accepts a sprint with goal, scope, BDD criteria, and done criteria', () => {
    const result = validateSprintDocument(validSprint);

    assert.equal(result.valid, true);
    assert.deepEqual(result.errors, []);
  });

  it('rejects a sprint without BDD Then outcome', () => {
    const result = validateSprintDocument(validSprint.replace('Then metadata is attached to the analysis result', ''));

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /Then/);
  });

  it('rejects a sprint without a definition of done', () => {
    const result = validateSprintDocument(validSprint.replace('## Definition of Done', '## Completion'));

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /Definition of Done/);
  });

  it('rejects a sprint without product owner readiness metadata', () => {
    const result = validateSprintDocument(
      validSprint
        .replace('## Priority\nP0\n\n', '')
        .replace('## Dependencies\n- Foundation sprint complete.\n\n', ''),
    );

    assert.equal(result.valid, false);
    assert.match(result.errors.join('\n'), /Priority/);
    assert.match(result.errors.join('\n'), /Dependencies/);
  });
});
