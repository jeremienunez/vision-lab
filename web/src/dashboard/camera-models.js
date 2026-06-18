export function orderCameraModels(models) {
  return (models ?? [])
    .filter((model) => model.status !== 'archived')
    .toSorted((left, right) => cameraModelRank(right) - cameraModelRank(left));
}

function cameraModelRank(model) {
  const searchable = `${model?.model_family ?? ''} ${model?.name ?? ''} ${model?.artifact_uri ?? ''}`.toLowerCase();

  if (searchable.includes('yolo')) return 30;
  if (searchable.includes('onnx')) return 20;
  if (model?.status === 'validated') return 10;

  return 0;
}
