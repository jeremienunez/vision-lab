# Experimental Fine-Tuning Pass

## Objective

This pass opens a controlled exploration track for fine-tuning computer vision models without destabilizing the product platform. The goal is to produce evidence: which model, dataset version, training configuration, and export path create the best portfolio-grade perception demo.

The experimental pass is not a notebook playground. It is an experiment protocol that plugs into the existing dataset ingestion, dataset versioning, training jobs, metrics, model registry, export, and inference flows.

## Core Question

```text
Can a fine-tuned model outperform the baseline detector on a bounded target dataset while staying deployable through the PerceptionLab pipeline?
```

Secondary questions:

- Which dataset version is good enough for the demo?
- Which classes produce stable detections in camera conditions?
- Which training configuration improves mAP50 without overfitting?
- Does the exported ONNX/CoreML model preserve acceptable inference behavior?
- Can the model support the live camera frontend with acceptable latency?

## Candidate Model Families

Initial families:

```text
baseline_yolo
fine_tuned_yolo
small_custom_torch
```

The first useful baseline is the existing real YOLO smoke path. Fine-tuning should improve domain-specific detection, not simply prove that a pretrained model already works.

## Candidate Datasets

### CPPE-5 Validation Track

Purpose: verify the pipeline with a known external object detection dataset.

Classes:

```text
Coverall
Face_Shield
Gloves
Goggles
Mask
```

Use this track to validate ingestion, training, metrics, and exports with a bounded external dataset.

### Desk Objects Demo Track

Purpose: build the final portfolio demo for live camera.

Candidate classes:

```text
cup
book
phone
keyboard
mouse
laptop
bottle
screen
notebook
```

This track should use captured phone/webcam images and be evaluated under realistic camera preview conditions.

## Experiment Naming

Experiment IDs should be deterministic and readable.

```text
exp_<dataset>_<model_family>_<size>_<date>_<index>
```

Examples:

```text
exp_cppe5_yolo11n_320_20260616_001
exp_desk_yolo11n_640_20260616_002
exp_desk_yolo11s_aug_640_20260616_003
```

## Required Metadata

Every experiment must capture:

```text
experiment_id
training_job_id
dataset_id
dataset_version_id
model_family
base_model
image_size
epochs
batch_size
learning_rate
augmentation profile
seed
trainer implementation
started_at
finished_at
status
artifact_uri
export_uris
metrics summary
qualitative sample predictions
notes
```

If the existing model registry can store the metadata in `metrics_summary` or related fields at first, that is acceptable. A dedicated experiment table can come later once the required metadata stabilizes.

## Experiment Matrix

### E0 - Baseline Detector

Purpose: establish baseline quality with the existing pretrained detector.

```text
model: pretrained YOLO
training: none
input: evaluation split
output: inference metrics and qualitative overlays
```

Success signal:

```text
baseline detections run through the same inference and overlay path as fine-tuned models
```

### E1 - Tiny Pipeline Sanity

Purpose: prove the training worker can fine-tune or train on a tiny dataset version quickly.

```text
image_size: 320
batch_size: 1 or 2
epochs: 1-2
trainer: tiny_torch or yolo fine-tune smoke
```

Success signal:

```text
job succeeds, metrics persist, model registers, inference runs
```

### E2 - First Domain Fine-Tune

Purpose: fine-tune on a bounded desk objects dataset.

```text
image_size: 640
epochs: 20-50
batch_size: hardware-dependent
augmentation: conservative
classes: 3-5 stable classes first
```

Success signal:

```text
mAP50 improves over baseline for selected classes
qualitative webcam/captured images look better than baseline
```

### E3 - Augmentation Sweep

Purpose: determine whether augmentation helps camera robustness.

Compare:

```text
no_aug
light_aug
medium_aug
```

Possible augmentation dimensions:

```text
brightness
contrast
blur
crop
rotation small
perspective small
```

Success signal:

```text
validation metric improves without damaging live camera qualitative predictions
```

### E4 - Input Size Sweep

Purpose: trade off quality and latency for live camera.

Compare:

```text
320
480
640
```

Success signal:

```text
acceptable detection quality at the lowest latency usable by the live camera frontend
```

### E5 - Export Fidelity

Purpose: verify exported models behave close enough to source model.

Compare:

```text
PyTorch inference
ONNX inference
CoreML export artifact validation
```

Success signal:

```text
exported model produces compatible detections and preserves class metadata
```

## Dataset Quality Rules

Before fine-tuning, a dataset version must pass quality checks:

```text
minimum samples per class
no class with zero annotations
valid normalized bboxes
train/validation/test split configured
no duplicate image checksum inside same split
classes snapshot matches annotations
```

Minimum exploratory thresholds:

```text
sanity: 10 images/class
first useful pass: 50 images/class
stronger demo pass: 150+ images/class
```

The project can start smaller, but the experiment result must explicitly state dataset limitations.

## Training Configuration Template

```json
{
  "model_family": "yolo",
  "base_model": "yolo11n",
  "hyperparameters": {
    "epochs": 30,
    "batch_size": 8,
    "image_size": 640,
    "learning_rate": 0.001,
    "seed": 42,
    "augmentation_profile": "light_aug"
  }
}
```

If the current API does not yet expose every field, encode experimental metadata in notes or metrics summary until the contract is promoted.

## Metrics

Primary metrics:

```text
mAP50
mAP50_95
precision
recall
validation loss
latency_ms
model_size_mb
```

Secondary metrics:

```text
per-class mAP50
false positives by class
missed detections by class
export size
export inference smoke success
```

Qualitative metrics:

```text
overlay clarity on webcam captures
stability across lighting
confidence flicker
partial occlusion behavior
small object behavior
```

## Evaluation Dataset

Every experiment should keep a small fixed evaluation set outside training.

Recommended sets:

```text
validation split: used for metrics during training
test split: used for final comparison
live-cam-eval set: 20-50 captured frames from real camera conditions
```

Do not tune repeatedly on the live-cam-eval set without noting it; otherwise the evaluation becomes misleading.

## Artifact Requirements

Each successful experiment should produce:

```text
model artifact
metrics JSON
per-class metrics JSON
sample overlays
training log excerpt
export artifact if requested
model card
experiment summary markdown
```

Recommended local path layout:

```text
.perceptionlab/experiments/<experiment_id>/
  metrics.json
  class_metrics.json
  overlays/
  model-card.md
  notes.md
```

## Model Card Template

```markdown
# Model Card - <model_name>

## Dataset
- dataset_id:
- dataset_version_id:
- sample_count:
- annotation_count:
- classes:

## Training
- base_model:
- image_size:
- epochs:
- batch_size:
- learning_rate:
- augmentation_profile:
- seed:

## Metrics
- mAP50:
- mAP50_95:
- precision:
- recall:

## Qualitative Notes
- strengths:
- weaknesses:
- failure cases:

## Deployment Notes
- PyTorch artifact:
- ONNX artifact:
- CoreML artifact:
- expected live camera latency:
```

## Experiment Lifecycle

```text
planned
  -> dataset_ready
  -> training_queued
  -> training_running
  -> training_succeeded
  -> evaluated
  -> exported
  -> promoted_for_demo

failure states:
  dataset_rejected
  training_failed
  evaluation_failed
  export_failed
```

## Integration With Existing Product

Use existing product flows first:

```text
Dataset ingestion
  -> Dataset version
  -> Training job
  -> Metrics
  -> Model registry
  -> Export
  -> Inference
  -> Overlay
  -> Frontend experiment comparison
```

Do not create a parallel experimental script path that bypasses the API and registry unless it is explicitly labeled as a scratch experiment.

## Fine-Tuning CLI Proposal

Future command shape:

```bash
uv run perception-worker experiment finetune \
  --dataset-version-id <dataset_version_id> \
  --base-model yolo11n \
  --image-size 640 \
  --epochs 30 \
  --batch-size 8 \
  --augmentation-profile light_aug \
  --experiment-name exp_desk_yolo11n_640_001
```

The command should still create or consume a training job. It should not become a hidden alternate product path.

## Frontend Experimental Lab

The frontend experimental lab should initially be read-only.

Views:

```text
Experiment list
Experiment detail
Metric comparison
Qualitative overlay gallery
Export status
Promote-to-demo action later
```

Comparison columns:

```text
experiment_id
dataset_version
base_model
image_size
epochs
augmentation_profile
mAP50
precision
recall
latency_ms
artifact status
```

## Acceptance Criteria

The experimental pass is successful when:

- a baseline model is evaluated
- at least one fine-tuned model is trained through the worker
- metrics are persisted
- model artifact is registered
- inference works through the existing API
- overlays are generated for test images
- at least one export path is validated
- the frontend can compare baseline and fine-tuned results
- a short written experiment summary exists

## Risk Register

### Risk: model quality does not improve

Mitigation: improve dataset quality, reduce class scope, compare qualitative output, and document limitations honestly.

### Risk: training becomes hardware-dependent

Mitigation: keep `fake_training`, `tiny_training`, and real fine-tuning separated. Use tiny passes for CI and real passes for local experiments.

### Risk: experiments bypass product infrastructure

Mitigation: every meaningful experiment must produce a training job, metrics, and model registry entry.

### Risk: live camera latency is too high

Mitigation: lower input size, reduce inference frequency, keep one in-flight request, compare ONNX path, and make latency visible in UI.

## Next Implementation Slice

1. Define frontend experiment read model.
2. Add experiment metadata convention to model/training metrics summary.
3. Create first `experiments/` documentation folder in artifacts.
4. Run baseline detector on a fixed evaluation set.
5. Run first tiny fine-tune sanity pass.
6. Compare baseline vs fine-tune in markdown.
7. Expose comparison in frontend after dashboard shell exists.

## Rule Of The Pass

```text
Exploration is allowed, but every useful result must become traceable.
```

That is the difference between a model experiment and ML infrastructure work.
