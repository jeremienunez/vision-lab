export const DEFAULT_CONFIDENCE_THRESHOLD = 0.25;

export function normalizeConfidenceThreshold(value) {
  const numericValue = Number(value);
  if (!Number.isFinite(numericValue)) return DEFAULT_CONFIDENCE_THRESHOLD;

  return Math.min(1, Math.max(0, numericValue));
}
