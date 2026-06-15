import {
  loadOpenApiContract,
  validateOpenApiContract,
} from './openapi-contract-policy.mjs';

const errors = validateOpenApiContract(loadOpenApiContract());

if (errors.length > 0) {
  console.error(errors.join('\n'));
  process.exit(1);
}

console.log('OpenAPI contract validated.');
