import { useCallback, useEffect, useMemo, useState } from 'react';
import {
  Activity,
  AlertTriangle,
  Box,
  BrainCircuit,
  CheckCircle2,
  ChevronRight,
  Database,
  Gauge,
  GitBranch,
  KeyRound,
  RefreshCw,
  Server,
  Settings,
  ShieldCheck,
} from 'lucide-react';

import {
  buildDashboardViewModel,
  formatMetricValue,
  metricSeriesForChart,
} from './dashboard-data.js';
import { createPerceptionApi } from './perception-api.js';

const EMPTY_PAYLOAD = {
  health: null,
  datasets: [],
  trainingJobs: [],
  models: [],
  metricsByJob: {},
};

const DEFAULT_CONFIG = {
  baseUrl: import.meta.env.VITE_PERCEPTIONLAB_API_BASE_URL ?? '/api',
  apiKey: import.meta.env.VITE_PERCEPTIONLAB_API_KEY ?? '',
};

const NAV_ITEMS = [
  { label: 'Overview', icon: Activity, active: true },
  { label: 'Datasets', icon: Database },
  { label: 'Training', icon: GitBranch },
  { label: 'Models', icon: BrainCircuit },
  { label: 'Inference', icon: Gauge },
];

const STATUS_ORDER = ['queued', 'running', 'succeeded', 'failed', 'cancelled'];

export function Dashboard() {
  const [config, setConfig] = useState(loadConfig);
  const [draftConfig, setDraftConfig] = useState(config);
  const [payload, setPayload] = useState(EMPTY_PAYLOAD);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [lastUpdated, setLastUpdated] = useState(null);
  const [settingsOpen, setSettingsOpen] = useState(true);
  const [jobFilter, setJobFilter] = useState('all');

  const viewModel = useMemo(() => buildDashboardViewModel(payload), [payload]);
  const chartMetrics = useMemo(
    () => metricSeriesForChart(payload.metricsByJob),
    [payload.metricsByJob],
  );
  const visibleJobs = useMemo(() => {
    if (jobFilter === 'all') return payload.trainingJobs;
    return payload.trainingJobs.filter((job) => job.status === jobFilter);
  }, [jobFilter, payload.trainingJobs]);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError('');

    try {
      const api = createPerceptionApi(config);
      const nextPayload = await api.loadDashboard();
      setPayload(nextPayload);
      setLastUpdated(new Date());
    } catch (refreshError) {
      setPayload(EMPTY_PAYLOAD);
      setError(refreshError.message);
    } finally {
      setLoading(false);
    }
  }, [config]);

  useEffect(() => {
    refresh();
  }, [refresh]);

  function applyConfig(event) {
    event.preventDefault();
    persistConfig(draftConfig);
    setConfig(draftConfig);
  }

  return (
    <div className="app-shell">
      <aside className="sidebar" aria-label="PerceptionLab navigation">
        <div className="brand-mark">
          <span className="brand-glyph">PL</span>
          <span>
            <strong>PerceptionLab</strong>
            <small>Vision ops</small>
          </span>
        </div>

        <nav className="nav-list">
          {NAV_ITEMS.map((item) => (
            <a className={item.active ? 'nav-item active' : 'nav-item'} href="#overview" key={item.label}>
              <item.icon size={18} aria-hidden="true" />
              <span>{item.label}</span>
            </a>
          ))}
        </nav>

        <div className="sidebar-status">
          <ShieldCheck size={18} aria-hidden="true" />
          <span>{config.apiKey ? 'Protected' : 'Local open'}</span>
        </div>
      </aside>

      <main className="workspace" id="overview">
        <header className="topbar">
          <div>
            <h1>Operations</h1>
            <p>{viewModel.healthLabel}</p>
          </div>
          <div className="topbar-actions">
            <StatusBadge
              icon={Server}
              label={loading ? 'Refreshing' : viewModel.healthLabel}
              tone={error ? 'danger' : 'success'}
            />
            <button className="icon-button text-button" type="button" onClick={refresh} title="Refresh dashboard">
              <RefreshCw size={17} aria-hidden="true" />
              <span>Refresh</span>
            </button>
            <button
              className="icon-button"
              type="button"
              onClick={() => setSettingsOpen((open) => !open)}
              title="API settings"
              aria-label="API settings"
            >
              <Settings size={18} aria-hidden="true" />
            </button>
          </div>
        </header>

        {error && (
          <div className="error-strip" role="status">
            <AlertTriangle size={18} aria-hidden="true" />
            <span>{error}</span>
          </div>
        )}

        <section className="kpi-grid" aria-label="Platform summary">
          {viewModel.kpis.map((kpi) => (
            <KpiTile key={kpi.label} label={kpi.label} value={kpi.value} tone={kpi.tone} />
          ))}
        </section>

        <section className={settingsOpen ? 'content-layout with-settings' : 'content-layout'}>
          <div className="primary-grid">
            <Panel
              title="Datasets"
              action={`${payload.datasets.length} total`}
              icon={Database}
            >
              <DatasetTable datasets={payload.datasets} />
            </Panel>

            <Panel
              title="Training queue"
              action={`${viewModel.activeJobCount} active`}
              icon={GitBranch}
            >
              <JobFilters
                selected={jobFilter}
                counts={viewModel.jobStatusCounts}
                onSelect={setJobFilter}
              />
              <JobList jobs={visibleJobs} />
            </Panel>

            <Panel
              title="Model registry"
              action={`${viewModel.promotedModelCount} promoted`}
              icon={BrainCircuit}
              wide
            >
              <ModelTable models={payload.models} />
            </Panel>

            <Panel
              title="Latest metrics"
              action={lastUpdated ? lastUpdated.toLocaleTimeString() : 'Pending'}
              icon={Gauge}
            >
              <MetricChart metrics={chartMetrics} />
            </Panel>
          </div>

          {settingsOpen && (
            <aside className="settings-panel" aria-label="API configuration">
              <div className="panel-heading compact">
                <div className="panel-title">
                  <KeyRound size={18} aria-hidden="true" />
                  <h2>API configuration</h2>
                </div>
              </div>
              <form className="settings-form" onSubmit={applyConfig}>
                <label>
                  <span>Base URL</span>
                  <input
                    value={draftConfig.baseUrl}
                    onChange={(event) =>
                      setDraftConfig((current) => ({ ...current, baseUrl: event.target.value }))
                    }
                  />
                </label>
                <label>
                  <span>API key</span>
                  <input
                    type="password"
                    value={draftConfig.apiKey}
                    onChange={(event) =>
                      setDraftConfig((current) => ({ ...current, apiKey: event.target.value }))
                    }
                  />
                </label>
                <button className="primary-button" type="submit">
                  <CheckCircle2 size={17} aria-hidden="true" />
                  <span>Apply</span>
                </button>
              </form>
              <DependencyList dependencies={viewModel.dependencyHealth} />
            </aside>
          )}
        </section>
      </main>
    </div>
  );
}

function StatusBadge({ icon: Icon, label, tone }) {
  return (
    <span className={`status-badge ${tone}`}>
      <Icon size={16} aria-hidden="true" />
      {label}
    </span>
  );
}

function KpiTile({ label, value, tone }) {
  return (
    <article className={`kpi-tile ${tone}`}>
      <span>{label}</span>
      <strong>{value}</strong>
    </article>
  );
}

function Panel({ title, action, icon: Icon, children, wide = false }) {
  return (
    <section className={wide ? 'panel wide' : 'panel'}>
      <div className="panel-heading">
        <div className="panel-title">
          <Icon size={18} aria-hidden="true" />
          <h2>{title}</h2>
        </div>
        <span>{action}</span>
      </div>
      {children}
    </section>
  );
}

function DatasetTable({ datasets }) {
  if (datasets.length === 0) {
    return <EmptyState icon={Database} label="No datasets" />;
  }

  return (
    <div className="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Classes</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {datasets.map((dataset) => (
            <tr key={dataset.id}>
              <td>
                <strong>{dataset.name}</strong>
                <small>{dataset.id}</small>
              </td>
              <td>{dataset.classes?.join(', ') || 'n/a'}</td>
              <td>
                <span className="state-chip neutral">{dataset.status}</span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function JobFilters({ selected, counts, onSelect }) {
  const statuses = ['all', ...STATUS_ORDER.filter((status) => counts[status])];

  return (
    <div className="segmented-control" aria-label="Training job filter">
      {statuses.map((status) => (
        <button
          type="button"
          className={selected === status ? 'selected' : ''}
          onClick={() => onSelect(status)}
          key={status}
        >
          {status}
        </button>
      ))}
    </div>
  );
}

function JobList({ jobs }) {
  if (jobs.length === 0) {
    return <EmptyState icon={GitBranch} label="No training jobs" />;
  }

  return (
    <div className="job-list">
      {jobs.map((job) => (
        <article className="job-row" key={job.id}>
          <span className={`state-dot ${job.status}`} aria-hidden="true" />
          <div>
            <strong>{job.model_family}</strong>
            <small>{job.dataset_version_id}</small>
          </div>
          <span className={`state-chip ${job.status}`}>{job.status}</span>
        </article>
      ))}
    </div>
  );
}

function ModelTable({ models }) {
  if (models.length === 0) {
    return <EmptyState icon={Box} label="No models" />;
  }

  return (
    <div className="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Model</th>
            <th>Family</th>
            <th>mAP50</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {models.map((model) => (
            <tr key={model.id}>
              <td>
                <strong>{model.name}</strong>
                <small>{model.version}</small>
              </td>
              <td>{model.model_family}</td>
              <td>{model.metrics_summary?.mAP50 ?? 'n/a'}</td>
              <td>
                <span className={`state-chip ${model.status}`}>{model.status}</span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function MetricChart({ metrics }) {
  if (metrics.length === 0) {
    return <EmptyState icon={Gauge} label="No metrics" />;
  }

  const max = Math.max(...metrics.map((metric) => metric.metric_value), 1);

  return (
    <div className="metric-chart" aria-label="Latest training metrics">
      {metrics.map((metric, index) => (
        <div className="metric-bar" key={`${metric.metric_name}-${metric.epoch}-${index}`}>
          <span style={{ height: `${Math.max((metric.metric_value / max) * 100, 8)}%` }} />
          <small>{metric.metric_name}</small>
          <strong>{formatMetricValue(metric)}</strong>
        </div>
      ))}
    </div>
  );
}

function DependencyList({ dependencies }) {
  const entries = Object.entries(dependencies);

  return (
    <div className="dependency-list">
      {entries.length === 0 ? (
        <EmptyState icon={Server} label="No dependency status" compact />
      ) : (
        entries.map(([name, value]) => (
          <div className="dependency-row" key={name}>
            <span>{name}</span>
            <strong>{value}</strong>
          </div>
        ))
      )}
    </div>
  );
}

function EmptyState({ icon: Icon, label, compact = false }) {
  return (
    <div className={compact ? 'empty-state compact' : 'empty-state'}>
      <Icon size={18} aria-hidden="true" />
      <span>{label}</span>
    </div>
  );
}

function loadConfig() {
  if (typeof window === 'undefined') return DEFAULT_CONFIG;

  return {
    baseUrl: window.localStorage.getItem('perceptionlab.apiBaseUrl') ?? DEFAULT_CONFIG.baseUrl,
    apiKey: window.localStorage.getItem('perceptionlab.apiKey') ?? DEFAULT_CONFIG.apiKey,
  };
}

function persistConfig(nextConfig) {
  if (typeof window === 'undefined') return;

  window.localStorage.setItem('perceptionlab.apiBaseUrl', nextConfig.baseUrl);
  window.localStorage.setItem('perceptionlab.apiKey', nextConfig.apiKey);
}
