# Demo Spec

## GitHub Demo Flow

The demo must be explainable in under two minutes:

1. Start the stack with Docker Compose.
2. Create dataset `desk-objects-v1`.
3. Upload a few images.
4. Add or import YOLO annotations.
5. Launch a PyTorch job.
6. Watch metrics update.
7. Retrieve the trained model.
8. Run inference on a test image.
9. Receive boxes and confidence scores from the API.
10. Generate a visual overlay.

Message: this is not a model demo; this is ML infrastructure.

## Final Demo Target

- Dataset: `desk-objects-v1`.
- Classes: `cup`, `book`, `phone`, `keyboard`, `mouse`.
- Input: desk photo.
- Output: annotated image plus JSON response.
- Optional depth metadata: include `distance_m` when available.

Expected demo artifacts live in `doc/demo/`:

- `input.jpg`
- `output_overlay.jpg`
- `inference_response.json`

## Portfolio Success Criteria

- The repo shows serious Rust API design.
- The repo shows serious PyTorch integration.
- The repo shows a pipeline architecture.
- Technical choices are documented.
- A visual demo is available.
- Curl examples and benchmark data are visible.
- The repo makes a technical interviewer want to ask deeper questions.
