# API Workspace

Rust API code lives under `api/crates/` as separate crates so dependency direction is enforceable.

Expected dependency direction:

```text
perception_domain -> no project crate
perception_app    -> perception_domain
perception_infra  -> perception_app + perception_domain
perception_http   -> perception_app + perception_domain
perception_api    -> perception_http + perception_infra
```

The technical pass will add Cargo workspace files and Rust implementation.
