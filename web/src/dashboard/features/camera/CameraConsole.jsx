import { AlertTriangle, Play, ScanSearch, Square } from 'lucide-react';

import { EmptyState } from '../../components/EmptyState.jsx';

export function CameraConsole({
  models,
  selectedModelId,
  confidenceThreshold,
  running,
  busy,
  error,
  result,
  onSelectModel,
  onConfidenceChange,
  onStart,
  onStop,
  onCapture,
}) {
  const detections = result?.detections ?? [];

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-wrap items-end gap-4">
        <label className="flex flex-1 flex-col gap-1 text-sm">
          <span className="text-xs font-medium text-muted">Model</span>
          <select
            value={selectedModelId}
            onChange={(event) => onSelectModel(event.target.value)}
            disabled={running}
            className="rounded-lg border border-line bg-surface-soft px-3 py-2 text-sm outline-none focus:border-cyan disabled:opacity-60"
          >
            {models.length === 0 ? (
              <option value="">No model available</option>
            ) : (
              models.map((model) => (
                <option value={model.id} key={model.id}>
                  {model.name} / {model.model_family} / {model.status}
                </option>
              ))
            )}
          </select>
        </label>

        <label className="flex w-28 flex-col gap-1 text-sm">
          <span className="text-xs font-medium text-muted">Confidence</span>
          <input
            type="number"
            min="0"
            max="1"
            step="0.05"
            value={confidenceThreshold}
            onChange={(event) => onConfidenceChange(Number(event.target.value))}
            className="rounded-lg border border-line bg-surface-soft px-3 py-2 text-sm outline-none focus:border-cyan"
          />
        </label>

        <div className="flex items-center gap-2">
          {running ? (
            <button
              type="button"
              onClick={onStop}
              className="inline-flex items-center gap-2 rounded-lg border border-line bg-surface px-3 py-2 text-sm font-medium text-ink hover:bg-surface-soft"
            >
              <Square size={16} aria-hidden="true" />
              <span>Stop</span>
            </button>
          ) : (
            <button
              type="button"
              onClick={onStart}
              disabled={models.length === 0}
              className="inline-flex items-center gap-2 rounded-lg bg-cyan px-4 py-2 text-sm font-medium text-white hover:opacity-90 disabled:opacity-60"
            >
              <Play size={16} aria-hidden="true" />
              <span>Start camera</span>
            </button>
          )}
          <button
            type="button"
            onClick={onCapture}
            disabled={!running || busy}
            className="inline-flex items-center gap-2 rounded-lg border border-line bg-surface px-3 py-2 text-sm font-medium text-ink hover:bg-surface-soft disabled:opacity-60"
          >
            <ScanSearch size={16} aria-hidden="true" />
            <span>Analyze now</span>
          </button>
        </div>
      </div>

      {error && (
        <div className="flex items-center gap-2 rounded-xl border border-red/30 bg-red-soft px-4 py-3 text-sm text-red" role="status">
          <AlertTriangle size={17} aria-hidden="true" />
          <span>{error}</span>
        </div>
      )}

      <div className="flex flex-col gap-2" aria-live="polite">
        <div className="flex items-center justify-between text-sm">
          <strong className="text-ink">Last identification</strong>
          <span className="text-subtle">{result?.capturedAt ? result.capturedAt.toLocaleTimeString() : 'Pending'}</span>
        </div>
        {detections.length === 0 ? (
          <EmptyState icon={ScanSearch} label="No analyzed frame yet" compact />
        ) : (
          <div className="flex flex-col gap-1">
            {detections.map((detection, index) => (
              <article
                key={`${detection.class_name}-${index}`}
                className="flex items-center justify-between rounded-lg border border-line bg-surface-soft px-3 py-2 text-sm"
              >
                <span className="text-ink">{detection.class_name}</span>
                <strong className="text-cyan">{Math.round(detection.confidence * 100)}%</strong>
              </article>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
