# BDD Steps

BDD step definitions live here.

Current executable subset:

- `smoke.steps.mjs` - stack-free smoke coverage for health, API key auth, dashboard API key headers, and product fire smoke.

Future full-stack step definitions:

- `dataset.steps.mjs`
- `sample.steps.mjs`
- `training.steps.mjs`
- `inference.steps.mjs`
- `overlay.steps.mjs`

The final runner is `@cucumber/cucumber`; step definitions are loaded with `qa/steps/**/*.mjs`.
