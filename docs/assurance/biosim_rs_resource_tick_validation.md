# BioSim-RS Resource Identity and Tick Validation

Stage 4 Chunk 6A adds a deliberately small clean-room implementation slice inside `crates/aero-codex-life-support`. It establishes generic resource identities and tick validation before any transaction, replay, ledger, conservation, controller, or scenario behavior is allowed.

AeroCodex remains research and preliminary-design software. This slice is not certification, operational approval, habitat-safety evidence, medical evidence, or regulated-use evidence.

## Scope added in Chunk 6A

Repository code added:

- `crates/aero-codex-life-support/src/biosim_resource_tick.rs`
- public module export from `crates/aero-codex-life-support/src/lib.rs`

Validation metadata added:

- `validation/source_registry/life_support_biosim_rs_resource_tick_clean_room.yaml`
- `validation/cards/life_support_biosim_rs_resource_tick.yaml`

The public Rust surface is intentionally narrow:

- `BioSimResourceKind`
- `BioSimResourceIdentity`
- `BioSimTick`
- `BioSimTickAdvance`
- `biosim_resource_catalog(...)`
- `biosim_resource_catalog_codex_id(...)`
- `biosim_tick_validation_codex_id(...)`
- `biosim_resource_tick_clean_room_source_id(...)`
- `biosim_resource_identity(...)`
- `validate_biosim_resource_catalog(...)`
- `validate_biosim_tick(...)`
- `validate_biosim_tick_advance(...)`
- `biosim_resource_tick_verification_record(...)`

## Clean-room boundary

The Chunk 6A implementation uses generic resource and tick concepts only. It does not import, translate, or repackage:

- Java BioSim source code;
- GPL-bound BioSim-RS scaffold crates;
- `.biosim` scenario files;
- schema/configuration files from the external archives;
- generated golden fixtures;
- Java package trees, class hierarchies, method bodies, tests, comments, or implementation expressions.

The source-registry seed for this slice is `source.life_support.biosim_rs.resource_tick_clean_room.research_required`. It records the implementation as clean-room and keeps the verification status at `research_required` until future source review and validation evidence are supplied.

## Domain and validation behavior

Resource identity validation checks only catalog shape:

- catalog must contain at least one resource;
- resource identities must be unique;
- canonical IDs and canonical units are static metadata for the generic resource families.

Tick validation checks only local tick syntax and ordering:

- tick duration must be finite and strictly positive;
- tick index `0` is accepted as an initialization boundary case with a warning;
- a tick transition is valid only when `next.index == previous.index + 1`;
- overflow at the previous tick index is rejected.

All returned values carry `EngineeringResult` metadata and `research_required` verification status. Successful helper calls are not evidence of external biological, environmental-control, crew-safety, or mission suitability.

## Explicit non-scope

Chunk 6A does not add:

- atomic transaction commit;
- resource-delta application;
- deterministic module scheduling;
- digest or replay proofs;
- resource-ledger persistence;
- minimal O2-loop conservation;
- CLI/API smoke-test reports;
- no scenario execution;
- BioSim archive import;
- GPL-compatible distribution decision beyond this clean-room primitive slice.

Those items were outside Chunk 6A. Chunk 6B records atomic transaction commit in `docs/assurance/biosim_rs_atomic_transaction_commit_validation.md`; Chunk 6C records deterministic ordering, digest, and one-tick replay proof in `docs/assurance/biosim_rs_deterministic_replay_validation.md`; Chunk 6D records grouped resource-ledger residual checks and bounded minimal oxygen-loop conservation in `docs/assurance/biosim_rs_resource_ledger_validation.md`. Persistent ledger storage and scenario execution remain future chunks.

## Gate expectations

A Chunk 6A closeout must include:

1. focused test-first evidence for the new Rust module;
2. workspace Rust gates;
3. `xtask` validation/source/status/data registry gates;
4. nomenclature/acronym/terminology policy review through the Rust-only public governance gate;
5. staged forbidden-file, local-path, archive, evidence, and secret scans;
6. remote `main` and GitHub Actions proof after merge/push.
