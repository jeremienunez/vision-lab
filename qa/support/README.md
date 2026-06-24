# BDD Support

Shared test support lives here.

Current executable subset:

- `world.mjs`
- `api-client.mjs`
- `assertions.mjs`

Future full-stack support:

- `world.mjs`
- `api-client.mjs`
- `database.mjs`
- `storage.mjs`
- `waiters.mjs`
- `assertions.mjs`

Avoid vague helper modules.

The final runner is `@cucumber/cucumber`; support files are loaded with `qa/support/**/*.mjs`.
