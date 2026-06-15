import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { runCli } from '../../scripts/perceptionlab-cli.mjs';

function response(body, status = 200) {
  return {
    status,
    async text() {
      return JSON.stringify(body);
    },
  };
}

describe('PerceptionLab CLI', () => {
  it('calls the health endpoint with the configured base URL', async () => {
    const calls = [];
    let output = '';

    const code = await runCli(['--base-url', 'http://api.local', 'health'], {
      fetchImpl: async (url, options) => {
        calls.push([url, options]);
        return response({ status: 'healthy' });
      },
      stdout: (value) => {
        output += value;
      },
    });

    assert.equal(code, 0);
    assert.equal(calls[0][0], 'http://api.local/health');
    assert.equal(calls[0][1].method, 'GET');
    assert.deepEqual(JSON.parse(output), { status: 'healthy' });
  });

  it('creates a dataset with parsed classes', async () => {
    const calls = [];

    const code = await runCli(
      [
        '--base-url',
        'http://api.local/',
        'create-dataset',
        '--name',
        'desk-objects-v1',
        '--description',
        'Desk demo',
        '--classes',
        'cup, book',
      ],
      {
        fetchImpl: async (url, options) => {
          calls.push([url, options]);
          return response({ id: 'ds_01', name: 'desk-objects-v1' }, 201);
        },
        stdout: () => {},
      },
    );

    assert.equal(code, 0);
    assert.equal(calls[0][0], 'http://api.local/datasets');
    assert.equal(calls[0][1].method, 'POST');
    assert.equal(calls[0][1].headers['content-type'], 'application/json');
    assert.deepEqual(JSON.parse(calls[0][1].body), {
      name: 'desk-objects-v1',
      description: 'Desk demo',
      task_type: 'object_detection',
      classes: ['cup', 'book'],
    });
  });

  it('prints the published OpenAPI contract without calling the API', async () => {
    let output = '';
    let called = false;

    const code = await runCli(['openapi'], {
      fetchImpl: async () => {
        called = true;
        return response({});
      },
      stdout: (value) => {
        output += value;
      },
      readFile: () => '{"openapi":"3.1.0"}',
    });

    assert.equal(code, 0);
    assert.equal(called, false);
    assert.deepEqual(JSON.parse(output), { openapi: '3.1.0' });
  });

  it('returns a usage error for unknown commands', async () => {
    let errorOutput = '';

    const code = await runCli(['unknown'], {
      fetchImpl: async () => response({}),
      stdout: () => {},
      stderr: (value) => {
        errorOutput += value;
      },
    });

    assert.equal(code, 2);
    assert.match(errorOutput, /Unknown command/);
  });
});
