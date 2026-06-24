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
      className="sticky top-0 z-20 flex w-full min-w-0 flex-col gap-3 overflow-hidden bg-nav px-3 py-3 text-white lg:h-screen lg:gap-6 lg:px-4 lg:py-6"
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

      <nav className="flex min-w-0 gap-1 overflow-x-auto lg:flex-1 lg:flex-col lg:overflow-visible">
        {NAV_ITEMS.map((item) => (
          <NavLink
            key={item.label}
            to={item.to}
            end={item.to === '/'}
            className={({ isActive }) =>
              `flex shrink-0 items-center gap-2 rounded-xl px-3 py-2 text-sm transition-colors lg:gap-3 ${
                isActive ? 'bg-white/10 text-white' : 'text-nav-muted hover:bg-white/5 hover:text-white'
              }`
            }
          >
            <item.icon size={18} aria-hidden="true" />
            <span>{item.label}</span>
          </NavLink>
        ))}
      </nav>

      <div className="hidden items-center gap-2 px-2 text-xs text-nav-muted lg:flex">
        <ShieldCheck size={18} aria-hidden="true" />
        <span>{config.apiKey ? 'Protected' : 'Local open'}</span>
      </div>
    </aside>
  );
}
