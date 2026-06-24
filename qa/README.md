# QA Features

This folder contains the Gherkin acceptance suite extracted from the PerceptionLab QA & BDD reference.

Fixtures live in `qa/fixtures/`. The first executable smoke step definitions live under `qa/steps/` and shared smoke support lives under `qa/support/`. Full-stack API, database, storage, and worker steps are still pending.

The final runner decision is documented in `qa/bdd-runner.md`: PerceptionLab uses `@cucumber/cucumber`, with feature files from `qa/features/**/*.feature`, step definitions from `qa/steps/**/*.mjs`, support files from `qa/support/**/*.mjs`, and JSON reports in `qa/reports/cucumber-report.json`.

Current command:

```bash
npm run validate:bdd
npm run bdd:dry-run
npm run bdd:smoke
./scripts/run_bdd.sh
```

`npm run validate:bdd` performs static validation. `npm run bdd:dry-run` verifies Cucumber-JS wiring across the full feature suite and intentionally reports undefined full-stack scenarios until their step definitions are implemented. `npm run bdd:smoke` executes the current stack-free `@smoke` subset.
