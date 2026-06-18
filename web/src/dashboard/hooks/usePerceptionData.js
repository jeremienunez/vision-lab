import { useCallback, useEffect, useMemo, useState } from 'react';

import { createPerceptionApi } from '../perception-api.js';
import { buildDashboardViewModel } from '../dashboard-data.js';

const EMPTY_PAYLOAD = {
  health: null,
  datasets: [],
  trainingJobs: [],
  models: [],
  metricsByJob: {},
};

export function usePerceptionData(config) {
  const [payload, setPayload] = useState(EMPTY_PAYLOAD);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [lastUpdated, setLastUpdated] = useState(null);

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

  const viewModel = useMemo(() => buildDashboardViewModel(payload), [payload]);

  return { payload, viewModel, loading, error, lastUpdated, refresh };
}
