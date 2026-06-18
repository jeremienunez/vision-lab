import { Play, ScanSearch, Square } from 'lucide-react';

import { Field } from '../../components/Field.jsx';
import { Select } from '../../components/Select.jsx';
import { TextInput } from '../../components/TextInput.jsx';
import { Button } from '../../components/Button.jsx';

export function CameraControls({
  models,
  selectedModelId,
  confidenceThreshold,
  running,
  busy,
  onSelectModel,
  onConfidenceChange,
  onStart,
  onStop,
  onCapture,
}) {
  return (
    <div className="flex flex-wrap items-end gap-4">
      <Field label="Model" className="flex-1">
        <Select value={selectedModelId} onChange={(event) => onSelectModel(event.target.value)} disabled={running}>
          {models.length === 0 ? (
            <option value="">No model available</option>
          ) : (
            models.map((model) => (
              <option value={model.id} key={model.id}>
                {model.name} / {model.model_family} / {model.status}
              </option>
            ))
          )}
        </Select>
      </Field>

      <Field label="Confidence" className="w-28">
        <TextInput
          type="number"
          min="0"
          max="1"
          step="0.05"
          value={confidenceThreshold}
          onChange={(event) => onConfidenceChange(Number(event.target.value))}
        />
      </Field>

      <div className="flex items-center gap-2">
        {running ? (
          <Button icon={Square} onClick={onStop}>
            Stop
          </Button>
        ) : (
          <Button variant="primary" icon={Play} onClick={onStart} disabled={models.length === 0}>
            Start camera
          </Button>
        )}
        <Button icon={ScanSearch} onClick={onCapture} disabled={!running || busy}>
          Analyze now
        </Button>
      </div>
    </div>
  );
}
