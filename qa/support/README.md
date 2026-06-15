# BDD Support

Shared test support lives here:

- `world.mjs`
- `api-client.mjs`
- `database.mjs`
- `storage.mjs`
- `waiters.mjs`
- `assertions.mjs`

Avoid vague helper modules.

The final runner is `@cucumber/cucumber`; support files are loaded with `qa/support/**/*.mjs`.
