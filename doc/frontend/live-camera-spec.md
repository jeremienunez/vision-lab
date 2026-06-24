# Live Camera Frontend Spec

## Objective

The live camera feature turns PerceptionLab into a visible real-time perception demo. The first implementation should be browser-based, bounded, and API-compatible with the existing inference contract.

Implementation note: the current dashboard route is `/camera` under `web/src/dashboard/features/camera`. Older `live-camera` labels in this document describe the feature domain, not a separate frontend workspace.

The goal is not to build a full streaming platform first. The goal is to prove that the existing ML infrastructure can accept camera frames, return detections, and render stable overlays in a product UI.

## Product Promise

```text
Open camera -> select model -> capture frame -> infer -> draw detections -> repeat with backpressure
```

The live camera page must show:

- camera permission state
- selected device
- selected model
- confidence threshold
- preview stream
- active inference state
- detections overlay
- latency
- frame history
- last error if any

## First Release Modes

### Preview Mode

The app requests camera permission and shows the video preview. No inference is executed.

### Manual Capture Mode

The user clicks `Capture & Infer`. The frontend captures one frame, sends it to `POST /models/{model_id}/infer`, then draws returned detections over the preview.

### Interval Capture Mode

The user enables bounded interval inference. The frontend samples a frame every configured interval, but never sends a new request while another inference request is in flight.

### Bounded Live Mode

The user runs a bounded live loop with explicit limits:

```text
max duration
max frames
minimum interval
single in-flight request
```

This mirrors the current bounded webcam smoke behavior already present in the project, but moves the experience into a browser product surface.

## Non Goals

The first live camera implementation does not include:

- WebRTC peer-to-peer streaming
- WebSocket frame streaming
- raw video upload
- server-side live sessions
- mobile iOS client
- background camera activation
- automatic recording

These are later extensions after the frame-based product path is stable.

## Architecture

```text
Browser Camera API
  -> MediaStream
  -> HTMLVideoElement preview
  -> Canvas frame capture
  -> Blob image payload
  -> API client POST /models/{model_id}/infer
  -> Detection response
  -> DetectionOverlayCanvas
  -> HUD metrics and local history
```

## Component Tree

```text
LiveCameraRoute
  CameraPermissionPanel
  CameraDeviceSelect
  ModelSelect
  ConfidenceThresholdControl
  LiveCameraPreview
    VideoPreview
    DetectionOverlayCanvas
    CameraHUD
  LiveCameraControls
    CaptureButton
    StartIntervalButton
    StopButton
  InferenceResultPanel
  FrameHistoryRail
  LiveCameraErrorPanel
```

## State Machine

```text
idle
  -> requesting_permission
  -> permission_denied
  -> no_device
  -> preview_ready
  -> capturing_frame
  -> inferring
  -> rendering_overlay
  -> preview_ready
  -> stopped

error transitions:
  requesting_permission -> permission_denied
  preview_ready -> device_lost
  inferring -> inference_failed
  capturing_frame -> capture_failed
```

Each state must map to a clear UI message and available controls.

## Backpressure Rules

Mandatory rules:

- only one inference request may be in flight
- if a frame is due while request is in flight, skip it
- never queue unbounded frames in memory
- stop camera tracks when leaving the route
- stop interval timers when the page unmounts
- clear stale overlay when selected model changes

Reasoning:

```text
live camera UX should demonstrate real-time behavior without accidentally load-testing the API or creating unstable latency
```

## Frame Capture

Recommended capture format for first implementation:

```text
image/jpeg or image/webp
max width: 640 or 960 for demo mode
quality: 0.82 to 0.9
```

The frame sampler should be isolated from React components:

```text
features/live-camera/lib/frameSampler.ts
```

Expected API:

```ts
export type CapturedFrame = {
  blob: Blob;
  width: number;
  height: number;
  capturedAt: string;
};

export function captureVideoFrame(video: HTMLVideoElement): Promise<CapturedFrame>;
```

## Detection Overlay Mapping

The inference API returns normalized bounding boxes. The canvas overlay must map normalized coordinates to the rendered video dimensions.

Mapping:

```text
canvas_x = bbox.x * rendered_video_width
canvas_y = bbox.y * rendered_video_height
canvas_width = bbox.width * rendered_video_width
canvas_height = bbox.height * rendered_video_height
```

The mapping function must be unit-tested independently.

Expected helper:

```ts
export type NormalizedBbox = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export type PixelBbox = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export function scaleBboxToCanvas(
  bbox: NormalizedBbox,
  canvasWidth: number,
  canvasHeight: number,
): PixelBbox;
```

## Detection Rendering Rules

Overlay labels should show:

```text
class_name confidence latency optional_depth
```

Example:

```text
cup 89% · 42 ms
cup 89% · 0.4 m
```

Rules:

- low confidence detections can be visually muted
- class name and confidence are mandatory
- depth is optional and displayed only if present
- stale detections clear or fade when new inference starts
- overlay cannot block camera controls

## API Integration

The first release uses the existing endpoint:

```text
POST /models/{model_id}/infer
multipart/form-data
  image=<captured frame>
  confidence_threshold=<number>
```

The frontend must support API key auth through the shared API client. If the API returns `401` or `403`, the live camera page shows an API auth error and stops interval inference.

## Model Selection

The live camera page should require a selected model before inference.

Selection source:

```text
GET /models
```

Preferred default:

```text
promoted model if available
else latest candidate model
else disabled inference controls
```

## Latency HUD

The HUD should show:

- API latency returned by server
- client round-trip latency
- capture interval
- inference mode
- in-flight state
- detection count

Example:

```text
model: desk-objects-v1 · mode: interval · latency: 48 ms · detections: 3
```

## Local Run History

The frontend keeps a small local history of recent frame results.

```text
max entries: 20
stored data: timestamp, thumbnail object URL, detections count, latency, error state
```

No camera frames should be persisted automatically to the backend in the first implementation. Explicit sample ingestion can come later.

## Error Handling

Expected errors:

- permission denied
- no camera device
- unsupported browser
- selected model missing
- API key missing
- API key invalid
- inference failed
- response malformed
- camera track ended

Each error must include:

- user-readable message
- recovery action
- debug-safe code

## Privacy Rules

- camera starts only after explicit user action
- camera stops when user clicks stop
- camera tracks stop when route unmounts
- no automatic recording
- no automatic upload outside explicit inference
- no hidden background camera usage

## Performance Budget

Initial demo budget:

```text
preview FPS: browser default
inference frequency: 1-2 FPS max in interval mode
max in-flight requests: 1
frame width: 640 or 960 depending demo quality
UI render target: no visible layout thrashing
```

If real YOLO inference is slow, the UI should stay stable and show latency rather than trying to fake real-time speed.

## Feature Flags

Recommended flags:

```text
PERCEPTIONLAB_FRONTEND_LIVE_CAMERA=true
PERCEPTIONLAB_FRONTEND_CAMERA_MAX_WIDTH=960
PERCEPTIONLAB_FRONTEND_CAMERA_INTERVAL_MS=1000
```

## Tests

Unit tests:

- bbox scaling
- state machine transitions
- frame interval backpressure
- confidence filtering
- API error mapping

Component tests:

- permission panel states
- model missing state
- inference loading state
- overlay with detections
- auth error state

Manual smoke:

```text
open /camera
allow camera
select model
manual capture
verify detections overlay
start interval mode
verify only one request in flight
stop camera
verify camera indicator turns off
```

## BDD Scenarios

```gherkin
Feature: Live camera inference
  Users must be able to run bounded camera-frame inference from the browser.

  Scenario: Camera preview starts after explicit permission
    Given the frontend is open on the live camera page
    When I click "Start camera"
    And I grant camera permission
    Then the video preview should be visible
    And no inference request should be sent yet

  Scenario: Manual frame inference draws detections
    Given the camera preview is ready
    And a model is selected
    When I click "Capture & Infer"
    Then one frame should be sent to the inference API
    And detections should be drawn on the overlay
    And latency should be displayed

  Scenario: Missing API key stops live inference
    Given API key auth is enabled on the backend
    And the frontend has no API key configured
    When I try to run live inference
    Then the page should show an API auth error
    And interval inference should not continue
```

## Implementation Order

1. Create route and static UI shell.
2. Implement tokenized camera panel.
3. Implement permission and device selection.
4. Implement preview-only mode.
5. Implement frame capture helper.
6. Implement manual inference.
7. Implement canvas overlay and bbox scaling tests.
8. Implement interval mode with backpressure.
9. Add local run history.
10. Add README demo instructions.

## Definition Of Done

The live camera slice is done if:

- camera permission is explicit
- camera stops cleanly
- a selected model is required
- one frame can be inferred manually
- interval mode skips frames under backpressure
- detections map correctly to the overlay
- auth errors are handled clearly
- no unbounded frame queue exists
- the feature can be demonstrated from a local running API
