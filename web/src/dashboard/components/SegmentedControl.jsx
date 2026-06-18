export function SegmentedControl({ options, selected, onSelect, ariaLabel }) {
  return (
    <div
      className="inline-flex flex-wrap gap-1 rounded-lg border border-line bg-surface-soft p-1"
      aria-label={ariaLabel}
    >
      {options.map((option) => (
        <button
          key={option.value}
          type="button"
          onClick={() => onSelect(option.value)}
          className={`rounded-md px-3 py-1 text-xs font-medium capitalize transition-colors ${
            selected === option.value ? 'bg-surface text-ink shadow-panel' : 'text-muted hover:text-ink'
          }`}
        >
          {option.label}
        </button>
      ))}
    </div>
  );
}
