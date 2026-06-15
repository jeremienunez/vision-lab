# Contracts

Public and cross-component contracts live here:

- `openapi.json`
- `training_job.schema.json`
- `inference_response.schema.json`
- `metrics.schema.json`

Contracts must be updated when public JSON or worker payloads change.

OpenAPI can be exported with:

```sh
sh scripts/export_openapi.sh > /tmp/perceptionlab-openapi.json
```

Validate the published contract with:

```sh
npm run validate:openapi
```
