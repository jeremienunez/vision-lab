import { NavLink } from 'react-router-dom';
import { Activity, BrainCircuit, Camera, Database, Gauge, GitBranch, ShieldCheck } from 'lucide-react';

import { useConfigContext } from '../context/ConfigContext.jsx';

const NAV_ITEMS = [
  { label: 'Overview', icon: Activity, to: '/' },
  { label: 'Datasets', icon: Database, to: '/datasets' },
  { label: 'Training', icon: GitBranch, to: '/training' },
  { label: 'Models', icon: BrainCircuit, to: '/models' },
  { label: 'Camera', icon: Camera, to: '/camera' },
  { label: 'Metrics', icon: Gauge, to: '/metrics' },
];

export function Sidebar() {
  const { config } = useConfigContext();

  return (
    <aside
      className="sticky top-0 flex h-screen flex-col gap-6 bg-nav px-4 py-6 text-white"
      aria-label="PerceptionLab navigation"
    >
      <div className="flex items-center gap-3 px-2">
        <span className="grid h-10 w-10 place-items-center rounded-xl bg-cyan/20 font-bold text-cyan">
          PL
        </span>
        <span className="flex flex-col leading-tight">
          <strong className="text-sm">PerceptionLab</strong>
          <small className="text-xs text-nav-muted">Vision ops</small>
        </span>
      </div>

      <nav className="flex flex-1 flex-col gap-1">
        {NAV_ITEMS.map((item) => (
          <NavLink
            key={item.label}
            to={item.to}
            end={item.to === '/'}
            className={({ isActive }) =>
              `flex items-center gap-3 rounded-xl px-3 py-2 text-sm transition-colors ${
                isActive ? 'bg-white/10 text-white' : 'text-nav-muted hover:bg-white/5 hover:text-white'
              }`
            }
          >
            <item.icon size={18} aria-hidden="true" />
            <span>{item.label}</span>
          </NavLink>
        ))}
      </nav>

      <div className="flex items-center gap-2 px-2 text-xs text-nav-muted">
        <ShieldCheck size={18} aria-hidden="true" />
        <span>{config.apiKey ? 'Protected' : 'Local open'}</span>
      </div>
    </aside>
  );
}
