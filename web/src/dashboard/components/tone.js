// Single source of truth mapping semantic tones / statuses to Tailwind classes.

export const BADGE_TONE = {
  success: 'bg-green-soft text-green',
  danger: 'bg-red-soft text-red',
  pending: 'bg-amber-soft text-amber',
  neutral: 'bg-surface-soft text-muted',
};

export const STATUS_TONE = {
  queued: 'bg-blue-soft text-blue',
  running: 'bg-amber-soft text-amber',
  succeeded: 'bg-green-soft text-green',
  validated: 'bg-green-soft text-green',
  promoted: 'bg-cyan-soft text-cyan',
  failed: 'bg-red-soft text-red',
  cancelled: 'bg-surface-soft text-muted',
  neutral: 'bg-surface-soft text-muted',
};

export const STATUS_DOT = {
  queued: 'bg-blue',
  running: 'bg-amber',
  succeeded: 'bg-green',
  failed: 'bg-red',
  cancelled: 'bg-subtle',
};

export const ACCENT_TEXT = {
  blue: 'text-blue',
  amber: 'text-amber',
  green: 'text-green',
  cyan: 'text-cyan',
  neutral: 'text-ink',
};
