const FALLBACK_BOX = {
  left: '6%',
  top: '8%',
  width: '88%',
  height: '84%',
};

export function buildDetectionOverlayItems(detections, { mirrored = false } = {}) {
  return (detections ?? [])
    .filter((detection) => detection?.class_name)
    .slice(0, 5)
    .map((detection, index) => {
      const bbox = normalizedBbox(detection.bbox);
      const style = bbox
        ? bboxStyle(bbox, { mirrored })
        : fallbackStyle(index);

      return {
        label: detection.class_name,
        confidenceLabel: formatConfidence(detection.confidence),
        style,
      };
    });
}

function normalizedBbox(bbox) {
  if (!bbox) return null;

  const x = Number(bbox.x);
  const y = Number(bbox.y);
  const width = Number(bbox.width);
  const height = Number(bbox.height);

  if (![x, y, width, height].every(Number.isFinite)) return null;
  if (width <= 0 || height <= 0) return null;
  if (x < 0 || y < 0 || x + width > 1 || y + height > 1) return null;

  return { x, y, width, height };
}

function bboxStyle(bbox, { mirrored }) {
  const left = mirrored ? 1 - bbox.x - bbox.width : bbox.x;

  return {
    left: percent(left),
    top: percent(bbox.y),
    width: percent(bbox.width),
    height: percent(bbox.height),
  };
}

function fallbackStyle(index) {
  const verticalOffset = Math.min(index * 0.08, 0.24);

  return {
    ...FALLBACK_BOX,
    top: percent(0.08 + verticalOffset),
  };
}

function formatConfidence(confidence) {
  const numericConfidence = Number(confidence);
  if (!Number.isFinite(numericConfidence)) return '';

  return `${Math.round(Math.min(1, Math.max(0, numericConfidence)) * 100)}%`;
}

function percent(value) {
  return `${Math.round(value * 10_000) / 100}%`;
}
