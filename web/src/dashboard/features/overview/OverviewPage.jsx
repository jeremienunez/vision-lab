import { usePerceptionDataContext } from '../../context/PerceptionDataContext.jsx';
import { KpiTile } from '../../components/KpiTile.jsx';

function formatCardLabel(label) {
  return String(label ?? '').replaceAll('_', ' ');
}

export function OverviewPage() {
  const { viewModel, loading, lastUpdated } = usePerceptionDataContext();

  const statusCards = [
    ...viewModel.systemCards,
    {
      label: 'sync',
      value: loading ? 'refreshing' : lastUpdated ? lastUpdated.toLocaleTimeString() : 'pending',
      tone: loading ? 'pending' : 'neutral',
    },
  ];

  return (
    <div className="flex flex-col gap-6">
      <section className="flex flex-wrap gap-3" aria-label="System status">
        {statusCards.map((card) => (
          <article
            key={`${card.label}-${card.value}`}
            className="flex min-w-32 flex-col gap-1 rounded-xl border border-line bg-surface px-4 py-3 shadow-panel"
          >
            <span className="text-xs uppercase tracking-wide text-subtle">
              {formatCardLabel(card.label)}
            </span>
            <strong
              className={`text-sm font-semibold ${
                card.tone === 'success' ? 'text-green' : card.tone === 'danger' ? 'text-red' : 'text-ink'
              }`}
            >
              {card.value}
            </strong>
          </article>
        ))}
      </section>

      <section className="grid grid-cols-2 gap-4 lg:grid-cols-4" aria-label="Platform summary">
        {viewModel.kpis.map((kpi) => (
          <KpiTile key={kpi.label} label={kpi.label} value={kpi.value} tone={kpi.tone} />
        ))}
      </section>
    </div>
  );
}
