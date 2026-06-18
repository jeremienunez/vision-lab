import { useState } from 'react';

const STORAGE_BASE_URL = 'perceptionlab.apiBaseUrl';
const STORAGE_API_KEY = 'perceptionlab.apiKey';

const DEFAULT_CONFIG = {
  baseUrl: import.meta.env.VITE_PERCEPTIONLAB_API_BASE_URL ?? '/api',
  apiKey: import.meta.env.VITE_PERCEPTIONLAB_API_KEY ?? '',
};

export function loadConfig() {
  if (typeof window === 'undefined') return DEFAULT_CONFIG;

  return {
    baseUrl: window.localStorage.getItem(STORAGE_BASE_URL) ?? DEFAULT_CONFIG.baseUrl,
    apiKey: window.localStorage.getItem(STORAGE_API_KEY) ?? DEFAULT_CONFIG.apiKey,
  };
}

export function persistConfig(nextConfig) {
  if (typeof window === 'undefined') return;

  window.localStorage.setItem(STORAGE_BASE_URL, nextConfig.baseUrl);
  window.localStorage.setItem(STORAGE_API_KEY, nextConfig.apiKey);
}

export function useConfig() {
  const [config, setConfigState] = useState(loadConfig);

  function setConfig(nextConfig) {
    persistConfig(nextConfig);
    setConfigState(nextConfig);
  }

  return { config, setConfig };
}
