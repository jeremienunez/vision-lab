# Hugging Face Adapters

Purpose: isolate Hugging Face dataset access from application services.

Rules:

- Read tokens from runtime configuration only.
- Do not log or persist `HF_TOKEN`.
- Convert external rows into worker domain samples before storage.
- Keep tests on injected loaders so CI never needs a real token or network access.
