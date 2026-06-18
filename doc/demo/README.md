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

Run a bounded live webcam loop while keeping YOLO loaded in memory:

```bash
npm run detect:webcam-live -- --device-index 0 --model-path .perceptionlab/models/yolo11n.pt --frame-limit 10 --confidence-threshold 0.10
```

Outputs are stored under `.perceptionlab/real-inference/` and `.perceptionlab/captures/`.

## YOLO Fine-Tuning Path

When the live detector confuses a target object, such as reading a phone as `remote`,
collect annotated samples for the target classes, create a dataset version, then create
a queued training job with:

```json
{
  "dataset_version_id": "<dataset_version_id>",
  "model_family": "yolo_finetune",
  "base_model": ".perceptionlab/models/yolo11n.pt",
  "hyperparameters": {
    "epochs": 20,
    "batch_size": 4,
    "image_size": 640,
    "learning_rate": 0.001
  }
}
```

Run one fine-tune worker pass:

```bash
make worker-yolo-once
```

The worker writes a materialized YOLO dataset under the configured artifact root,
trains with Ultralytics, and registers the resulting `best.pt` as a candidate model.
Start inference with the real API path using `make api-real`, then select the new
candidate model in the dashboard camera panel.

For the phone-vs-remote correction pass, keep the first training mix focused on:

```text
phone,remote,person,laptop,mouse,keyboard
```

Use external YOLO exports for broad coverage and local webcam captures for the real
room lighting. A usable first mix is Open Images V7 subsets for the six classes above,
Roboflow Mobile phone detection, Roboflow E-collect, Roboflow Classroom Cell Phone
Detection, and local hard negatives such as a real remote, mouse, empty hand, and
rectangular objects that are not phones.

Import a YOLO export into the local seed-manifest format:

```bash
cd worker
PERCEPTIONLAB_DATA_ROOT=/media/jerem/ubuntu1/perceptionlab/datasets \
UV_CACHE_DIR=../.perceptionlab/cache/uv \
uv run perception-worker ingest-yolo \
  /media/jerem/ubuntu1/perceptionlab/raw/phone-remote-yolo \
  --target-name phone-remote-mix \
  --classes phone,remote,person,laptop,mouse,keyboard \
  --split train
```

Then seed and train:

```bash
PERCEPTIONLAB_SEED_DATASET_ROOT=/media/jerem/ubuntu1/perceptionlab/datasets/phone-remote-mix make seed
make worker-yolo-once
```
