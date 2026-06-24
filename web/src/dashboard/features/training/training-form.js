import { orderCameraModels } from '../../camera-models.js';

export const DEFAULT_TRAINING_FORM = {
  datasetVersionId: '',
  modelFamily: 'yolo11s_finetune',
  baseModel: '',
  epochs: '2',
  batchSize: '4',
  imageSize: '640',
  learningRate: '0.001',
};

export function buildDatasetVersionOptions(datasets, versionsByDatasetId) {
  const datasetById = new Map((datasets ?? []).map((dataset) => [dataset.id, dataset]));

  return Object.entries(versionsByDatasetId ?? {}).flatMap(([datasetId, versions]) => {
    const dataset = datasetById.get(datasetId);

    return (versions ?? []).map((version) => {
      const classes = dataset?.classes ?? version.classes_snapshot ?? [];
      const detailParts = [
        `${version.sample_count ?? 0} samples`,
        `${version.annotation_count ?? 0} annotations`,
      ];

      if (classes.length > 0) {
        detailParts.push(classes.join(', '));
      }

      return {
        value: version.id,
        label: `${dataset?.name ?? datasetId} / ${version.version_name}`,
        detail: detailParts.join(' / '),
      };
    });
  });
}

export function buildTrainingJobCommand(formState) {
  return {
    datasetVersionId: String(formState.datasetVersionId ?? '').trim(),
    modelFamily: String(formState.modelFamily ?? '').trim(),
    baseModel: localPathFromArtifactUri(formState.baseModel),
    epochs: positiveInteger(formState.epochs),
    batchSize: positiveInteger(formState.batchSize),
    imageSize: positiveInteger(formState.imageSize),
    learningRate: positiveNumber(formState.learningRate),
  };
}

export function localPathFromArtifactUri(value) {
  const normalized = String(value ?? '').trim();
  if (!normalized.startsWith('file://')) return normalized;

  try {
    return decodeURIComponent(new URL(normalized).pathname);
  } catch {
    return normalized.replace(/^file:\/\//, '');
  }
}

export function preferredBaseModelPath(models) {
  const yoloModel = (models ?? []).find((model) => {
    const searchable = `${model.model_family ?? ''} ${model.name ?? ''} ${model.artifact_uri ?? ''}`.toLowerCase();
    return searchable.includes('yolo') && model.artifact_uri;
  });

  return localPathFromArtifactUri(yoloModel?.artifact_uri ?? '');
}

export function preferredDatasetVersionId(models, versionOptions) {
  const availableVersions = new Set((versionOptions ?? []).map((option) => option.value));
  const model = orderCameraModels(models).find((candidate) =>
    availableVersions.has(candidate.dataset_version_id),
  );

  return model?.dataset_version_id ?? versionOptions?.[0]?.value ?? '';
}

function positiveInteger(value) {
  const numeric = Number(value);
  return Number.isFinite(numeric) ? Math.max(1, Math.trunc(numeric)) : 1;
}

function positiveNumber(value) {
  const numeric = Number(value);
  return Number.isFinite(numeric) && numeric > 0 ? numeric : 0.001;
}
