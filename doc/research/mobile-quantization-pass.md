# Mobile Quantization And Compression Pass

## Objective

This pass defines how PerceptionLab turns a trained or fine-tuned vision model into a mobile-deployable model that can run on a phone-class chip with acceptable latency, memory use, model size, and accuracy.

The target is not only to make the artifact smaller. The target is to preserve the product behavior needed by the live camera frontend:

```text
camera frame -> mobile model inference -> detections -> overlay -> stable latency
```

This pass sits after baseline training and before mobile/live deployment. It also feeds back into the experimental fine-tuning pass because the best training-time model is not always the best deployable model.

## Technique Reminder

The technique you were thinking of is most likely **teacher-student distillation**, often combined with **quantization-aware training**.

The idea:

```text
large teacher model
  -> trained/fine-tuned for best accuracy
  -> generates labels, logits, boxes, confidence distributions, or feature targets

smaller student model
  -> trained to imitate the teacher
  -> optimized for mobile size and latency
  -> quantized/compressed for deployment
```

Then the student goes through quantization and export:

```text
student FP32/FP16
  -> post-training quantization or QAT
  -> ONNX/CoreML export
  -> mobile benchmark
  -> promote mobile candidate
```

Use the bigger model for learning quality. Use the smaller model for inference on-device.

## Why This Matters

A phone deployment has different constraints from local training:

- memory pressure
- model artifact size
- battery and thermal limits
- inference latency
- CPU/GPU/Neural Engine compatibility
- camera preview responsiveness
- stable frame cadence

A model that wins offline mAP may lose the product demo if it causes camera inference to stutter.

## Compression Techniques

### 1. FP16 Conversion

Purpose: quick reduction in artifact size and memory bandwidth while usually keeping accuracy close to FP32.

Use when:

- first mobile export pass
- accuracy must stay close to baseline
- quantization risk is unknown

Expected role:

```text
baseline mobile candidate
```

### 2. Post-Training Quantization

Purpose: quantize after training without changing the training loop.

Variants:

```text
weight-only quantization
static activation quantization with calibration
dynamic quantization where supported
```

Use when:

- quick compression experiment
- calibration dataset exists
- model quality loss is acceptable

Expected role:

```text
fastest first compression experiment
```

### 3. Quantization-Aware Training

Purpose: simulate quantization effects during training or fine-tuning so the model adapts before final conversion.

Use when:

- PTQ accuracy drop is too high
- mobile target needs INT8 behavior
- final model must preserve detection accuracy under low precision

Expected role:

```text
high-confidence mobile candidate
```

### 4. Palettization

Purpose: compress model weights through lookup tables, especially useful for reducing model size.

Use when:

- model artifact size is a primary constraint
- Core ML deployment is a target
- accuracy-size tradeoff needs exploration

Expected role:

```text
size-focused CoreML candidate
```

### 5. Pruning

Purpose: remove low-importance weights and produce sparse model representations.

Use when:

- model has redundant weights
- fine-tuning can recover quality
- sparse export path is supported by deployment target

Expected role:

```text
secondary compression pass after baseline quantization
```

### 6. Distillation

Purpose: transfer quality from a large teacher into a smaller student.

Use when:

- the model that learns well is too large for phone inference
- the target needs a small architecture
- annotated data is limited but teacher predictions are strong

Expected role:

```text
best long-term route for mobile-quality tradeoff
```

## Recommended Strategy

The recommended strategy is progressive:

```text
1. train/fine-tune a strong teacher
2. evaluate teacher on fixed validation and live-camera eval sets
3. train a small student using labels + teacher signals
4. export student FP16
5. run PTQ INT8
6. compare FP16 vs INT8
7. if PTQ drops too much, run QAT
8. export ONNX/CoreML
9. benchmark on phone or phone-like runtime
10. promote the best mobile candidate
```

Do not start with aggressive INT4 or exotic compression. Establish an accurate and measurable FP16/INT8 path first.

## Mobile Artifact Lifecycle

```text
model candidate
  -> compression candidate
  -> exported artifact
  -> mobile benchmarked artifact
  -> promoted mobile artifact
```

Statuses:

```text
planned
compressed
exported
benchmarking
benchmarked
rejected
promoted_mobile
```

## Experiment Matrix

### Q0 - FP32/FP16 Baseline

Purpose: measure the uncompressed or lightly compressed model.

Inputs:

```text
fine-tuned model artifact
fixed test set
live-camera eval set
```

Outputs:

```text
model size
mAP50
mAP50_95
latency
memory estimate
qualitative overlays
```

### Q1 - CoreML FP16 Export

Purpose: create the first mobile-friendly candidate.

Success signal:

```text
artifact exports successfully and inference behavior remains close to FP32
```

### Q2 - Post-Training INT8 Quantization

Purpose: test fast quantization with calibration data.

Calibration data:

```text
representative frames from train/validation and live-camera conditions
```

Success signal:

```text
model size and latency improve while mAP drop stays within threshold
```

### Q3 - Quantization-Aware Fine-Tune

Purpose: recover accuracy lost during PTQ.

Success signal:

```text
QAT INT8 beats PTQ INT8 on validation and live-camera eval
```

### Q4 - Teacher-Student Distillation

Purpose: train a smaller student to imitate a larger teacher.

Teacher:

```text
larger YOLO or stronger fine-tuned detector
```

Student:

```text
small/nano detector selected for mobile latency
```

Signals:

```text
hard labels
teacher boxes
teacher confidence scores
optional feature-level distillation if implemented
```

Success signal:

```text
student beats a directly fine-tuned small model at the same size/latency budget
```

### Q5 - Distilled Student + QAT

Purpose: produce the strongest mobile candidate.

Pipeline:

```text
distilled student FP32/FP16
  -> QAT
  -> INT8 export
  -> mobile benchmark
```

Success signal:

```text
best tradeoff between quality, artifact size, latency, and live camera stability
```

### Q6 - Palettization / Pruning Exploration

Purpose: reduce artifact size further after stable INT8 path exists.

Success signal:

```text
smaller artifact without unacceptable detection degradation
```

## Acceptance Thresholds

Initial thresholds should be explicit but adjustable:

```text
mAP50 drop FP16 vs FP32: <= 1 point
mAP50 drop INT8 PTQ vs FP32: <= 5 points
mAP50 drop INT8 QAT vs FP32: <= 3 points
student latency vs teacher: at least 2x faster
mobile model size: under target app budget
live camera interval: stable at 1 FPS first, then 2-5 FPS if possible
```

For portfolio demo purposes, qualitative live-camera stability is a first-class acceptance criterion.

## Metrics To Track

Accuracy:

```text
mAP50
mAP50_95
precision
recall
per-class mAP50
false positives by class
missed detections by class
```

Deployment:

```text
artifact size MB
load time
cold inference latency
warm inference latency
memory estimate
runtime backend
export format
mobile device/runtime used
```

Live camera:

```text
client round-trip latency
server or on-device latency
frame interval
detection count stability
confidence flicker
thermal/battery notes if measured
```

## Calibration Dataset

PTQ needs representative calibration data.

Recommended calibration set:

```text
100-500 frames
same classes as target demo
mix of lighting conditions
mix of distances and angles
include real webcam/phone frames
exclude exact test images when possible
```

Store under:

```text
.perceptionlab/calibration/<dataset_version_id>/
```

If calibration is too narrow, INT8 may look good in tests but fail in the live camera page.

## Distillation Dataset

Distillation can use:

```text
human-labeled training data
unlabeled camera captures with teacher pseudo-labels
hard negative frames
edge cases from live-camera failures
```

Pseudo-label rules:

```text
keep teacher detections above threshold
store teacher confidence
store teacher model id
separate pseudo-labels from human labels
allow later human correction
```

## Product Integration

New product concepts can start as metadata before becoming tables.

First metadata fields:

```text
compression_method
precision
teacher_model_id
student_model_family
calibration_dataset_version_id
quantization_mode
export_runtime
mobile_benchmark_summary
```

Later first-class entities:

```text
CompressionJob
MobileArtifact
MobileBenchmarkRun
DistillationRun
CalibrationDataset
```

## Proposed API Surface Later

Not required for first doc-only pass, but target shape:

```text
POST /models/{model_id}/compress
GET  /models/{model_id}/compression-runs
GET  /compression-runs/{run_id}
POST /compression-runs/{run_id}/exports
POST /models/{model_id}/benchmarks/mobile
GET  /models/{model_id}/benchmarks
```

Initial implementation can remain worker/CLI driven until the workflow stabilizes.

## Worker CLI Proposal

```bash
uv run perception-worker compress ptq \
  --model-id <model_id> \
  --format coreml \
  --precision int8 \
  --calibration-root .perceptionlab/calibration/<dataset_version_id>
```

```bash
uv run perception-worker compress qat \
  --model-id <model_id> \
  --dataset-version-id <dataset_version_id> \
  --epochs 5 \
  --precision int8
```

```bash
uv run perception-worker distill \
  --teacher-model-id <teacher_model_id> \
  --student-family yolo11n \
  --dataset-version-id <dataset_version_id> \
  --pseudo-label-threshold 0.55
```

## Frontend Integration

The frontend should surface mobile readiness in the model detail page and experimental lab.

Views:

```text
model detail -> deployment readiness panel
experiments -> compression comparison table
live camera -> select mobile candidate model
```

Comparison columns:

```text
model_id
method
precision
artifact format
size_mb
mAP50
latency_ms
runtime
status
```

Visual labels:

```text
FP32 baseline
FP16 mobile candidate
INT8 PTQ
INT8 QAT
Distilled student
Promoted mobile
```

## Mobile Target Policy

First target:

```text
CoreML artifact for iPhone-class deployment
```

Secondary target:

```text
ONNX artifact for cross-platform benchmark and fallback
```

Decision rule:

```text
CoreML is the portfolio target for iPhone live camera.
ONNX remains the portable validation target.
```

## Implementation Order

1. Evaluate current best model FP32/FP16.
2. Export CoreML FP16 and verify artifact.
3. Build calibration dataset from validation and live-camera frames.
4. Run PTQ INT8 experiment.
5. Compare FP16 vs INT8 PTQ.
6. If quality drop is high, run QAT fine-tune.
7. Fine-tune or select larger teacher.
8. Train distilled smaller student.
9. Quantize distilled student.
10. Benchmark mobile candidate in live camera context.
11. Promote best mobile artifact.

## Definition Of Done

The quantization pass is done when:

- a baseline FP32/FP16 artifact is measured
- an INT8 PTQ artifact is produced or a clear blocker is documented
- a calibration dataset exists
- at least one mobile export exists
- accuracy drop is measured against baseline
- latency and size are measured
- the live camera frontend can select or display a mobile candidate
- the experiment summary explains whether PTQ, QAT, or distillation is the next best move

## Key Rule

```text
Train big if needed. Deploy small always.
```

The large model is a teacher and research asset. The phone model is a compressed or distilled student with measurable deployment behavior.
