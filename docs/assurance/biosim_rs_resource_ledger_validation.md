# BioSim-RS Resource Ledger and Minimal Oxygen-Loop Conservation Validation

Stage 4 Chunk 6D adds a minimal clean-room resource-ledger accounting slice for BioSim-RS planning. It builds on the Chunk 6A resource/tick primitives, the Chunk 6B atomic commit primitive, and the Chunk 6C deterministic ordering/replay evidence in `crates/aero-codex-life-support/src/biosim_resource_tick.rs`.

AeroCodex remains research and preliminary-design software. This slice is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Implemented scope

Chunk 6D introduces:

- `biosim_resource_ledger_codex_id()` for `life_support.biosim_rs.resource_ledger_minimal_o2_loop_conservation`;
- `biosim_resource_ledger_clean_room_source_id()` for the Chunk 6D source seed;
- `BioSimResourceUnitKey` for deterministic ledger grouping by resource kind and canonical unit;
- `BioSimResourceLedgerStore` for caller-supplied before/after store snapshots;
- `BioSimResourceLedgerAccounting` for caller-accounted source and sink terms;
- `BioSimResourceLedgerRow` and `BioSimResourceLedgerReport` for residual evidence;
- `BioSimMinimalO2LoopConservationProof` for the bounded two-store oxygen transfer helper;
- `validate_biosim_resource_ledger_tick(...)` for one-tick grouped residual checks;
- `prove_biosim_minimal_o2_loop_conservation(...)` for a bounded minimal oxygen-loop conservation proof.

The ledger residual convention is:

```text
residual = (after_total - before_total) - (accounted_source + accounted_sink)
```

A row passes when the absolute residual is less than or equal to the caller-declared absolute tolerance in the resource canonical unit.

## Clean-room and license boundary

This slice does not import or translate Java BioSim implementation code, Java comments, Java package structure, translated tests, BioSim scenario files, GPL-bound fixtures, external golden-master outputs, or the BioSim-RS bootstrap scaffold crates. The implementation is a generic resource-ledger accounting primitive authored inside the existing dual `MIT OR Apache-2.0` life-support crate.

The relevant validation artifacts are:

- validation card: `validation/cards/life_support_biosim_rs_resource_ledger_minimal_o2_loop_conservation.yaml`;
- source seed: `validation/source_registry/life_support_biosim_rs_resource_ledger_clean_room.yaml`;
- inherited Chunk 6C card: `validation/cards/life_support_biosim_rs_deterministic_ordering_digest_replay.yaml`;
- inherited Chunk 6C source seed: `validation/source_registry/life_support_biosim_rs_deterministic_replay_clean_room.yaml`.

The validation status remains `research_required`.

## Explicit non-scope

Chunk 6D itself does not add:

- persistent resource-ledger storage;
- append-only audit logging;
- no BioSim scenario execution;
- persistent BioSim run/replay command surfaces;
- external BioSim reference validation;
- no biological dynamics;
- no habitat-control behavior;
- crew-safety, medical, operational, certification, or regulated-use approval;
- BioSim archive import;
- GPL-compatible distribution decision beyond this clean-room primitive slice.

## Gate expectations

A complete Chunk 6D closeout should keep evidence outside the repository and include:

- focused test-first red/green evidence for resource-ledger accounting and minimal oxygen-loop conservation;
- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --workspace --all-features`;
- `cargo test -p aero-codex-life-support biosim_resource_tick -- --nocapture`;
- `cargo run -p xtask -- verify --all`;
- `cargo run -p xtask -- verify data-registry`;
- `cargo run -p xtask -- verify status-vocabulary`;
- `cargo run -p xtask -- dependency-policy`;
- `cargo doc --workspace --all-features --no-deps`;
- Rust-only public governance checks using `cargo run -p xtask -- verify --all`;
- safety scans proving no GPL source, archives, fixtures, generated binaries, evidence logs, `target/`, or root `Cargo.lock` are staged.

Chunk 6E adds a separate static CLI/API smoke and friend-test report wrapper; it does not change the Chunk 6D scope or promote scenario execution.
