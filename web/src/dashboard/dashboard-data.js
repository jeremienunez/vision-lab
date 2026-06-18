const ACTIVE_JOB_STATUSES = new Set(['queued', 'running']);
const METRIC_PRIORITY = new Map([
  ['mAP50', 4],
  ['map50', 4],
  ['accuracy', 3],
  ['precision', 2],
  ['recall', 2],
  ['loss', 1],
]);

export function buildApiHeaders(apiKey) {
  const normalizedApiKey = apiKey?.trim();
  return normalizedApiKey ? { 'x-api-key': normalizedApiKey } : {};
}

export function buildDashboardViewModel({
  health,
  datasets,
  trainingJobs,
  models,
  metricsByJob,
}) {
  const normalizedDatasets = Array.isArray(datasets) ? datasets : [];
  const normalizedJobs = Array.isArray(trainingJobs) ? trainingJobs : [];
  const normalizedModels = Array.isArray(models) ? models : [];
  const statusCounts = countBy(normalizedJobs, (job) => job.status ?? 'unknown');
  const activeJobCount = normalizedJobs.filter((job) => ACTIVE_JOB_STATUSES.has(job.status)).length;
  const promotedModelCount = normalizedModels.filter((model) => model.status === 'promoted').length;
  const latestMetric = selectLatestMetric(metricsByJob);
  const systemCards = buildSystemCards(health);
  const connectionTone = systemCards.every((card) => card.tone === 'success') ? 'success' : 'danger';

  return {
    healthLabel: connectionTone === 'success' ? 'API healthy' : 'API degraded',
    connectionTone,
    systemCards,
    dependencyHealth: health?.dependencies ?? {},
    jobStatusCounts: statusCounts,
    activeJobCount,
    promotedModelCount,
    latestMetric,
    kpis: [
      { label: 'Datasets', value: String(normalizedDatasets.length), tone: 'blue' },
      { label: 'Jobs active', value: String(activeJobCount), tone: 'amber' },
      { label: 'Models', value: String(normalizedModels.length), tone: 'green' },
      {
        label: 'Latest metric',
        value: latestMetric ? `${latestMetric.metric_name} ${formatMetricValue(latestMetric)}` : 'No metrics',
        tone: 'cyan',
      },
    ],
  };
}

function buildSystemCards(health) {
  return [
    { label: 'API', value: health?.status ?? 'unreachable', tone: readinessTone(health?.status) },
    ...Object.entries(health?.dependencies ?? {}).map(([label, value]) => ({
      label,
      value,
      tone: readinessTone(value),
    })),
  ];
}

function readinessTone(value) {
  return ['healthy', 'ready', 'ok', 'connected'].includes(String(value ?? '').toLowerCase())
    ? 'success'
    : 'danger';
}

export function formatMetricValue(metric) {
  if (!metric || typeof metric.metric_value !== 'number') {
    return 'n/a';
  }

  return String(Math.round(metric.metric_value * 1000) / 1000);
}

export function metricSeriesForChart(metricsByJob) {
  return Object.values(metricsByJob ?? {})
    .flat()
    .filter((metric) => typeof metric.metric_value === 'number')
    .sort(compareMetricAge)
    .slice(-8);
}

function countBy(items, selector) {
  return items.reduce((counts, item) => {
    const key = selector(item);
    return { ...counts, [key]: (counts[key] ?? 0) + 1 };
  }, {});
}

function selectLatestMetric(metricsByJob) {
  return Object.values(metricsByJob ?? {})
    .flat()
    .filter((metric) => typeof metric.metric_value === 'number')
    .sort(compareMetricRank)
    .at(0);
}

function compareMetricRank(left, right) {
  const age = compareMetricTime(right, left);
  if (age !== 0) return age;

  return metricPriority(right) - metricPriority(left);
}

function compareMetricTime(left, right) {
  return (
    (left.epoch ?? 0) - (right.epoch ?? 0)
    || (left.step ?? 0) - (right.step ?? 0)
  );
}

function compareMetricAge(left, right) {
  return (
    (left.epoch ?? 0) - (right.epoch ?? 0)
    || (left.step ?? 0) - (right.step ?? 0)
    || String(left.metric_name ?? '').localeCompare(String(right.metric_name ?? ''))
  );
}

function metricPriority(metric) {
  return METRIC_PRIORITY.get(metric.metric_name) ?? 0;
}
