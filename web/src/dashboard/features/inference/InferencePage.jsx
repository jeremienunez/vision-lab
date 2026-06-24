import { useEffect, useMemo, useState } from 'react';
import { ImageUp, ScanSearch } from 'lucide-react';

import { buildDetectionOverlayItems } from '../../camera-overlay.js';
import { orderCameraModels } from '../../camera-models.js';
import { normalizeConfidenceThreshold } from '../../camera-controls.js';
import { createPerceptionApi } from '../../perception-api.js';
import { useConfigContext } from '../../context/ConfigContext.jsx';
import { usePerceptionDataContext } from '../../context/PerceptionDataContext.jsx';
import { Button } from '../../components/Button.jsx';
import { DataTable } from '../../components/DataTable.jsx';
import { EmptyState } from '../../components/EmptyState.jsx';
import { ErrorBanner } from '../../components/ErrorBanner.jsx';
import { Field } from '../../components/Field.jsx';
import { Panel } from '../../components/Panel.jsx';
import { Select } from '../../components/Select.jsx';
import { TextInput } from '../../components/TextInput.jsx';
import {
  buildInferenceRequest,
  buildInferenceResultView,
  validateInferenceImageFile,
} from './inference-lab.js';

const DETECTION_COLUMNS = [
  { key: 'className', header: 'Class' },
  { key: 'confidenceLabel', header: 'Confidence' },
  { key: 'bboxLabel', header: 'BBox' },
];

export function InferencePage() {
  const { config } = useConfigContext();
  const { payload } = usePerceptionDataContext();
  const models = useMemo(() => orderCameraModels(payload.models ?? []), [payload.models]);
  const [selectedModelId, setSelectedModelId] = useState('');
  const [confidenceThreshold, setConfidenceThreshold] = useState(0.25);
  const [imageFile, setImageFile] = useState(null);
  const [previewUrl, setPreviewUrl] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState('');
  const [result, setResult] = useState(null);

  useEffect(() => {
    if (!selectedModelId && models[0]) setSelectedModelId(models[0].id);
  }, [models, selectedModelId]);

  useEffect(() => {
    if (!imageFile) {
      setPreviewUrl('');
      return undefined;
    }

    const nextPreviewUrl = URL.createObjectURL(imageFile);
    setPreviewUrl(nextPreviewUrl);

    return () => URL.revokeObjectURL(nextPreviewUrl);
  }, [imageFile]);

  function updateImageFile(file) {
    setImageFile(file);
    setResult(null);
    setError(validateInferenceImageFile(file));
  }

  async function submitInference(event) {
    event.preventDefault();
    setSubmitting(true);
    setError('');
    setResult(null);

    try {
      const api = createPerceptionApi(config);
      const inference = await api.runModelInference(
        buildInferenceRequest({
          modelId: selectedModelId,
          imageFile,
          confidenceThreshold,
        }),
      );
      setResult(inference);
    } catch (submitError) {
      setError(submitError.message);
    } finally {
      setSubmitting(false);
    }
  }

  const resultView = buildInferenceResultView(result);
  const overlayItems = buildDetectionOverlayItems(result?.detections ?? []);

  return (
    <div className="grid min-w-0 gap-6 xl:grid-cols-[minmax(0,0.85fr)_minmax(0,1.15fr)]">
      <Panel id="single-image-inference" title="Image inference" action="single image" icon={ImageUp}>
        <form className="flex min-w-0 flex-col gap-4" onSubmit={submitInference}>
          <Field label="Model">
            <Select
              value={selectedModelId}
              onChange={(event) => setSelectedModelId(event.target.value)}
              disabled={models.length === 0 || submitting}
              required
            >
              {models.length === 0 ? (
                <option value="">No model available</option>
              ) : (
                models.map((model) => (
                  <option key={model.id} value={model.id}>
                    {model.name} / {model.model_family} / {model.status}
                  </option>
                ))
              )}
            </Select>
          </Field>

          <Field label="Image">
            <input
              className="w-full rounded-lg border border-line bg-surface px-3 py-2 text-sm text-ink file:mr-3 file:rounded-md file:border-0 file:bg-cyan file:px-3 file:py-1.5 file:text-sm file:font-medium file:text-white"
              type="file"
              accept="image/jpeg,image/png,image/webp"
              onChange={(event) => updateImageFile(event.target.files?.[0] ?? null)}
              disabled={submitting}
              required
            />
          </Field>

          <Field label="Confidence">
            <TextInput
              type="number"
              min="0"
              max="1"
              step="0.05"
              value={confidenceThreshold}
              onChange={(event) => setConfidenceThreshold(normalizeConfidenceThreshold(Number(event.target.value)))}
              disabled={submitting}
            />
          </Field>

          {error && <ErrorBanner>{error}</ErrorBanner>}

          <div className="flex justify-end">
            <Button
              type="submit"
              variant="primary"
              icon={ScanSearch}
              disabled={submitting || models.length === 0 || Boolean(validateInferenceImageFile(imageFile))}
            >
              {submitting ? 'Running' : 'Run inference'}
            </Button>
          </div>
        </form>
      </Panel>

      <Panel
        id="inference-result"
        title="Inference result"
        action={result ? `${resultView.latencyLabel} / ${resultView.detectionCountLabel}` : 'waiting'}
        icon={ScanSearch}
      >
        <div className="flex min-w-0 flex-col gap-4">
          {previewUrl ? (
            <div className="relative aspect-video overflow-hidden rounded-2xl border border-line bg-nav">
              <img src={previewUrl} alt="Selected inference input" className="h-full w-full object-contain" />
              {overlayItems.length > 0 && (
                <div className="pointer-events-none absolute inset-0" aria-label="Detected objects on selected image">
                  {overlayItems.map((item) => (
                    <div
                      key={`${item.label}-${item.style.left}-${item.style.top}`}
                      className="absolute rounded-md border-2 border-cyan"
                      style={item.style}
                    >
                      <span className="absolute left-0 top-0 flex -translate-y-full items-center gap-1 rounded bg-cyan px-1.5 py-0.5 text-[10px] text-white">
                        <strong>{item.label}</strong>
                        {item.confidenceLabel && <em className="not-italic opacity-80">{item.confidenceLabel}</em>}
                      </span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          ) : (
            <EmptyState icon={ImageUp} label="No image selected" />
          )}

          {result ? (
            <div className="flex min-w-0 flex-col gap-3">
              <div className="rounded-lg border border-line bg-surface-soft px-3 py-2 text-sm text-muted">
                <span className="font-medium text-ink">Run</span> {resultView.runId}
              </div>
              {resultView.detections.length === 0 ? (
                <EmptyState icon={ScanSearch} label="No detections" />
              ) : (
                <DataTable
                  columns={DETECTION_COLUMNS}
                  rows={resultView.detections}
                  getRowKey={(detection) => detection.id}
                />
              )}
            </div>
          ) : null}
        </div>
      </Panel>
    </div>
  );
}
