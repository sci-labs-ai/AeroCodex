# BioSim-plus B2c replay-ledger report assurance

Stage 5 BioSim v3 B2c closes the bounded replay-integrity, ledger, synthetic example, and friend-test report slice for the clean-room BioSim-plus scenario engine. It builds only on deployed B2a/B2b-1/B2b-2 Rust records in the dual `MIT OR Apache-2.0` repository.

## Implemented checks

B2c validates the following before report output is emitted:

1. B2b-2 replay tick count equals the tick-summary count.
2. Tick-summary indexes are contiguous and digest links form an initial-to-final chain.
3. Replay events are ordered by tick and sequence index.
4. Event amounts are finite and nonnegative; requested amount is positive.
5. Event units match the scenario resource canonical unit.
6. Produce/source events satisfy `after = before + committed`, `committed = requested`, and `clamp = 0`.
7. Consume/sink events satisfy `after = before - committed` and `requested = committed + clamp`.
8. Event chains for the same compartment/resource cell are continuous.
9. Final cells are unique, finite, nonnegative, and unit-consistent.
10. Every touched event cell has a matching final cell.
11. Ledger rows satisfy `final = initial + source - sink` using committed event amounts.

## Friend-test report safety

`format_biosim_scenario_friend_test_report` emits deterministic plain text with stable field names and no local path disclosure. The fixed package example in `crates/aero-codex-life-support/examples/biosim_synthetic_resource_scenario.rs` constructs a visibly synthetic two-compartment oxygen transfer using only public validated constructors and prints the report.

## Governance

Validation status remains `research_required`. The validation card is `validation/cards/validation_biosim_plus_clean_room_scenario_engine.yaml`; the source seed is `validation/source_registry/source_biosim_plus_clean_room_scenario_engine.yaml`.

No source import, external BioSim parity claim, biological-dynamics claim, safety claim, medical claim, operational claim, certification claim, or regulated-use claim is made.
