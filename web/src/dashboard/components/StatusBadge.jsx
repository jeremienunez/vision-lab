const TONE = {
  success: 'bg-green-soft text-green',
  danger: 'bg-red-soft text-red',
  pending: 'bg-amber-soft text-amber',
  neutral: 'bg-surface-soft text-muted',
};

export function StatusBadge({ icon: Icon, label, tone = 'neutral' }) {
  return (
    <span
      className={`inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-medium ${
        TONE[tone] ?? TONE.neutral
      }`}
    >
      {Icon && <Icon size={16} aria-hidden="true" />}
      {label}
    </span>
  );
}
