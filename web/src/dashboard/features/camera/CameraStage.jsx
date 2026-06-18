import { ScanSearch } from 'lucide-react';

import { StatusBadge } from '../../components/StatusBadge.jsx';
import { buildDetectionOverlayItems } from '../../camera-overlay.js';

export function CameraStage({ videoRef, canvasRef, busy, status, result }) {
  const detections = result?.detections ?? [];
  const overlayItems = buildDetectionOverlayItems(detections, { mirrored: true });

  return (
    <div className="relative aspect-video overflow-hidden rounded-2xl border border-line bg-nav">
      <video
        ref={videoRef}
        className="h-full w-full -scale-x-100 object-cover"
        aria-label="Live webcam preview"
        autoPlay
        muted
        playsInline
      />
      <canvas ref={canvasRef} hidden />

      {overlayItems.length > 0 && (
        <div className="pointer-events-none absolute inset-0" aria-label="Detected objects on camera preview">
          {overlayItems.map((item, index) => (
            <div
              key={`${item.label}-${index}`}
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

      <div className="absolute left-3 top-3">
        <StatusBadge icon={ScanSearch} label={busy ? 'Analyzing frame' : status} tone={busy ? 'pending' : 'success'} />
      </div>
    </div>
  );
}
