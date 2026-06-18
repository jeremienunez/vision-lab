import { CONTROL_CLASS } from './controls.js';

export function TextInput({ className = '', ...props }) {
  return <input className={`${CONTROL_CLASS} ${className}`} {...props} />;
}
