# BioSim-RS Atomic Transaction Commit Validation

Stage 4 Chunk 6B adds a minimal clean-room atomic transaction commit slice for BioSim-RS resource deltas. It builds only on the Chunk 6A generic resource identities and tick validation in `crates/aero-codex-life-support/src/biosim_resource_tick.rs`.

AeroCodex remains research and preliminary-design software. This slice is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Implemented scope

Chunk 6B introduces:

- `biosim_transaction_commit_codex_id()` for `life_support.biosim_rs.atomic_transaction_commit`;
- `biosim_resource_transaction_clean_room_source_id()` for the Chunk 6B source seed;
- `BioSimResourceQuantity` as a caller-supplied resource balance;
- `BioSimResourceDelta` as one staged resource delta;
- `BioSimResourceTransactionCommit` as the returned post-commit snapshot;
- `commit_biosim_resource_transaction(...)` as the pure clean-room commit helper.

The helper validates:

1. the previous and next ticks through the Chunk 6A consecutive tick gate;
2. a nonempty pre-commit state;
3. unique resource balances;
4. finite nonnegative pre-commit balances;
5. a nonempty delta set;
6. finite deltas with at most one delta per resource kind;
7. every delta references an existing resource balance;
8. each post-commit balance remains finite and nonnegative.

If validation fails, the function returns an error and exposes no committed output. Caller state is passed by immutable slice and is not mutated.

## Clean-room and license boundary

This slice does not import or translate Java BioSim implementation code, Java comments, Java package structure, translated tests, BioSim scenario files, GPL-bound fixtures, or the BioSim-RS bootstrap scaffold crates. The implementation is a generic resource-delta transaction primitive authored inside the existing dual `MIT OR Apache-2.0` life-support crate.

The relevant validation artifacts are:

- validation card: `validation/cards/life_support_biosim_rs_atomic_transaction_commit.yaml`;
- source seed: `validation/source_registry/life_support_biosim_rs_transaction_commit_clean_room.yaml`;
- inherited Chunk 6A card: `validation/cards/life_support_biosim_rs_resource_tick.yaml`;
- inherited Chunk 6A source seed: `validation/source_registry/life_support_biosim_rs_resource_tick_clean_room.yaml`.

The validation status remains `research_required`.

## Explicit non-scope

Chunk 6B itself does not add:

- deterministic module ordering;
- digest generation;
- replay proof;
- persistent resource ledger entries;
- conservation closure or minimal O2-loop accounting;
- no BioSim scenario execution;
- CLI/API smoke tests;
- external BioSim reference validation;
- BioSim archive import;
- GPL-compatible distribution decision beyond this clean-room primitive slice.

Chunk 6C now records the follow-on deterministic ordering, digest, and one-tick replay proof slice in `docs/assurance/biosim_rs_deterministic_replay_validation.md`. Chunk 6D records grouped resource-ledger residual checks and bounded minimal oxygen-loop conservation in `docs/assurance/biosim_rs_resource_ledger_validation.md`. Persistent ledger storage and scenario execution remain future chunks.

## Gate expectations

A complete Chunk 6B closeout should keep evidence outside the repository and include:

- focused test-first red/green evidence for `commit_biosim_resource_transaction(...)`;
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
