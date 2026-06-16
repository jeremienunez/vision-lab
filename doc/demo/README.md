# Demo Artifacts

Run the executable smoke demo from the repository root:

```bash
npm run demo:fire
```

Run it against a captured phone or webcam image:

```bash
npm run demo:fire -- --image /absolute/path/to/capture.jpg
```

Run the same API path with the real YOLO worker adapter:

```bash
PERCEPTIONLAB_INFERENCE_ENGINE=yolo_cli \
npm run demo:fire -- --image /absolute/path/to/capture.jpg --confidence-threshold 0.25
```

The command starts a transient API, seeds the bundled desk-object dataset, registers a demo model, runs inference, and generates an overlay artifact. It fails if the inference response contains no detections. Custom images must be `.jpg`, `.jpeg`, `.png`, or `.webp`. The default demo uses the local deterministic inference adapter for repeatable product-path validation. With `PERCEPTIONLAB_INFERENCE_ENGINE=yolo_cli`, `POST /models/{model_id}/infer` shells through the worker environment and uses the registered model artifact, defaulting to `.perceptionlab/models/yolo11n.pt`.

Primary inputs and outputs:

- Default input image: `datasets/seed/images/desk-objects.png`
- Optional custom input: `--image /absolute/path/to/capture.jpg`
- JSON output: stdout summary with `detected_classes`, `inference_run_id`, and `overlay_artifact_uri`
- Overlay output: local artifact URI returned by `POST /inference-runs/{run_id}/overlay`

These artifacts make the object-recognition value visible without requiring a long manual API sequence first.

## Real Detector Smoke

Run YOLO through the worker environment on an existing image:

```bash
npm run detect:image -- image.png --model-path .perceptionlab/models/yolo11n.pt --run-name manual
```

Capture a webcam frame and run YOLO:

```bash
npm run detect:webcam -- --device-index 0 --model-path .perceptionlab/models/yolo11n.pt
```

Outputs are stored under `.perceptionlab/real-inference/` and `.perceptionlab/captures/`.
