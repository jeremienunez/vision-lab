import { createContext, useContext } from 'react';

import { useConfigContext } from './ConfigContext.jsx';
import { usePerceptionData } from '../hooks/usePerceptionData.js';

const PerceptionDataContext = createContext(null);

export function PerceptionDataProvider({ children }) {
  const { config } = useConfigContext();
  const value = usePerceptionData(config);
  return (
    <PerceptionDataContext.Provider value={value}>{children}</PerceptionDataContext.Provider>
  );
}

export function usePerceptionDataContext() {
  const value = useContext(PerceptionDataContext);
  if (!value) throw new Error('usePerceptionDataContext must be used within PerceptionDataProvider');
  return value;
}
