import { BADGE_TONE } from './tone.js';

export function StatusBadge({ icon: Icon, label, tone = 'neutral' }) {
  return (
    <span
      className={`inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-medium ${
        BADGE_TONE[tone] ?? BADGE_TONE.neutral
      }`}
    >
      {Icon && <Icon size={16} aria-hidden="true" />}
      {label}
    </span>
  );
}
