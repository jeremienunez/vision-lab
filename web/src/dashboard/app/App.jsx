import { Route, Routes } from 'react-router-dom';

import { ConfigProvider } from '../context/ConfigContext.jsx';
import { PerceptionDataProvider } from '../context/PerceptionDataContext.jsx';
import { AppLayout } from '../layout/AppLayout.jsx';
import { ROUTES } from './routes.jsx';

export function App() {
  return (
    <ConfigProvider>
      <PerceptionDataProvider>
        <Routes>
          <Route element={<AppLayout />}>
            {ROUTES.map((route) => (
              <Route key={route.path} path={route.path} element={route.element} />
            ))}
          </Route>
        </Routes>
      </PerceptionDataProvider>
    </ConfigProvider>
  );
}
