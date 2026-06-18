import { useMemo, useState } from 'react';
import { GitBranch } from 'lucide-react';

import { usePerceptionDataContext } from '../../context/PerceptionDataContext.jsx';
import { Panel } from '../../components/Panel.jsx';
import { SegmentedControl } from '../../components/SegmentedControl.jsx';
import { StateChip } from '../../components/StateChip.jsx';
import { EmptyState } from '../../components/EmptyState.jsx';

const STATUS_ORDER = ['queued', 'running', 'succeeded', 'failed', 'cancelled'];

const DOT_TONE = {
  queued: 'bg-blue',
  running: 'bg-amber',
  succeeded: 'bg-green',
  failed: 'bg-red',
  cancelled: 'bg-subtle',
};

export function TrainingPage() {
  const { payload, viewModel } = usePerceptionDataContext();
  const [filter, setFilter] = useState('all');
  const jobs = payload.trainingJobs ?? [];

  const options = useMemo(
    () => [
      { value: 'all', label: 'all' },
      ...STATUS_ORDER.filter((status) => viewModel.jobStatusCounts[status]).map((status) => ({
        value: status,
        label: status,
      })),
    ],
    [viewModel.jobStatusCounts],
  );

  const visibleJobs = filter === 'all' ? jobs : jobs.filter((job) => job.status === filter);

  return (
    <Panel id="training" title="Training queue" action={`${viewModel.activeJobCount} active`} icon={GitBranch} wide>
      <div className="mb-4">
        <SegmentedControl
          options={options}
          selected={filter}
          onSelect={setFilter}
          ariaLabel="Training job filter"
        />
      </div>

      {visibleJobs.length === 0 ? (
        <EmptyState icon={GitBranch} label="No training jobs" />
      ) : (
        <div className="flex flex-col gap-2">
          {visibleJobs.map((job) => (
            <article
              key={job.id}
              className="flex items-center gap-3 rounded-xl border border-line bg-surface-soft px-4 py-3"
            >
              <span
                className={`h-2.5 w-2.5 shrink-0 rounded-full ${DOT_TONE[job.status] ?? 'bg-subtle'}`}
                aria-hidden="true"
              />
              <div className="flex min-w-0 flex-1 flex-col">
                <strong className="truncate text-ink">{job.model_family}</strong>
                <small className="truncate text-subtle">{job.dataset_version_id}</small>
              </div>
              <StateChip status={job.status} />
            </article>
          ))}
        </div>
      )}
    </Panel>
  );
}
