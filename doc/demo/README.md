# Demo Artifacts

Run the executable smoke demo from the repository root:

```bash
npm run demo:fire
```

The command starts a transient API, seeds the bundled desk-object dataset, registers a demo model, runs inference, and generates an overlay artifact. It fails if the inference response contains no detections. The current demo uses the local deterministic inference adapter, so it validates the product path rather than real model accuracy.

Primary inputs and outputs:

- Input image: `datasets/seed/images/desk-objects.png`
- JSON output: stdout summary with `detected_classes`, `inference_run_id`, and `overlay_artifact_uri`
- Overlay output: local artifact URI returned by `POST /inference-runs/{run_id}/overlay`

These artifacts make the object-recognition value visible without requiring a long manual API sequence first.
