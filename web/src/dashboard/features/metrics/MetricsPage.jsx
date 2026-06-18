import { Gauge } from 'lucide-react';
import { EmptyState } from '../../components/EmptyState.jsx';

export function MetricsPage() {
  return <EmptyState icon={Gauge} label="Metrics view — migrating" />;
}
