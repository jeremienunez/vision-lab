# Architecture Review Checklist

- Layering: does the dependency follow `http/infra -> app -> domain`?
- Domain: do business concepts have explicit types?
- Handlers: are handlers free of SQL and heavy business orchestration?
- Repositories: is persistence behind a port?
- Transactions: are multi-write operations atomic and short-lived?
- Worker: are payloads Pydantic strict and is `torch` isolated to allowed adapters?
- Contracts: is OpenAPI or JSON schema updated when public JSON changes?
- Files: do names describe a specific responsibility?
- Tests: does the pattern introduced have the expected test level?
- ADR: does the decision need an ADR or exception record?
