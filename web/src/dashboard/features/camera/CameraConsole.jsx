import { CameraControls } from './CameraControls.jsx';
import { DetectionList } from './DetectionList.jsx';
import { ErrorBanner } from '../../components/ErrorBanner.jsx';

export function CameraConsole({
  models,
  selectedModelId,
  confidenceThreshold,
  running,
  busy,
  error,
  result,
  onSelectModel,
  onConfidenceChange,
  onStart,
  onStop,
  onCapture,
}) {
  return (
    <div className="flex flex-col gap-4">
      <CameraControls
        models={models}
        selectedModelId={selectedModelId}
        confidenceThreshold={confidenceThreshold}
        running={running}
        busy={busy}
        onSelectModel={onSelectModel}
        onConfidenceChange={onConfidenceChange}
        onStart={onStart}
        onStop={onStop}
        onCapture={onCapture}
      />
      {error && <ErrorBanner>{error}</ErrorBanner>}
      <DetectionList result={result} />
    </div>
  );
}
