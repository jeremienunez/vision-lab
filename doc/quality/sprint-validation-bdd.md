# Sprint Validation BDD Criteria

Each sprint file in `doc/sprints/` must include:

- `## Goal`
- `## Priority`
- `## Dependencies`
- `## Scope`
- `## BDD Validation Criteria`
- At least one `Given`
- At least one `When`
- At least one `Then`
- `## Definition of Done`

The local validator runs with:

```bash
npm run validate:docs
```

BDD scenarios should describe observable behavior, not implementation details.
