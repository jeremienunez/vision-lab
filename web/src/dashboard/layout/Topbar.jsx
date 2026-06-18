import { RefreshCw, Server, Settings } from 'lucide-react';

import { useConfigContext } from '../context/ConfigContext.jsx';
import { usePerceptionDataContext } from '../context/PerceptionDataContext.jsx';
import { StatusBadge } from '../components/StatusBadge.jsx';

export function Topbar({ settingsOpen, onToggleSettings }) {
  const { config } = useConfigContext();
  const { viewModel, loading, lastUpdated, refresh } = usePerceptionDataContext();

  return (
    <header className="flex flex-wrap items-end justify-between gap-4">
      <div className="flex flex-col gap-1">
        <span className="text-xs font-medium uppercase tracking-wide text-subtle">
          Computer vision operations
        </span>
        <h1 className="text-2xl font-semibold text-ink">Operations</h1>
        <p className="text-sm text-muted">
          {config.baseUrl}
          {lastUpdated ? ` / synced ${lastUpdated.toLocaleTimeString()}` : ' / waiting for sync'}
        </p>
      </div>

      <div className="flex items-center gap-2">
        <StatusBadge
          icon={Server}
          label={loading ? 'Refreshing' : viewModel.healthLabel}
          tone={loading ? 'pending' : viewModel.connectionTone}
        />
        <button
          type="button"
          onClick={refresh}
          disabled={loading}
          title="Refresh dashboard"
          className="inline-flex items-center gap-2 rounded-lg border border-line bg-surface px-3 py-2 text-sm font-medium text-ink transition-colors hover:bg-surface-soft disabled:opacity-60"
        >
          <RefreshCw size={17} aria-hidden="true" className={loading ? 'animate-spin' : ''} />
          <span>{loading ? 'Syncing' : 'Refresh'}</span>
        </button>
        <button
          type="button"
          onClick={onToggleSettings}
          aria-label="API settings"
          aria-expanded={settingsOpen}
          className="grid h-10 w-10 place-items-center rounded-lg border border-line bg-surface text-ink transition-colors hover:bg-surface-soft"
        >
          <Settings size={18} aria-hidden="true" />
        </button>
      </div>
    </header>
  );
}
