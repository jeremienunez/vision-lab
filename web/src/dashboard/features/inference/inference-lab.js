const SUPPORTED_IMAGE_TYPES = new Set(['image/jpeg', 'image/png', 'image/webp']);

export function validateInferenceImageFile(file) {
  if (!file) return 'Choose a JPG, PNG, or WebP image.';
  if (!SUPPORTED_IMAGE_TYPES.has(file.type)) return 'Choose a JPG, PNG, or WebP image.';

  return '';
}

export function buildInferenceRequest({ modelId, imageFile, confidenceThreshold }) {
  const selectedModelId = String(modelId ?? '').trim();
  if (!selectedModelId) throw new Error('Select a model before running inference.');

  const imageError = validateInferenceImageFile(imageFile);
  if (imageError) throw new Error(imageError);

  return {
    modelId: selectedModelId,
    imageBlob: imageFile,
    filename: imageFile.name || 'inference-image.jpg',
    confidenceThreshold,
  };
}

export function buildInferenceResultView(result) {
  const detections = Array.isArray(result?.detections) ? result.detections : [];

  return {
    runId: result?.run_id ?? 'n/a',
    latencyLabel: formatLatency(result?.latency_ms),
    detectionCountLabel: `${detections.length} ${detections.length === 1 ? 'detection' : 'detections'}`,
    detections: detections.map((detection, index) => ({
      id: `${detection.class_name ?? 'detection'}-${index}`,
      className: detection.class_name ?? 'unknown',
      confidenceLabel: formatConfidence(detection.confidence),
      bboxLabel: formatBbox(detection.bbox),
    })),
  };
}

function formatLatency(latencyMs) {
  const numericLatency = Number(latencyMs);
  return Number.isFinite(numericLatency) ? `${Math.round(numericLatency)} ms` : 'n/a';
}

function formatConfidence(confidence) {
  const numericConfidence = Number(confidence);
  if (!Number.isFinite(numericConfidence)) return 'n/a';

  return `${Math.round(Math.min(1, Math.max(0, numericConfidence)) * 100)}%`;
}

function formatBbox(bbox) {
  if (!bbox) return 'bbox n/a';

  const values = ['x', 'y', 'width', 'height'].map((key) => Number(bbox[key]));
  if (!values.every(Number.isFinite)) return 'bbox n/a';

  const [x, y, width, height] = values.map((value) => value.toFixed(2));
  return `x ${x} / y ${y} / w ${width} / h ${height}`;
}
