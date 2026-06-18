import { useMemo } from 'react';
import { Gauge } from 'lucide-react';

import { usePerceptionDataContext } from '../../context/PerceptionDataContext.jsx';
import { metricSeriesForChart, formatMetricValue } from '../../dashboard-data.js';
import { Panel } from '../../components/Panel.jsx';
import { EmptyState } from '../../components/EmptyState.jsx';

export function MetricsPage() {
  const { payload, lastUpdated } = usePerceptionDataContext();
  const metrics = useMemo(() => metricSeriesForChart(payload.metricsByJob), [payload.metricsByJob]);
  const max = Math.max(...metrics.map((metric) => metric.metric_value), 1);

  return (
    <Panel
      id="metrics"
      title="Latest metrics"
      action={lastUpdated ? lastUpdated.toLocaleTimeString() : 'Pending'}
      icon={Gauge}
      wide
    >
      {metrics.length === 0 ? (
        <EmptyState icon={Gauge} label="No metrics" />
      ) : (
        <div className="flex h-56 gap-3" aria-label="Latest training metrics">
          {metrics.map((metric, index) => (
            <div
              key={`${metric.metric_name}-${metric.epoch}-${index}`}
              className="flex flex-1 flex-col items-center gap-2"
            >
              <div className="flex w-full flex-1 items-end">
                <span
                  className="w-full rounded-t-md bg-cyan/70"
                  style={{ height: `${Math.max((metric.metric_value / max) * 100, 8)}%` }}
                />
              </div>
              <small className="text-xs text-subtle">{metric.metric_name}</small>
              <strong className="text-xs text-ink">{formatMetricValue(metric)}</strong>
            </div>
          ))}
        </div>
      )}
    </Panel>
  );
}
