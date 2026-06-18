const STATUS_TONE = {
  queued: 'bg-blue-soft text-blue',
  running: 'bg-amber-soft text-amber',
  succeeded: 'bg-green-soft text-green',
  validated: 'bg-green-soft text-green',
  promoted: 'bg-cyan-soft text-cyan',
  failed: 'bg-red-soft text-red',
  cancelled: 'bg-surface-soft text-muted',
  neutral: 'bg-surface-soft text-muted',
};

export function StateChip({ status, children }) {
  const tone = STATUS_TONE[status] ?? STATUS_TONE.neutral;
  return (
    <span className={`inline-flex rounded-full px-2.5 py-0.5 text-xs font-medium ${tone}`}>
      {children ?? status}
    </span>
  );
}
