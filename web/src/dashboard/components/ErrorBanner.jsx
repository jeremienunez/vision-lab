import { AlertTriangle } from 'lucide-react';

export function ErrorBanner({ children }) {
  return (
    <div
      className="flex items-center gap-2 rounded-xl border border-red/30 bg-red-soft px-4 py-3 text-sm text-red"
      role="status"
    >
      <AlertTriangle size={18} aria-hidden="true" />
      <span>{children}</span>
    </div>
  );
}
