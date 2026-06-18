import { Camera } from 'lucide-react';

import { usePerceptionDataContext } from '../../context/PerceptionDataContext.jsx';
import { useConfigContext } from '../../context/ConfigContext.jsx';
import { useCameraInference } from '../../hooks/useCameraInference.js';
import { normalizeConfidenceThreshold } from '../../camera-controls.js';
import { Panel } from '../../components/Panel.jsx';
import { CameraStage } from './CameraStage.jsx';
import { CameraConsole } from './CameraConsole.jsx';

export function CameraPage() {
  const { payload } = usePerceptionDataContext();
  const { config } = useConfigContext();
  const camera = useCameraInference({ config, models: payload.models });

  return (
    <Panel
      id="camera"
      title="Camera inference"
      action={camera.running ? '10 sec cadence' : 'manual start'}
      icon={Camera}
      wide
    >
      <div className="grid gap-6 lg:grid-cols-2">
        <CameraStage
          videoRef={camera.videoRef}
          canvasRef={camera.canvasRef}
          busy={camera.busy}
          status={camera.status}
          result={camera.result}
        />
        <CameraConsole
          models={camera.models}
          selectedModelId={camera.selectedModelId}
          confidenceThreshold={camera.confidenceThreshold}
          running={camera.running}
          busy={camera.busy}
          error={camera.error}
          result={camera.result}
          onSelectModel={camera.setSelectedModelId}
          onConfidenceChange={(value) => camera.setConfidenceThreshold(normalizeConfidenceThreshold(value))}
          onStart={camera.start}
          onStop={() => camera.stop()}
          onCapture={camera.capture}
        />
      </div>
    </Panel>
  );
}
