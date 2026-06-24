import { OverviewPage } from '../features/overview/OverviewPage.jsx';
import { DatasetsPage } from '../features/datasets/DatasetsPage.jsx';
import { TrainingPage } from '../features/training/TrainingPage.jsx';
import { ModelsPage } from '../features/models/ModelsPage.jsx';
import { InferencePage } from '../features/inference/InferencePage.jsx';
import { CameraPage } from '../features/camera/CameraPage.jsx';
import { MetricsPage } from '../features/metrics/MetricsPage.jsx';

export const ROUTES = [
  { path: '/', element: <OverviewPage /> },
  { path: '/datasets', element: <DatasetsPage /> },
  { path: '/training', element: <TrainingPage /> },
  { path: '/models', element: <ModelsPage /> },
  { path: '/inference', element: <InferencePage /> },
  { path: '/camera', element: <CameraPage /> },
  { path: '/metrics', element: <MetricsPage /> },
];
