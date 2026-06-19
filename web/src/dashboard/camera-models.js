export function orderCameraModels(models) {
  return (models ?? [])
    .filter((model) => model.status !== 'archived')
    .toSorted((left, right) => cameraModelRank(right) - cameraModelRank(left));
}

function cameraModelRank(model) {
  const searchable = `${model?.model_family ?? ''} ${model?.name ?? ''} ${model?.artifact_uri ?? ''}`.toLowerCase();

  const metricRank = numericRank(model?.metrics_summary?.mAP50) * 10;
  const statusRank = model?.status === 'promoted' ? 5 : model?.status === 'validated' ? 4 : 0;

  if (searchable.includes('yolo')) return 30 + metricRank + statusRank;
  if (searchable.includes('onnx')) return 20 + metricRank + statusRank;
  if (model?.status === 'validated') return 10 + metricRank;

  return 0;
}

function numericRank(value) {
  const numeric = Number(value);
  return Number.isFinite(numeric) ? numeric : 0;
}
