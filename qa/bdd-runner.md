# BDD Runner Decision

## Final Choice

- Runner: `@cucumber/cucumber`.
- Feature glob: `qa/features/**/*.feature`.
- Step definitions: `qa/steps/**/*.mjs`.
- Shared support: `qa/support/**/*.mjs`.
- JSON report: `qa/reports/cucumber-report.json`.

## Rationale

Cucumber-JS is the official Cucumber implementation for Node.js and fits the repository's existing Node-based policy and validation tooling. It keeps Gherkin feature files readable for product review while step definitions can use the same local API clients, seed dataset, and benchmark scripts already maintained under Node.

## Commands

Static feature validation remains part of the normal quality gate:

```sh
npm run validate:bdd
```

The selected executable runner command is:

```sh
npm run bdd:dry-run
```

Full scenario execution will be enabled once `qa/steps/**/*.mjs` and `qa/support/**/*.mjs` contain the API, database, storage, and worker step definitions.
