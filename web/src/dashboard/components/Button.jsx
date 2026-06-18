const BASE =
  'inline-flex items-center justify-center gap-2 rounded-lg text-sm font-medium transition-colors disabled:opacity-60';

const VARIANT = {
  primary: 'bg-cyan px-4 py-2 text-white hover:opacity-90',
  secondary: 'border border-line bg-surface px-3 py-2 text-ink hover:bg-surface-soft',
};

export function Button({ variant = 'secondary', icon: Icon, iconClassName = '', children, className = '', ...props }) {
  return (
    <button className={`${BASE} ${VARIANT[variant]} ${className}`} {...props}>
      {Icon && <Icon size={16} aria-hidden="true" className={iconClassName} />}
      {children && <span>{children}</span>}
    </button>
  );
}
