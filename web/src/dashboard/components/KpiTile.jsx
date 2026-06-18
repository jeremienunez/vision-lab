import { ACCENT_TEXT } from './tone.js';

export function KpiTile({ label, value, tone = 'neutral' }) {
  return (
    <article className="flex flex-col gap-1 rounded-2xl border border-line bg-surface p-4 shadow-panel">
      <span className="text-xs font-medium uppercase tracking-wide text-subtle">{label}</span>
      <strong className={`text-2xl font-semibold ${ACCENT_TEXT[tone] ?? ACCENT_TEXT.neutral}`}>{value}</strong>
    </article>
  );
}
