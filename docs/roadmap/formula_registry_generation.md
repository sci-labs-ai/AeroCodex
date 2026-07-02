# Formula Registry Generation

RR-017 extends the v0.7.2 research-readiness registry tooling with a read-only consistency check for the generated registry artifacts.

- JSON source of truth: `generated/formula_registry.json`.
- JSON integrity guard: `generated/formula_registry.sha256` must exactly match the JSON bytes as `<sha256>  generated/formula_registry.json` plus a trailing newline.
- Generated Rust output: `generated/rust/formula_registry.rs`.
- JSON generation command: `cargo run -p xtask -- formula-registry generate --out generated/formula_registry.json`.
- Rust generation command: `cargo run -p xtask -- formula-registry generate-rust --out generated/rust/formula_registry.rs`.
- Read-only consistency check: `cargo run -p xtask -- formula-registry check`.
- Check behavior: regenerates deterministic JSON from current governed sources into a temporary check file, renders the Rust registry module through the RR-016 generator path into a temporary check file, compares both checked-in generated artifacts byte-for-byte, verifies the JSON SHA sidecar, and performs Formula Registry v1 structural checks.
- Failure repair commands are the JSON and Rust generation commands above; the check command itself does not rewrite checked-in generated artifacts.
- Safety posture: this is a software consistency gate, not formula validation, status promotion, certification, or formula execution.
- Rust SHA sidecar posture: no `generated/rust/formula_registry.sha256` is emitted or required by RR-017; the v0.7.2 deterministic-generation policy requires `.sha256` sidecars for checked-in generated JSON artifacts, not this generated Rust module.
