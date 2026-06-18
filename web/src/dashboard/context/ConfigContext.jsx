import { createContext, useContext } from 'react';

import { useConfig } from '../hooks/useConfig.js';

const ConfigContext = createContext(null);

export function ConfigProvider({ children }) {
  const value = useConfig();
  return <ConfigContext.Provider value={value}>{children}</ConfigContext.Provider>;
}

export function useConfigContext() {
  const value = useContext(ConfigContext);
  if (!value) throw new Error('useConfigContext must be used within ConfigProvider');
  return value;
}
