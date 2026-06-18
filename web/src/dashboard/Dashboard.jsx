import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import {
  Activity,
  AlertTriangle,
  Box,
  BrainCircuit,
  Camera,
  CheckCircle2,
  Database,
  Gauge,
  GitBranch,
  KeyRound,
  Play,
  RefreshCw,
  ScanSearch,
  Square,
  Server,
  Settings,
  ShieldCheck,
} from 'lucide-react';

import {
  buildDashboardViewModel,
  formatMetricValue,
  metricSeriesForChart,
} from './dashboard-data.js';
import { buildDetectionOverlayItems } from './camera-overlay.js';
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
  { label: 'Overview', icon: Activity, href: '#overview', active: true },
  { label: 'Datasets', icon: Database, href: '#datasets' },
  { label: 'Training', icon: GitBranch, href: '#training' },
  { label: 'Models', icon: BrainCircuit, href: '#models' },
  { label: 'Camera', icon: Camera, href: '#camera' },
  { label: 'Metrics', icon: Gauge, href: '#metrics' },
];

const STATUS_ORDER = ['queued', 'running', 'succeeded', 'failed', 'cancelled'];
const CAMERA_INTERVAL_MS = 10_000;
const DEFAULT_CONFIDENCE_THRESHOLD = 0.25;

export function Dashboard() {
  const [config, setConfig] = useState(loadConfig);
  const [draftConfig, setDraftConfig] = useState(config);
  const [payload, setPayload] = useState(EMPTY_PAYLOAD);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [lastUpdated, setLastUpdated] = useState(null);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [jobFilter, setJobFilter] = useState('all');
  const [cameraModelId, setCameraModelId] = useState('');
  const [cameraRunning, setCameraRunning] = useState(false);
  const [cameraError, setCameraError] = useState('');
  const [cameraStatus, setCameraStatus] = useState('Camera idle');
  const [cameraInference, setCameraInference] = useState(null);
  const [cameraBusy, setCameraBusy] = useState(false);
  const [confidenceThreshold, setConfidenceThreshold] = useState(DEFAULT_CONFIDENCE_THRESHOLD);
  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const streamRef = useRef(null);
  const captureInFlightRef = useRef(false);

  const viewModel = useMemo(() => buildDashboardViewModel(payload), [payload]);
  const chartMetrics = useMemo(
    () => metricSeriesForChart(payload.metricsByJob),
    [payload.metricsByJob],
  );
  const inferenceModels = useMemo(
    () => payload.models.filter((model) => model.status !== 'archived'),
    [payload.models],
  );
  const selectedCameraModel = useMemo(
    () => inferenceModels.find((model) => model.id === cameraModelId) ?? inferenceModels[0] ?? null,
    [cameraModelId, inferenceModels],
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

  useEffect(() => {
    if (!cameraModelId && inferenceModels[0]) {
      setCameraModelId(inferenceModels[0].id);
    }
  }, [cameraModelId, inferenceModels]);

  const stopCamera = useCallback((nextStatus = 'Camera stopped') => {
    streamRef.current?.getTracks().forEach((track) => track.stop());
    streamRef.current = null;
    if (videoRef.current) {
      videoRef.current.srcObject = null;
    }
    setCameraRunning(false);
    setCameraBusy(false);
    setCameraStatus(nextStatus);
  }, []);

  const captureAndInfer = useCallback(async () => {
    if (captureInFlightRef.current || !videoRef.current || !selectedCameraModel) return;
    if (!videoRef.current.videoWidth || !videoRef.current.videoHeight) return;

    captureInFlightRef.current = true;
    setCameraBusy(true);
    setCameraError('');
    setCameraStatus('Capturing frame');

    try {
      const canvas = canvasRef.current;
      canvas.width = videoRef.current.videoWidth;
      canvas.height = videoRef.current.videoHeight;
      canvas.getContext('2d').drawImage(videoRef.current, 0, 0, canvas.width, canvas.height);
      const imageBlob = await canvasToBlob(canvas, 'image/jpeg', 0.86);
      const api = createPerceptionApi(config);
      const result = await api.runModelInference({
        modelId: selectedCameraModel.id,
        imageBlob,
        filename: `webcam-frame-${Date.now()}.jpg`,
        confidenceThreshold,
      });

      setCameraInference({
        ...result,
        capturedAt: new Date(),
        modelName: selectedCameraModel.name,
      });
      setCameraStatus(`Last analyzed ${new Date().toLocaleTimeString()}`);
    } catch (captureError) {
      setCameraError(captureError.message);
      setCameraStatus('Inference failed');
    } finally {
      captureInFlightRef.current = false;
      setCameraBusy(false);
    }
  }, [config, confidenceThreshold, selectedCameraModel]);

  const startCamera = useCallback(async () => {
    setCameraError('');

    try {
      if (!selectedCameraModel) {
        throw new Error('Select a model before starting camera inference.');
      }
      if (!navigator.mediaDevices?.getUserMedia) {
        throw new Error('Camera access is not available in this browser.');
      }

      const stream = await navigator.mediaDevices.getUserMedia({
        video: {
          facingMode: 'user',
          width: { ideal: 1280 },
          height: { ideal: 720 },
        },
        audio: false,
      });
      streamRef.current = stream;
      if (!videoRef.current) {
        throw new Error('Camera preview is not ready yet.');
      }

      videoRef.current.srcObject = stream;
      await videoRef.current.play();
      setCameraRunning(true);
      setCameraStatus('Camera live');
      setTimeout(() => {
        captureAndInfer();
      }, 250);
    } catch (cameraStartError) {
      setCameraError(cameraStartError.message);
      stopCamera('Camera blocked');
    }
  }, [captureAndInfer, selectedCameraModel, stopCamera]);

  useEffect(() => {
    if (!cameraRunning) return undefined;

    const intervalId = window.setInterval(() => {
      captureAndInfer();
    }, CAMERA_INTERVAL_MS);

    return () => window.clearInterval(intervalId);
  }, [cameraRunning, captureAndInfer]);

  useEffect(() => () => {
    streamRef.current?.getTracks().forEach((track) => track.stop());
    streamRef.current = null;
  }, []);

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
            <a className={item.active ? 'nav-item active' : 'nav-item'} href={item.href} key={item.label}>
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

      <main className="workspace" id="overview" aria-busy={loading}>
        <header className="topbar">
          <div>
            <span className="eyebrow">Computer vision operations</span>
            <h1>Operations</h1>
            <p>
              {config.baseUrl}
              {lastUpdated ? ` / synced ${lastUpdated.toLocaleTimeString()}` : ' / waiting for sync'}
            </p>
          </div>
          <div className="topbar-actions">
            <StatusBadge
              icon={Server}
              label={loading ? 'Refreshing' : viewModel.healthLabel}
              tone={loading ? 'pending' : viewModel.connectionTone}
            />
            <button
              className="icon-button text-button"
              type="button"
              onClick={refresh}
              title="Refresh dashboard"
              disabled={loading}
            >
              <RefreshCw className={loading ? 'spin' : ''} size={17} aria-hidden="true" />
              <span>{loading ? 'Syncing' : 'Refresh'}</span>
            </button>
            <button
              className="icon-button"
              type="button"
              onClick={() => setSettingsOpen((open) => !open)}
              title="API settings"
              aria-label="API settings"
              aria-expanded={settingsOpen}
            >
              <Settings size={18} aria-hidden="true" />
            </button>
          </div>
        </header>

        <SystemStrip
          cards={viewModel.systemCards}
          apiKeyConfigured={Boolean(config.apiKey)}
          lastUpdated={lastUpdated}
          loading={loading}
        />

        {error && (
          <div className="error-strip" role="status">
            <AlertTriangle size={18} aria-hidden="true" />
            <span>
              <strong>API response</strong>
              {error}
            </span>
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
              id="datasets"
              title="Datasets"
              action={`${payload.datasets.length} total`}
              icon={Database}
            >
              <DatasetTable datasets={payload.datasets} />
            </Panel>

            <Panel
              id="training"
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
              id="models"
              title="Model registry"
              action={`${viewModel.promotedModelCount} promoted`}
              icon={BrainCircuit}
              wide
            >
              <ModelTable models={payload.models} />
            </Panel>

            <Panel
              id="camera"
              title="Camera inference"
              action={cameraRunning ? '10 sec cadence' : 'manual start'}
              icon={Camera}
              wide
            >
              <CameraInferencePanel
                videoRef={videoRef}
                canvasRef={canvasRef}
                models={inferenceModels}
                selectedModelId={selectedCameraModel?.id ?? ''}
                confidenceThreshold={confidenceThreshold}
                running={cameraRunning}
                busy={cameraBusy}
                status={cameraStatus}
                error={cameraError}
                result={cameraInference}
                onSelectModel={setCameraModelId}
                onConfidenceChange={(value) => setConfidenceThreshold(normalizeConfidenceThreshold(value))}
                onStart={startCamera}
                onStop={() => stopCamera()}
                onCapture={captureAndInfer}
              />
            </Panel>

            <Panel
              id="metrics"
              title="Latest metrics"
              action={lastUpdated ? lastUpdated.toLocaleTimeString() : 'Pending'}
              icon={Gauge}
              wide
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

function SystemStrip({ cards, apiKeyConfigured, lastUpdated, loading }) {
  const syncLabel = loading
    ? 'refreshing'
    : lastUpdated
      ? lastUpdated.toLocaleTimeString()
      : 'pending';
  const statusCards = [
    ...cards,
    {
      label: 'auth',
      value: apiKeyConfigured ? 'protected' : 'local open',
      tone: apiKeyConfigured ? 'success' : 'neutral',
    },
    {
      label: 'sync',
      value: syncLabel,
      tone: loading ? 'pending' : 'neutral',
    },
  ];

  return (
    <section className="system-strip" aria-label="System status">
      {statusCards.map((card) => (
        <article className={`system-card ${card.tone}`} key={`${card.label}-${card.value}`}>
          <span>{formatCardLabel(card.label)}</span>
          <strong>{card.value}</strong>
        </article>
      ))}
    </section>
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

function Panel({ id, title, action, icon: Icon, children, wide = false }) {
  return (
    <section className={wide ? 'panel wide' : 'panel'} id={id}>
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

function CameraInferencePanel({
  videoRef,
  canvasRef,
  models,
  selectedModelId,
  confidenceThreshold,
  running,
  busy,
  status,
  error,
  result,
  onSelectModel,
  onConfidenceChange,
  onStart,
  onStop,
  onCapture,
}) {
  const detections = result?.detections ?? [];
  const overlayItems = buildDetectionOverlayItems(detections, { mirrored: true });

  return (
    <div className="camera-panel">
      <div className="camera-stage">
        <video
          ref={videoRef}
          className="camera-video"
          aria-label="Live webcam preview"
          autoPlay
          muted
          playsInline
        />
        <canvas ref={canvasRef} hidden />
        {overlayItems.length > 0 && (
          <div className="camera-detection-layer" aria-label="Detected objects on camera preview">
            {overlayItems.map((item, index) => (
              <div
                className="camera-detection-box"
                key={`${item.label}-${index}`}
                style={item.style}
              >
                <span className="camera-detection-tag">
                  <strong>{item.label}</strong>
                  {item.confidenceLabel && <em>{item.confidenceLabel}</em>}
                </span>
              </div>
            ))}
          </div>
        )}
        <div className="camera-overlay">
          <StatusBadge icon={ScanSearch} label={busy ? 'Analyzing frame' : status} tone={busy ? 'pending' : 'success'} />
        </div>
      </div>

      <div className="camera-console">
        <div className="camera-controls">
          <label>
            <span>Model</span>
            <select
              value={selectedModelId}
              onChange={(event) => onSelectModel(event.target.value)}
              disabled={running}
            >
              {models.length === 0 ? (
                <option value="">No model available</option>
              ) : (
                models.map((model) => (
                  <option value={model.id} key={model.id}>
                    {model.name} / {model.status}
                  </option>
                ))
              )}
            </select>
          </label>

          <label>
            <span>Confidence</span>
            <input
              type="number"
              min="0"
              max="1"
              step="0.05"
              value={confidenceThreshold}
              onChange={(event) => onConfidenceChange(Number(event.target.value))}
            />
          </label>

          <div className="camera-actions">
            {running ? (
              <button className="icon-button text-button" type="button" onClick={onStop}>
                <Square size={16} aria-hidden="true" />
                <span>Stop</span>
              </button>
            ) : (
              <button
                className="primary-button"
                type="button"
                onClick={onStart}
                disabled={models.length === 0}
              >
                <Play size={16} aria-hidden="true" />
                <span>Start camera</span>
              </button>
            )}
            <button
              className="icon-button text-button"
              type="button"
              onClick={onCapture}
              disabled={!running || busy}
            >
              <ScanSearch size={16} aria-hidden="true" />
              <span>Analyze now</span>
            </button>
          </div>
        </div>

        {error && (
          <div className="camera-error" role="status">
            <AlertTriangle size={17} aria-hidden="true" />
            <span>{error}</span>
          </div>
        )}

        <div className="camera-result" aria-live="polite">
          <div className="camera-result-header">
            <strong>Last identification</strong>
            <span>{result?.capturedAt ? result.capturedAt.toLocaleTimeString() : 'Pending'}</span>
          </div>
          {detections.length === 0 ? (
            <EmptyState icon={ScanSearch} label="No analyzed frame yet" compact />
          ) : (
            <div className="detection-list">
              {detections.map((detection, index) => (
                <article className="detection-row" key={`${detection.class_name}-${index}`}>
                  <span>{detection.class_name}</span>
                  <strong>{Math.round(detection.confidence * 100)}%</strong>
                </article>
              ))}
            </div>
          )}
        </div>
      </div>
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

function formatCardLabel(label) {
  return String(label ?? '').replaceAll('_', ' ');
}

function normalizeConfidenceThreshold(value) {
  const numericValue = Number(value);
  if (!Number.isFinite(numericValue)) return DEFAULT_CONFIDENCE_THRESHOLD;

  return Math.min(1, Math.max(0, numericValue));
}

function canvasToBlob(canvas, type, quality) {
  return new Promise((resolve, reject) => {
    canvas.toBlob((blob) => {
      if (blob) {
        resolve(blob);
      } else {
        reject(new Error('Unable to capture webcam frame.'));
      }
    }, type, quality);
  });
}
