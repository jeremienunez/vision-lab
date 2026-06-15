# QA Features

This folder contains the Gherkin acceptance suite extracted from the PerceptionLab QA & BDD reference.

Fixtures live in `qa/fixtures/`. Step definitions and test support will be added under `qa/steps/` and `qa/support/` during the technical QA pass.

The final runner decision is documented in `qa/bdd-runner.md`: PerceptionLab uses `@cucumber/cucumber`, with feature files from `qa/features/**/*.feature`, step definitions from `qa/steps/**/*.mjs`, support files from `qa/support/**/*.mjs`, and JSON reports in `qa/reports/cucumber-report.json`.

Current command:

```bash
npm run validate:bdd
npm run bdd:dry-run
./scripts/run_bdd.sh
```

`npm run validate:bdd` performs static validation. `npm run bdd:dry-run` verifies Cucumber-JS wiring before executable step definitions are filled in.
