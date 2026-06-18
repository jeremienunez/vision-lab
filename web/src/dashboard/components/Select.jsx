import { CONTROL_CLASS } from './controls.js';

export function Select({ className = '', children, ...props }) {
  return (
    <select className={`${CONTROL_CLASS} ${className}`} {...props}>
      {children}
    </select>
  );
}
