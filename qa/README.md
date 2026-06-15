# QA Features

This folder contains the Gherkin acceptance suite extracted from the PerceptionLab QA & BDD reference.

Fixtures live in `qa/fixtures/`. Step definitions and test support will be added under `qa/steps/` and `qa/support/` during the technical QA pass.

Current command:

```bash
npm run validate:bdd
./scripts/run_bdd.sh
```

The command performs static validation until the technical QA pass chooses the final runner and step-definition stack.
