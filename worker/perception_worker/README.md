# perception_worker

Expected dependency direction:

```text
entrypoints -> app -> ports/domain/contracts
adapters    -> ports/domain/contracts
```

PyTorch is only allowed in `adapters/training` and `adapters/inference`.
