const TONE_ACCENT = {
  blue: 'text-blue',
  amber: 'text-amber',
  green: 'text-green',
  cyan: 'text-cyan',
  neutral: 'text-ink',
};

export function KpiTile({ label, value, tone = 'neutral' }) {
  return (
    <article className="flex flex-col gap-1 rounded-2xl border border-line bg-surface p-4 shadow-panel">
      <span className="text-xs font-medium uppercase tracking-wide text-subtle">{label}</span>
      <strong className={`text-2xl font-semibold ${TONE_ACCENT[tone] ?? TONE_ACCENT.neutral}`}>
        {value}
      </strong>
    </article>
  );
}
