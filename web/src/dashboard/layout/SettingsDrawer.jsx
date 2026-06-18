import { useState } from 'react';
import { CheckCircle2, KeyRound, Server } from 'lucide-react';

import { useConfigContext } from '../context/ConfigContext.jsx';
import { usePerceptionDataContext } from '../context/PerceptionDataContext.jsx';
import { EmptyState } from '../components/EmptyState.jsx';
import { Field } from '../components/Field.jsx';
import { TextInput } from '../components/TextInput.jsx';
import { Button } from '../components/Button.jsx';

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
        <Field label="Base URL">
          <TextInput
            value={draft.baseUrl}
            onChange={(event) => setDraft((current) => ({ ...current, baseUrl: event.target.value }))}
          />
        </Field>
        <Field label="API key">
          <TextInput
            type="password"
            value={draft.apiKey}
            onChange={(event) => setDraft((current) => ({ ...current, apiKey: event.target.value }))}
          />
        </Field>
        <Button type="submit" variant="primary" icon={CheckCircle2}>
          Apply
        </Button>
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
