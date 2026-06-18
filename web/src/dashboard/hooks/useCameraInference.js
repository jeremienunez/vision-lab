import { useCallback, useEffect, useMemo, useRef, useState } from 'react';

import { createPerceptionApi } from '../perception-api.js';
import { orderCameraModels } from '../camera-models.js';
import { DEFAULT_CONFIDENCE_THRESHOLD } from '../camera-controls.js';

const CAMERA_INTERVAL_MS = 10_000;

function canvasToBlob(canvas, type, quality) {
  return new Promise((resolve, reject) => {
    canvas.toBlob((blob) => {
      if (blob) resolve(blob);
      else reject(new Error('Unable to capture webcam frame.'));
    }, type, quality);
  });
}

export function useCameraInference({ config, models }) {
  const [selectedModelId, setSelectedModelId] = useState('');
  const [confidenceThreshold, setConfidenceThreshold] = useState(DEFAULT_CONFIDENCE_THRESHOLD);
  const [running, setRunning] = useState(false);
  const [busy, setBusy] = useState(false);
  const [status, setStatus] = useState('Camera idle');
  const [error, setError] = useState('');
  const [result, setResult] = useState(null);

  const videoRef = useRef(null);
  const canvasRef = useRef(null);
  const streamRef = useRef(null);
  const captureInFlightRef = useRef(false);

  const inferenceModels = useMemo(() => orderCameraModels(models), [models]);
  const selectedModel = useMemo(
    () => inferenceModels.find((model) => model.id === selectedModelId) ?? inferenceModels[0] ?? null,
    [selectedModelId, inferenceModels],
  );

  useEffect(() => {
    if (!selectedModelId && inferenceModels[0]) {
      setSelectedModelId(inferenceModels[0].id);
    }
  }, [selectedModelId, inferenceModels]);

  const stop = useCallback((nextStatus = 'Camera stopped') => {
    streamRef.current?.getTracks().forEach((track) => track.stop());
    streamRef.current = null;
    if (videoRef.current) videoRef.current.srcObject = null;
    setRunning(false);
    setBusy(false);
    setStatus(nextStatus);
  }, []);

  const capture = useCallback(async () => {
    if (captureInFlightRef.current || !videoRef.current || !selectedModel) return;
    if (!videoRef.current.videoWidth || !videoRef.current.videoHeight) return;

    captureInFlightRef.current = true;
    setBusy(true);
    setError('');
    setStatus('Capturing frame');

    try {
      const canvas = canvasRef.current;
      canvas.width = videoRef.current.videoWidth;
      canvas.height = videoRef.current.videoHeight;
      canvas.getContext('2d').drawImage(videoRef.current, 0, 0, canvas.width, canvas.height);
      const imageBlob = await canvasToBlob(canvas, 'image/jpeg', 0.86);
      const api = createPerceptionApi(config);
      const inference = await api.runModelInference({
        modelId: selectedModel.id,
        imageBlob,
        filename: `webcam-frame-${Date.now()}.jpg`,
        confidenceThreshold,
      });

      setResult({ ...inference, capturedAt: new Date(), modelName: selectedModel.name });
      setStatus(`Last analyzed ${new Date().toLocaleTimeString()}`);
    } catch (captureError) {
      setError(captureError.message);
      setStatus('Inference failed');
    } finally {
      captureInFlightRef.current = false;
      setBusy(false);
    }
  }, [config, confidenceThreshold, selectedModel]);

  const start = useCallback(async () => {
    setError('');

    try {
      if (!selectedModel) throw new Error('Select a model before starting camera inference.');
      if (!navigator.mediaDevices?.getUserMedia) {
        throw new Error('Camera access is not available in this browser.');
      }

      const stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'user', width: { ideal: 1280 }, height: { ideal: 720 } },
        audio: false,
      });
      streamRef.current = stream;
      if (!videoRef.current) throw new Error('Camera preview is not ready yet.');

      videoRef.current.srcObject = stream;
      await videoRef.current.play();
      setRunning(true);
      setStatus('Camera live');
      setTimeout(() => capture(), 250);
    } catch (cameraStartError) {
      setError(cameraStartError.message);
      stop('Camera blocked');
    }
  }, [capture, selectedModel, stop]);

  useEffect(() => {
    if (!running) return undefined;
    const intervalId = window.setInterval(() => capture(), CAMERA_INTERVAL_MS);
    return () => window.clearInterval(intervalId);
  }, [running, capture]);

  useEffect(
    () => () => {
      streamRef.current?.getTracks().forEach((track) => track.stop());
      streamRef.current = null;
    },
    [],
  );

  return {
    videoRef,
    canvasRef,
    models: inferenceModels,
    selectedModel,
    selectedModelId: selectedModel?.id ?? '',
    setSelectedModelId,
    confidenceThreshold,
    setConfidenceThreshold,
    running,
    busy,
    status,
    error,
    result,
    start,
    stop,
    capture,
  };
}
