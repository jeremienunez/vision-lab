import { ScanSearch } from 'lucide-react';

import { EmptyState } from '../../components/EmptyState.jsx';

export function DetectionList({ result }) {
  const detections = result?.detections ?? [];

  return (
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
  );
}
