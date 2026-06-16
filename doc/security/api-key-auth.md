# API Key Auth

PerceptionLab keeps local development friction low while supporting a protected API surface.

## Configuration

API key auth is controlled by `PERCEPTIONLAB_API_KEY`.

| Value | Behavior |
| --- | --- |
| unset | API routes remain unprotected for local development. |
| blank | API routes remain unprotected for local development. |
| non-blank | `/health` is public and all other routes require `x-api-key`. |

## Local Run

```bash
PERCEPTIONLAB_API_KEY=example-key \
PERCEPTIONLAB_API_ADDR=127.0.0.1:8080 \
cargo run --manifest-path api/Cargo.toml -p perception_api
```

## Requests

Healthcheck stays public:

```bash
curl http://127.0.0.1:8080/health
```

Protected route with key:

```bash
curl -H 'x-api-key: example-key' http://127.0.0.1:8080/datasets
```

Protected route without key returns `401` and error code `missing_api_key`.

Protected route with the wrong key returns `403` and error code `invalid_api_key`.

## Status Codes

| Condition | Status |
| --- | --- |
| `/health` without key | `200` |
| Protected route without key | `401` |
| Protected route with wrong key | `403` |
| Protected route with matching key | Route-specific success or error |

## Design Notes

This is intentionally API-key protection, not full authentication or authorization. It covers P2B security scope without introducing users, sessions, OAuth, RBAC, or multi-tenant ownership.

The middleware lives in `perception_http::auth` and is wired by `perception_api` from environment configuration. This keeps the domain and application layers free from HTTP authentication concerns.
