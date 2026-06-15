# Product Spec

## Vision

PerceptionLab proves that a computer vision model does not live in an isolated notebook. A useful ML system needs infrastructure around the model: data ingestion, annotation storage, dataset versioning, async jobs, metrics, model registry, export, monitoring signals, and an inference API.

The product has two goals:

- Build a useful local platform for creating and training custom computer vision models.
- Produce a GitHub portfolio repo that demonstrates Rust backend, PyTorch, ML pipeline, API design, data engineering, MLOps, and deployment thinking.

## Product Promise

```text
Upload data -> build dataset -> launch training -> track metrics -> export model -> run inference
```

Portfolio promise:

```text
I can build production-grade infrastructure around machine learning models.
```

## Problem

Many AI repositories stay superficial: notebook, pretrained model, API wrapper, or thin UI over an external service. PerceptionLab solves the deeper systems problem between raw visual data and a usable production model.

- Datasets are often unversioned and scattered.
- Training jobs are often not traceable to data, hyperparameters, and metrics.
- Generated models often lack registry, promotion, export, and artifact tracking.
- Inference is often disconnected from training.
- Computer vision demos often hide backend and infrastructure competence.

## Positioning

PerceptionLab is a ML infrastructure project applied to computer vision. The core value is not model novelty; the value is the industrialization around the model.

It is not:

- A simple YOLO app.
- An isolated PyTorch notebook.
- A full Roboflow clone.
- A mobile detection app.
- A generic dashboard.

It is:

- A Rust backend for ingestion and orchestration.
- A Python/PyTorch training worker.
- A dataset versioning system.
- A model registry.
- An inference API.
- An ONNX/CoreML export pipeline.
- A final object detection demo with optional depth metadata.

## Target Users

- ML Builder: wants to fine-tune a detection model from custom images with traceability.
- Backend/Infra Engineer: wants to inspect how ML fits into robust backend architecture.
- Technical Recruiter or Hiring Manager: wants to see quickly that the repo is not a thin AI wrapper.
- Project Owner: uses the repo as proof of competence for interviews, freelance missions, and technical discussions.

## Value Proposition

- User value: transform raw visual data into trained, versioned, exportable models available via API.
- Technical value: demonstrate a complete Rust + PyTorch architecture for a computer vision pipeline.
- Portfolio value: prove the ability to build infrastructure around a model, not just call an existing model.

## MVP Scope

Included:

- Dataset creation.
- Image upload.
- File storage in MinIO or local filesystem compatible with the storage abstraction.
- Metadata storage in PostgreSQL.
- Object detection annotations.
- Dataset versions.
- Training jobs and queue.
- Python/PyTorch worker.
- Fine-tuning of a detection model.
- Checkpoint and metrics persistence.
- Minimal model registry.
- Inference endpoint.
- ONNX export.
- Complete documentation.
- Docker Compose local stack.

Out of scope for MVP:

- Advanced annotation UI like CVAT.
- Complex multi-user flows.
- Payment.
- Full OAuth.
- Distributed multi-GPU training.
- AutoML.
- Full real-time video support.
- Mandatory Prometheus/Grafana production monitoring.
- Full DVC-style dataset versioning.

## Success Criteria

The MVP succeeds when someone can clone the repo, launch the stack, create a dataset, upload images, add or import annotations, launch a training job, produce a model, run inference, generate an overlay, and understand the value in under ten minutes.
