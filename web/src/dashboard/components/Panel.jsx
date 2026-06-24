export function Panel({ id, title, action, icon: Icon, children, wide = false }) {
  return (
    <section
      id={id}
      className={`flex min-w-0 flex-col rounded-2xl border border-line bg-surface p-5 shadow-panel ${
        wide ? 'lg:col-span-2' : ''
      }`}
    >
      <div className="mb-4 flex items-center justify-between gap-3">
        <div className="flex items-center gap-2 text-ink">
          {Icon && <Icon size={18} aria-hidden="true" className="text-cyan" />}
          <h2 className="text-base font-semibold">{title}</h2>
        </div>
        {action && <span className="text-xs font-medium text-muted">{action}</span>}
      </div>
      {children}
    </section>
  );
}
