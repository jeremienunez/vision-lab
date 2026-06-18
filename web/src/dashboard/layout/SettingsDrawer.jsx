import { useState } from 'react';
import { CheckCircle2, KeyRound, Server } from 'lucide-react';

import { useConfigContext } from '../context/ConfigContext.jsx';
import { usePerceptionDataContext } from '../context/PerceptionDataContext.jsx';
import { EmptyState } from '../components/EmptyState.jsx';

export function SettingsDrawer() {
  const { config, setConfig } = useConfigContext();
  const { viewModel } = usePerceptionDataContext();
  const [draft, setDraft] = useState(config);

  function applyConfig(event) {
    event.preventDefault();
    setConfig(draft);
  }

  const dependencies = Object.entries(viewModel.dependencyHealth ?? {});

  return (
    <aside
      className="flex w-full flex-col gap-5 rounded-2xl border border-line bg-surface p-5 shadow-panel lg:w-80"
      aria-label="API configuration"
    >
      <div className="flex items-center gap-2 text-ink">
        <KeyRound size={18} aria-hidden="true" className="text-cyan" />
        <h2 className="text-base font-semibold">API configuration</h2>
      </div>

      <form className="flex flex-col gap-3" onSubmit={applyConfig}>
        <label className="flex flex-col gap-1 text-sm">
          <span className="text-xs font-medium text-muted">Base URL</span>
          <input
            className="rounded-lg border border-line bg-surface-soft px-3 py-2 text-sm outline-none focus:border-cyan"
            value={draft.baseUrl}
            onChange={(event) => setDraft((current) => ({ ...current, baseUrl: event.target.value }))}
          />
        </label>
        <label className="flex flex-col gap-1 text-sm">
          <span className="text-xs font-medium text-muted">API key</span>
          <input
            type="password"
            className="rounded-lg border border-line bg-surface-soft px-3 py-2 text-sm outline-none focus:border-cyan"
            value={draft.apiKey}
            onChange={(event) => setDraft((current) => ({ ...current, apiKey: event.target.value }))}
          />
        </label>
        <button
          type="submit"
          className="inline-flex items-center justify-center gap-2 rounded-lg bg-cyan px-4 py-2 text-sm font-medium text-white transition-opacity hover:opacity-90"
        >
          <CheckCircle2 size={17} aria-hidden="true" />
          <span>Apply</span>
        </button>
      </form>

      <div className="flex flex-col gap-2">
        {dependencies.length === 0 ? (
          <EmptyState icon={Server} label="No dependency status" compact />
        ) : (
          dependencies.map(([name, value]) => (
            <div key={name} className="flex items-center justify-between text-sm">
              <span className="text-muted">{name}</span>
              <strong className="text-ink">{value}</strong>
            </div>
          ))
        )}
      </div>
    </aside>
  );
}
