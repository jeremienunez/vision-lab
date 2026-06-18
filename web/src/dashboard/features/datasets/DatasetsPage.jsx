import { Database } from 'lucide-react';

import { usePerceptionDataContext } from '../../context/PerceptionDataContext.jsx';
import { Panel } from '../../components/Panel.jsx';
import { DataTable } from '../../components/DataTable.jsx';
import { StateChip } from '../../components/StateChip.jsx';
import { EmptyState } from '../../components/EmptyState.jsx';

const COLUMNS = [
  {
    key: 'name',
    header: 'Name',
    render: (dataset) => (
      <span className="flex flex-col">
        <strong>{dataset.name}</strong>
        <small className="text-subtle">{dataset.id}</small>
      </span>
    ),
  },
  { key: 'classes', header: 'Classes', render: (d) => d.classes?.join(', ') || 'n/a' },
  { key: 'status', header: 'Status', render: (d) => <StateChip status="neutral">{d.status}</StateChip> },
];

export function DatasetsPage() {
  const { payload } = usePerceptionDataContext();
  const datasets = payload.datasets ?? [];

  return (
    <Panel id="datasets" title="Datasets" action={`${datasets.length} total`} icon={Database} wide>
      {datasets.length === 0 ? (
        <EmptyState icon={Database} label="No datasets" />
      ) : (
        <DataTable columns={COLUMNS} rows={datasets} getRowKey={(d) => d.id} />
      )}
    </Panel>
  );
}
