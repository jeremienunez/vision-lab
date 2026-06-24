import assert from 'node:assert/strict';
import { Given, Then, When } from '@cucumber/cucumber';

import { buildApiHeaders } from '../../web/src/dashboard/dashboard-data.js';
import { fireDemoProduct } from '../../scripts/fire-demo-product.mjs';
import { apiResponseFor, fireDemoFetch } from '../support/api-client.mjs';
import { assertResponseErrorCode, assertResponseStatus } from '../support/assertions.mjs';

Given('the PerceptionLab API is running', function () {
  this.apiKey = '';
});

Given('the PerceptionLab API is running with API key {string}', function (apiKey) {
  this.apiKey = apiKey;
});

Given('the PerceptionLab API is configured with an API key', function () {
  this.apiKey = 'dev-secret';
});

Given('the local transient API can start', function () {
  this.fireSummary = null;
});

When('I call GET {string}', async function (path) {
  this.response = apiResponseFor({ path, configuredApiKey: this.apiKey });
});

When('I call GET {string} without an API key', async function (path) {
  this.response = apiResponseFor({ path, configuredApiKey: this.apiKey });
});

When('I call GET {string} with API key {string}', async function (path, providedApiKey) {
  this.response = apiResponseFor({
    path,
    configuredApiKey: this.apiKey,
    providedApiKey,
  });
});

When('I save the API key in the dashboard configuration panel', function () {
  this.dashboardHeaders = buildApiHeaders(this.apiKey);
});

When('I run the product fire smoke', async function () {
  let output = '';
  const smokeApi = fireDemoFetch();

  await fireDemoProduct({
    baseUrl: 'http://api.local',
    apiKey: 'local-secret',
    fetchImpl: smokeApi.fetch,
    stdout: (value) => {
      output += value;
    },
  });

  this.fireSummary = JSON.parse(output);
});

Then('the response status should be {int}', function (expectedStatus) {
  assertResponseStatus(this.response, expectedStatus);
});

Then('the response body should contain database, storage, and queue status', async function () {
  const body = JSON.parse(await this.response.text());

  assert.equal(body.dependencies.database, 'ready');
  assert.equal(body.dependencies.storage, 'ready');
  assert.equal(body.dependencies.queue, 'ready');
});

Then('the response body should contain error code {string}', async function (expectedCode) {
  await assertResponseErrorCode(this.response, expectedCode);
});

Then('the response should not be {int}', function (unexpectedStatus) {
  assert.notEqual(this.response?.status, unexpectedStatus);
});

Then('protected dashboard API requests should include the x-api-key header', function () {
  assert.equal(this.dashboardHeaders['x-api-key'], this.apiKey);
});

Then('the summary should include detected classes and an overlay artifact URI', function () {
  assert.deepEqual(this.fireSummary.detected_classes, ['cup', 'book']);
  assert.match(this.fireSummary.overlay_artifact_uri, /^file:\/\/.+\.svg$/);
  assert.equal(this.fireSummary.status, 'object_recognition_smoke_passed');
});
