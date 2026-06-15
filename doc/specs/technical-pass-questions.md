# Technical Pass Questions

The Product Owning document fixes the product direction. The technical pass must answer these implementation questions before feature coding starts:

- Which Rust framework is confirmed for the API?
- What is the storage strategy for local MVP and future object storage?
- What is the PostgreSQL schema and migration strategy?
- Which queue implementation is used for training jobs?
- How does Rust communicate with Python?
- Where do artifacts live?
- How is a dataset version materialized for PyTorch?
- Which object detection base model is used?
- How are metrics persisted?
- How are job statuses transitioned safely?
- Which service owns inference?
- Is ONNX export handled in Python or Rust?
- How is Docker Compose packaged?
- Which tests prove the end-to-end flow works?

## Decisions Started In P0 Bootstrap

- Rust API framework dependency: Axum.
- Rust async runtime: Tokio.
- Rust database dependency: SQLx with PostgreSQL support.
- API documentation dependency: Utoipa.
- Python package manager: uv.
- Worker ML dependencies: CPU PyTorch, torchvision, and Ultralytics through the `ml` extra.
- Local project root: `/home/jerem/vision-lab`.
- Local dataset root: `/home/jerem/vision-lab/datasets`.
- Local storage root: `/home/jerem/vision-lab/.perceptionlab/storage`.
- Local artifact root: `/home/jerem/vision-lab/.perceptionlab/artifacts`.

The remaining technical pass still has to define migrations, concrete storage adapters, queue implementation details, worker contracts, and the first executable HTTP endpoints.
