import { useState } from 'react';
import { Outlet } from 'react-router-dom';

import { Sidebar } from './Sidebar.jsx';
import { Topbar } from './Topbar.jsx';
import { SettingsDrawer } from './SettingsDrawer.jsx';
import { ErrorBanner } from '../components/ErrorBanner.jsx';
import { usePerceptionDataContext } from '../context/PerceptionDataContext.jsx';

export function AppLayout() {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const { error, loading } = usePerceptionDataContext();

  return (
    <div className="grid min-h-screen grid-cols-1 lg:grid-cols-[236px_minmax(0,1fr)]">
      <Sidebar />
      <main className="flex min-w-0 flex-col gap-6 p-4 sm:p-6 lg:p-8" aria-busy={loading}>
        <Topbar settingsOpen={settingsOpen} onToggleSettings={() => setSettingsOpen((open) => !open)} />

        {error && (
          <ErrorBanner>
            <strong className="mr-2">API response</strong>
            {error}
          </ErrorBanner>
        )}

        <div className={settingsOpen ? 'flex flex-col gap-6 lg:flex-row' : ''}>
          <div className="min-w-0 flex-1">
            <Outlet />
          </div>
          {settingsOpen && <SettingsDrawer />}
        </div>
      </main>
    </div>
  );
}
