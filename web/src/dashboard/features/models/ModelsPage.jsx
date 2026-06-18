import { BrainCircuit, Box } from 'lucide-react';

import { usePerceptionDataContext } from '../../context/PerceptionDataContext.jsx';
import { Panel } from '../../components/Panel.jsx';
import { DataTable } from '../../components/DataTable.jsx';
import { StateChip } from '../../components/StateChip.jsx';
import { EmptyState } from '../../components/EmptyState.jsx';

const COLUMNS = [
  {
    key: 'name',
    header: 'Model',
    render: (model) => (
      <span className="flex flex-col">
        <strong>{model.name}</strong>
        <small className="text-subtle">{model.version}</small>
      </span>
    ),
  },
  { key: 'model_family', header: 'Family' },
  { key: 'mAP50', header: 'mAP50', render: (m) => m.metrics_summary?.mAP50 ?? 'n/a' },
  { key: 'status', header: 'Status', render: (m) => <StateChip status={m.status} /> },
];

export function ModelsPage() {
  const { payload, viewModel } = usePerceptionDataContext();
  const models = payload.models ?? [];

  return (
    <Panel
      id="models"
      title="Model registry"
      action={`${viewModel.promotedModelCount} promoted`}
      icon={BrainCircuit}
      wide
    >
      {models.length === 0 ? (
        <EmptyState icon={Box} label="No models" />
      ) : (
        <DataTable columns={COLUMNS} rows={models} getRowKey={(m) => m.id} />
      )}
    </Panel>
  );
}
