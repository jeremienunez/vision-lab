# Frontend Architecture Spec

## Objective

The frontend must make PerceptionLab visible as a product, not only as an API. It should expose the current ML infrastructure through a clean dashboard, an inference lab, and a live camera experience while preserving the existing API-first architecture.

The frontend is not allowed to become the source of truth. The Rust API remains the product boundary. The frontend is a typed client that reads datasets, samples, jobs, metrics, models, exports, inference runs, overlays, and future camera sessions from the API.

## Product Surfaces

The first frontend surface is a minimal dashboard for platform state:

- datasets and dataset versions
- training jobs and current statuses
- training metrics and class metrics
- registered models and promoted model state
- model exports
- latest inference runs and overlay artifacts

The second frontend surface is an inference lab:

- select a model
- upload an image
- run inference through `POST /models/{model_id}/infer`
- display detections, confidence scores, latency, and overlay
- keep only local current-run state in the first slice; server-side history can come later from persisted inference runs

The third frontend surface is a live camera lab:

- request camera permission explicitly
- select an available device
- preview camera stream
- capture frames manually or on a bounded interval
- send frames to the existing inference endpoint
- draw bounding boxes over the video preview
- display latency and FPS budget

The fourth frontend surface is an experimental lab:

- compare fine-tuning experiments
- inspect dataset versions used by experiments
- inspect metric evolution
- compare baseline, fine-tuned, ONNX, and CoreML exports
- surface qualitative prediction examples

## Recommended Stack

The recommended frontend stack is:

```text
React + TypeScript
Vite application shell
CSS custom properties for design tokens
TanStack Query-style server state
Small client store only for UI/session state
Canvas overlay for detections and live camera boxes
Typed API client generated from or aligned with OpenAPI
Playwright-style E2E tests later
```

The stack choice follows the project constraints:

- API-first backend already exists.
- The app needs strong typing and explicit contracts.
- The dashboard needs server state caching, refetching, and mutation handling.
- The live camera needs browser APIs, canvas rendering, and careful backpressure.
- The repo already uses Node-based local automation, and the current `web/` Vite workspace integrates with the existing scripts.

## Repository Layout

Implementation note: the current production frontend lives under `web/` and uses the repository's Vite app configured by `web/vite.config.mjs`. Current slices should extend `web/src/dashboard` instead of creating a second app shell.

Reference target structure for a future extraction, not the current implementation root:

```text
frontend/
  package.json
  index.html
  vite.config.ts
  tsconfig.json

  src/
    app/
      App.tsx
      router.tsx
      providers.tsx
      queryClient.ts
      env.ts

    routes/
      dashboard.route.tsx
      datasets.route.tsx
      jobs.route.tsx
      models.route.tsx
      inference-lab.route.tsx
      live-camera.route.tsx
      experiments.route.tsx
      settings.route.tsx

    layouts/
      AppShell.tsx
      Sidebar.tsx
      Topbar.tsx
      PageHeader.tsx

    features/
      dashboard/
      datasets/
      training-jobs/
      model-registry/
      inference-lab/
      live-camera/
      experiments/
      api-auth/

    entities/
      dataset/
      training-job/
      model/
      inference-run/
      metric/

    shared/
      api/
        client.ts
        errors.ts
        schemas.ts
      components/
        Button.tsx
        Card.tsx
        DataTable.tsx
        StatusBadge.tsx
        MetricCard.tsx
        EmptyState.tsx
      design/
        tokens.css
        tokens.ts
      lib/
        formatters.ts
        result.ts
        invariant.ts
      testing/
        test-utils.tsx
```

The important convention is the same as the backend: folder names describe responsibility. No vague `utils/`, no direct API calls inside visual components, and no raw color values inside components.

## Dependency Rules

Allowed dependency direction:

```text
routes -> features -> entities -> shared
```

Forbidden dependency direction:

```text
shared -> entities
entities -> features
features -> routes
```

Components in `shared/components` are product-agnostic. They do not know what a dataset, job, model, or inference run is.

Components in `entities/*` represent domain objects visually.

Components in `features/*` orchestrate API calls, mutations, local interaction state, and composed UI.

Routes only compose feature-level components and page layout.

## API Client Rules

Rules:

- No raw `fetch` inside React components.
- No untyped JSON passed to UI components.
- Every API response has a frontend type.
- Every mutation exposes loading, success, validation error, and failure states.
- API key handling lives in one API client layer.

Recommended client boundary:

```text
shared/api/client.ts
  -> request<TResponse>()
  -> inject x-api-key if configured
  -> map HTTP errors to typed ApiError
  -> never leak stack traces or raw Response objects into components
```

The first auth mode is local API key:

```text
PERCEPTIONLAB_API_BASE_URL=http://127.0.0.1:8080
PERCEPTIONLAB_FRONTEND_API_KEY=dev-secret
```

The frontend should store the API key only in local development settings or environment configuration. It should not hard-code secrets in committed code.

## Server State And Client State

Server state:

- datasets
- samples
- dataset versions
- training jobs
- training metrics
- models
- model exports
- inference runs
- overlays

Client state:

- selected model
- selected camera device
- live camera mode
- overlay visibility
- confidence threshold
- local frame history
- API key input for local development

Server state must be cached and invalidated after mutations.

Client state must stay small and local. Do not create a global store for data that belongs to the API.

## Route Map

```text
/                     -> Dashboard
/datasets             -> Dataset list and creation entry
/training             -> Training job list and creation entry
/models               -> Model registry
/inference            -> Single-image inference lab
/camera               -> Camera preview and bounded live inference
/metrics              -> Training metrics overview

Future routes:

/experiments          -> Fine-tuning experiment comparison
/settings             -> Dedicated local API base URL and API key configuration if the drawer outgrows the shell
```

## Dashboard Composition

The dashboard should answer four questions immediately:

1. What datasets exist?
2. What jobs are running or failed?
3. What model is currently promoted or most recent?
4. Can I run a quick inference now?

Primary cards:

- Dataset count
- Training jobs by status
- Latest model
- Latest inference latency
- Latest export status
- Fire smoke shortcut

## Live Camera Architecture

The live camera page is not a WebSocket feature in the first implementation. It should reuse the existing image inference endpoint by sampling frames and sending bounded multipart requests.

Initial data flow:

```text
getUserMedia()
  -> HTMLVideoElement preview
  -> FrameSampler captures canvas frame
  -> Blob JPEG/WebP
  -> POST /models/{model_id}/infer
  -> Detection response
  -> Canvas overlay mapped to video dimensions
  -> Latency and result history
```

Backpressure rule:

```text
Only one inference request may be in flight per camera session.
```

Modes:

- preview only
- manual capture
- interval capture
- bounded live loop

The first release should not attempt raw video streaming. It should use bounded frame inference because the backend already supports image inference and the existing CLI already has bounded webcam commands.

## Live Camera State Machine

```text
idle
  -> requesting_permission
  -> preview_ready
  -> capturing_frame
  -> inferring
  -> rendering_overlay
  -> preview_ready

error states:
  permission_denied
  no_device
  inference_failed
  unsupported_browser
```

The UI must make camera state explicit. The user should always know whether the camera is inactive, previewing, capturing, or sending frames.

## Detection Overlay Rules

The overlay should be a canvas positioned over the video element.

Rules:

- overlay scales normalized bboxes to visible video dimensions
- labels use semantic detection tokens
- confidence threshold filters results before drawing
- stale detections fade or clear when a new frame starts
- latency is displayed separately from the bbox label

Do not draw directly into the video. The overlay must be disposable and independently testable.

## Experimental Lab Architecture

The experimental lab is a read-heavy UI first. It should compare experiment runs without introducing notebook-driven behavior into the product.

Initial view:

- experiment run table
- dataset version used
- model family
- base model
- hyperparameters summary
- mAP50, mAP50_95, precision, recall
- artifact links
- qualitative prediction grid

The first implementation can read from existing training jobs and model registry metadata. A dedicated `experiments` API can come later when the fine-tuning pass proves which metadata is worth promoting to first-class product concepts.

## Component Rules

Mandatory components:

```text
StatusBadge
MetricCard
ModelCard
DatasetCard
JobTimeline
InferenceResultPanel
DetectionOverlayCanvas
CameraPermissionPanel
LiveCameraPreview
FrameHistoryRail
ExperimentComparisonTable
```

Rules:

- components receive typed props
- components do not fetch data directly
- components do not contain raw API URLs
- components do not contain hard-coded colors
- detection UI consumes tokens from `tokens.css` or `tokens.ts`

## Testing Strategy

Frontend test layers:

```text
unit tests
  -> formatters, bbox scaling, state machines

component tests
  -> cards, status badges, overlay canvas, camera permission states

API mock tests
  -> dashboard loads datasets/jobs/models

E2E smoke
  -> open app, configure API URL, load dashboard, run fake inference

manual hardware smoke
  -> live camera device selection and frame inference
```

Live camera tests should keep hardware-specific behavior out of CI. CI can test the state machine, frame sampler abstraction, and mock MediaStream behavior. Real camera validation stays manual or nightly on a known environment.

## Implementation Order

Delivered in `web/`:

- Vite app shell, routed layout, and dashboard navigation.
- Tailwind token theme and shared dashboard controls.
- API client with API key injection.
- Read-only dashboard cards for datasets, jobs, models, and metrics.
- Model registry route and single-image inference lab at `/inference`.
- Camera preview, manual frame inference, and bounded interval mode at `/camera`.

Remaining order:

1. Add job-level training logs.
2. Add experimental lab read-only comparison table.
3. Add quantization/mobile metadata to model registry summaries.
4. Add calibration dataset validation and mobile export/benchmark smoke checks.

## Definition Of Done

A frontend slice is done only if:

- it uses design tokens instead of raw visual values
- it uses the typed API client instead of direct fetch
- it has loading, empty, success, and error states
- it handles missing API key or wrong API key states
- it is responsive enough for laptop and desktop demo
- live camera features require explicit user action
- bbox scaling is tested independently from the browser camera

## Non Goals For First Frontend Slice

- no full authentication product
- no role-based authorization
- no complex dashboard builder
- no direct video upload pipeline
- no WebSocket streaming inference
- no mobile app UI yet
- no replacing the CLI fire smoke

The frontend must make the existing infrastructure legible. It must not slow down the ML systems work.
