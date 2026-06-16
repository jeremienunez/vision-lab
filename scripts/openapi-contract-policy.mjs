import fs from 'node:fs';

export const requiredOperations = [
  ['get', '/health'],
  ['post', '/datasets'],
  ['get', '/datasets'],
  ['get', '/datasets/{dataset_id}/stats'],
  ['post', '/datasets/{dataset_id}/samples'],
  ['post', '/samples/{sample_id}/annotations'],
  ['get', '/samples/{sample_id}/annotations'],
  ['post', '/datasets/{dataset_id}/versions'],
  ['post', '/training-jobs'],
  ['get', '/training-jobs'],
  ['get', '/training-jobs/{training_job_id}/metrics'],
  ['get', '/training-jobs/{training_job_id}/metrics/by-class'],
  ['get', '/models'],
  ['get', '/models/{model_id}'],
  ['post', '/models/{model_id}/infer'],
  ['post', '/models/{model_id}/exports'],
  ['get', '/models/{model_id}/exports'],
  ['post', '/datasets/{dataset_id}/import/yolo'],
  ['get', '/datasets/{dataset_id}/export/yolo'],
  ['post', '/inference-runs/{run_id}/overlay'],
];

const requiredSchemas = [
  'DatasetResponse',
  'SampleResponse',
  'AnnotationResponse',
  'DatasetVersionResponse',
  'TrainingJobResponse',
  'ListTrainingJobsResponse',
  'TrainingMetricResponse',
  'TrainingClassMetricResponse',
  'ModelResponse',
  'InferenceResponse',
  'ModelExportResponse',
  'OverlayResponse',
  'YoloAnnotationImportResponse',
  'YoloAnnotationExportResponse',
  'ErrorResponse',
];

export function loadOpenApiContract(path = 'contracts/openapi.json') {
  return JSON.parse(fs.readFileSync(path, 'utf8'));
}

export function validateOpenApiContract(contract) {
  const errors = [];

  if (contract.openapi !== '3.1.0') {
    errors.push('OpenAPI version must be 3.1.0.');
  }

  if (contract.info?.title !== 'PerceptionLab API') {
    errors.push('OpenAPI info.title must be PerceptionLab API.');
  }

  for (const [method, path] of requiredOperations) {
    const operation = contract.paths?.[path]?.[method];
    if (!operation) {
      errors.push(`Missing ${method.toUpperCase()} ${path}.`);
      continue;
    }

    if (!operation.operationId) {
      errors.push(`${method.toUpperCase()} ${path} is missing operationId.`);
    }

    if (!Array.isArray(operation.tags) || operation.tags.length === 0) {
      errors.push(`${method.toUpperCase()} ${path} is missing tags.`);
    }
  }

  const schemas = contract.components?.schemas ?? {};
  for (const schemaName of requiredSchemas) {
    if (!schemas[schemaName]) {
      errors.push(`Missing schema ${schemaName}.`);
    }
  }

  const inferenceRequired = schemas.InferenceResponse?.required ?? [];
  for (const field of ['run_id', 'model_id', 'latency_ms', 'detections']) {
    if (!inferenceRequired.includes(field)) {
      errors.push(`InferenceResponse must require ${field}.`);
    }
  }

  const overlayRequired = schemas.OverlayResponse?.required ?? [];
  for (const field of ['inference_run_id', 'artifact_uri', 'labels']) {
    if (!overlayRequired.includes(field)) {
      errors.push(`OverlayResponse must require ${field}.`);
    }
  }

  return errors;
}
