# Inference Latency Benchmark

## Goal

Track local MVP inference latency against the product requirement: small-model CPU inference should stay under 500 ms.

## Command

Start the API, make sure a model id exists, then run:

```sh
npm run benchmark:inference -- --model-id <model_id> --iterations 10
```

The default image is:

```text
datasets/seed/images/desk-objects.png
```

Override inputs when needed:

```sh
npm run benchmark:inference -- \
  --base-url http://127.0.0.1:8080 \
  --model-id <model_id> \
  --image datasets/seed/images/desk-objects.png \
  --iterations 25 \
  --confidence-threshold 0.25
```

## Output

The script prints JSON with:

- `client_latency_ms`: measured wall-clock HTTP latency.
- `api_latency_ms`: server-reported inference latency from the API response.
- `min`, `max`, `avg`, and `p95` for each latency group.

## Current MVP Baseline

The fake local inference adapter returns deterministic detections and currently reports `latency_ms: 9` in route tests. Real PyTorch or ONNX inference must be benchmarked with this same command before portfolio polish is considered complete.
