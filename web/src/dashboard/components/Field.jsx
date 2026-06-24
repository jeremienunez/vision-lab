export function Field({ label, children, className = '' }) {
  return (
    <label className={`flex min-w-0 flex-col gap-1 text-sm ${className}`}>
      <span className="text-xs font-medium text-muted">{label}</span>
      {children}
    </label>
  );
}
