# Formula Registry Generation

RR-016 records the generated Rust registry module decision for the v0.7.2 research-readiness plan.

- JSON source of truth: `generated/formula_registry.json`.
- JSON integrity guard: `generated/formula_registry.sha256` must match before Rust generation starts.
- Generated Rust output: `generated/rust/formula_registry.rs`.
- Command: `cargo run -p xtask -- formula-registry generate-rust --out generated/rust/formula_registry.rs`.
- Check-in posture: the Rust registry module is a deterministic checked-in generated artifact for later CLI/library lookup tasks.
- Build posture: RR-016 does not wire product CLI builds, product CLI behavior, runtime execution, registry stale-check commands, or a registry crate boundary.
- Rust SHA sidecar posture: no `generated/rust/formula_registry.sha256` is emitted in RR-016 because the v0.7.2 deterministic-generation policy requires `.sha256` sidecars for checked-in generated JSON artifacts; it does not clearly require a sidecar for this generated Rust module.
