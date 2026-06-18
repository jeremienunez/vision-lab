import { RefreshCw, Server, Settings } from 'lucide-react';

import { useConfigContext } from '../context/ConfigContext.jsx';
import { usePerceptionDataContext } from '../context/PerceptionDataContext.jsx';
import { StatusBadge } from '../components/StatusBadge.jsx';
import { Button } from '../components/Button.jsx';
import { IconButton } from '../components/IconButton.jsx';

export function Topbar({ settingsOpen, onToggleSettings }) {
  const { config } = useConfigContext();
  const { viewModel, loading, lastUpdated, refresh } = usePerceptionDataContext();

  return (
    <header className="flex flex-wrap items-end justify-between gap-4">
      <div className="flex flex-col gap-1">
        <span className="text-xs font-medium uppercase tracking-wide text-subtle">Computer vision operations</span>
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
        <Button
          onClick={refresh}
          disabled={loading}
          title="Refresh dashboard"
          icon={RefreshCw}
          iconClassName={loading ? 'animate-spin' : ''}
        >
          {loading ? 'Syncing' : 'Refresh'}
        </Button>
        <IconButton
          icon={Settings}
          onClick={onToggleSettings}
          aria-label="API settings"
          aria-expanded={settingsOpen}
        />
      </div>
    </header>
  );
}
