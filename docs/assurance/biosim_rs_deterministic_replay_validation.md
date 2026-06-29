# BioSim-RS Deterministic Ordering, Digest, and Replay Proof

Stage 4 Chunk 6C adds a minimal clean-room deterministic ordering and replay-digest slice for BioSim-RS resource transactions. It builds only on the Chunk 6A generic resource identities and tick validation plus the Chunk 6B atomic transaction helper in `crates/aero-codex-life-support/src/biosim_resource_tick.rs`.

AeroCodex remains research and preliminary-design software. This slice is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Implemented scope

Chunk 6C introduces:

- `biosim_deterministic_replay_codex_id()` for `life_support.biosim_rs.deterministic_ordering_digest_replay`;
- `biosim_resource_replay_clean_room_source_id()` for the Chunk 6C source seed;
- `BioSimResourceDigest` as a deterministic digest record with an algorithm string, tick index, and 16-character hexadecimal value;
- `BioSimResourceReplayProof` as the one-tick before/after replay proof record;
- `digest_biosim_resource_state(...)` as the deterministic resource-state digest helper;
- `prove_biosim_resource_replay(...)` as the composed one-tick proof helper.

The helpers validate or inherit validation for:

1. positive-duration tick metadata;
2. consecutive previous-to-next tick transitions;
3. nonempty unique pre-commit resource balances;
4. finite nonnegative pre-commit balances;
5. nonempty finite resource deltas with at most one delta per resource kind;
6. resource deltas that reference existing balances;
7. nonnegative finite post-commit balances before proof output is exposed.

## Deterministic ordering and digest rule

Resource states and resource deltas are canonicalized by static clean-room resource ID before digest generation. This keeps exact replay comparisons stable even when caller slices arrive in different orders and avoids reliance on hash-map iteration order.

The state digest uses the algorithm label `fnv1a64:biosim_resource_state:v1`. The delta digest uses `fnv1a64:biosim_resource_delta:v1`. Both are dependency-free fnv-1a 64-bit digests formatted as 16 hexadecimal characters. They are replay smoke-test evidence only; they are not cryptographic checksums, not persistent ledger keys, and not external validation artifacts.

The before digest uses the previous tick index. The after digest uses the next tick index after the clean-room atomic commit succeeds. If the commit fails, no replay proof is returned and caller-owned state remains unchanged because all inputs are immutable slices.

## Clean-room and license boundary

This slice does not import or translate Java BioSim implementation code, Java comments, Java package structure, translated tests, BioSim scenario files, GPL-bound fixtures, or the BioSim-RS bootstrap scaffold crates. The implementation is a generic deterministic ordering and digest primitive authored inside the existing dual `MIT OR Apache-2.0` life-support crate.

The relevant validation artifacts are:

- validation card: `validation/cards/life_support_biosim_rs_deterministic_ordering_digest_replay.yaml`;
- source seed: `validation/source_registry/life_support_biosim_rs_deterministic_replay_clean_room.yaml`;
- inherited Chunk 6B card: `validation/cards/life_support_biosim_rs_atomic_transaction_commit.yaml`;
- inherited Chunk 6B source seed: `validation/source_registry/life_support_biosim_rs_transaction_commit_clean_room.yaml`;
- inherited Chunk 6A card: `validation/cards/life_support_biosim_rs_resource_tick.yaml`;
- inherited Chunk 6A source seed: `validation/source_registry/life_support_biosim_rs_resource_tick_clean_room.yaml`.

The validation status remains `research_required`.

## Explicit non-scope

Chunk 6C does not add:

- persistent resource ledger entries;
- O2-loop conservation, mass/energy closure validation, or biological dynamics;
- module scheduler execution beyond deterministic ordering of current resource-state and delta inputs;
- BioSim scenario execution;
- CLI/API smoke tests;
- external BioSim reference validation;
- BioSim archive import;
- GPL-compatible distribution decision beyond this clean-room primitive slice.

Those are outside Chunk 6C. Chunk 6D now records grouped resource-ledger residual checks and bounded minimal oxygen-loop conservation in `docs/assurance/biosim_rs_resource_ledger_validation.md`. Persistent ledger storage, scenario execution, and habitat-control behavior remain future chunks.

## Gate expectations

A complete Chunk 6C closeout should keep evidence outside the repository and include:

- focused test-first red/green evidence for `digest_biosim_resource_state(...)` and `prove_biosim_resource_replay(...)`;
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
