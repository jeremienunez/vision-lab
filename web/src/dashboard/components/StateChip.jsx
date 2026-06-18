import { STATUS_TONE } from './tone.js';

export function StateChip({ status, children }) {
  return (
    <span
      className={`inline-flex rounded-full px-2.5 py-0.5 text-xs font-medium ${
        STATUS_TONE[status] ?? STATUS_TONE.neutral
      }`}
    >
      {children ?? status}
    </span>
  );
}
