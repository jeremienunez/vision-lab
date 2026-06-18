export function EmptyState({ icon: Icon, label, compact = false }) {
  return (
    <div
      className={`flex items-center justify-center gap-2 rounded-xl border border-dashed border-line text-sm text-subtle ${
        compact ? 'p-4' : 'p-8'
      }`}
    >
      {Icon && <Icon size={18} aria-hidden="true" />}
      <span>{label}</span>
    </div>
  );
}
