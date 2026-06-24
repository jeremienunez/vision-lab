import assert from 'node:assert/strict';

export function assertResponseStatus(response, expectedStatus) {
  assert.equal(response?.status, expectedStatus);
}

export async function assertResponseErrorCode(response, expectedCode) {
  const body = JSON.parse(await response.text());
  assert.equal(body.error?.code, expectedCode);
}
