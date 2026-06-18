export function IconButton({ icon: Icon, className = '', ...props }) {
  return (
    <button
      className={`grid h-10 w-10 place-items-center rounded-lg border border-line bg-surface text-ink transition-colors hover:bg-surface-soft ${className}`}
      {...props}
    >
      <Icon size={18} aria-hidden="true" />
    </button>
  );
}
