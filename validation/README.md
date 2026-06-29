# AeroCodex Validation Scaffold

This directory contains Phase 0.001 validation-planning artifacts. These files are governance and traceability scaffolding; they are not certification, flight-readiness, mission-readiness, or operational-approval artifacts.

## Layout

- `schema/codex_card.schema.json`: JSON Schema for Codex Cards after YAML-to-JSON conversion by downstream tooling.
- `status_vocabulary.yaml`: machine-readable status vocabulary registry for validation cards, source-registry seeds, and data-governance fields.
- `status_vocabulary.md`: human-facing status vocabulary guidance and forbidden readiness-claim notes.
- `cards/`: validation-planning cards for implemented or planned engineering primitives.
- `source_registry/`: conservative source-research seeds that cards can reference by source ID.
- `equation_inventory.tsv`: machine-readable equation inventory/readiness dashboard for executable research equations, formula-vault metadata candidates, external M07 rows with terminal dispositions, remaining external M07 backlog rows, validation-card-only records, and helper algorithms.

## Verification status ladder

Cards and source-registry entries use the same canonical status strings as the Rust `VerificationStatus` type:

1. `research_required`: source and validation details still need review.
2. `equation_traceable`: exact source edition and equation/table/page identifiers have been reviewed.
3. `implementation_verified`: implementation has been checked against traceable equations and unit tests.
4. `reference_validated`: implementation has been compared against reference values or tables with documented tolerance.
5. `experiment_validated`: implementation has been compared against experimental data with documented applicability.

Phase 0.001 keeps the current cards and source-registry entries at `research_required` unless a later source-review pull request supplies the necessary evidence.

The astrodynamics time/frame/state, bounded elliptic-helper, oracle-record, and TLE-contract foundation card is also `research_required`; its TLE layer is contract-only, and its presence does not imply parser, SGP4, frame-transform, external-oracle, or operational-tracking capability.

Current non-example cards include atmosphere troposphere planning, thermodynamics perfect-gas planning, gas-dynamics isentropic-flow, normal-shock, Mach-angle/Prandtl-Meyer expansion-flow, branch-explicit oblique-shock planning, aerodynamics basic force/coefficient planning, propulsion rocket/nozzle bookkeeping planning, heat-transfer radiation/convection/conduction planning, structures beam/buckling planning, flight-dynamics level-turn/performance planning, astrodynamics two-body and Hohmann/celestial-helper planning, bio-regenerative life-support closure-fraction, required-production-area, buffer-residence-time, crew/daily-balance, optional O2/CO2/water mass-balance planning, BioSim-RS clean-room resource identity/tick validation planning, BioSim-RS clean-room atomic transaction commit planning, BioSim-RS clean-room deterministic ordering/digest/replay planning, BioSim-RS clean-room resource-ledger/minimal oxygen-loop conservation planning, BioSim-RS clean-room CLI/API smoke/friend-test reporting, formula-vault implementation-candidate gate planning, formula-vault M00 angle/unit conversion metadata planning, formula-vault candidate-verifier planning, formula-vault M00 per-candidate manifest/reference-link planning, equation-inventory/readiness-dashboard planning, and formula-vault M00 source-expression/test-vector planning. Their presence does not imply source validation.

## Local scaffold checks

- Stage 5 BioSim v3 B2c records `life_support.biosim_plus.clean_room_scenario_engine` as research_required metadata for clean-room replay-integrity, committed-event ledger accounting, deterministic friend-test reporting, and a fixed synthetic example.


Run these from the repository root in a Rust-enabled environment:

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- verify data-registry
cargo run -p xtask -- verify status-vocabulary
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify equation-inventory
cargo run -p xtask -- dependency-policy
```

The `xtask` checks intentionally avoid YAML/JSON parser dependencies in Phase 0.001. They check required top-level fields, nonempty list sections, known status/category strings, card-to-source-registry ID links, schema markers, formula-vault candidate metadata cross-links, equation-inventory counts/block reasons, data-governance status vocabulary coverage, forbidden readiness markers, and native-dependency policy. They are a scaffold, not a replacement for later schema validation, source review, numerical validation, or certification evidence.

A11 adds `cargo run -p xtask -- verify formula-vault` to the governance wrapper. It verifies metadata-only terminal dispositions and does not read raw M07 or Scilab implementation source.

## Card authoring rules

Every card should include:

- a stable dotted `id`;
- a nonempty `name`;
- a known `category`;
- a conservative `status`;
- a `source` object containing `id` and `status`;
- nonempty `assumptions`, `inputs`, `outputs`, `tests`, and `failure_modes` lists;
- a clear `domain` statement;
- a `notes` field that preserves applicability limits.

Source IDs in cards must match an entry under `source_registry/`. Upgrade a card status only in the same review that documents the supporting source evidence, test evidence, or validation evidence.

## Thin-film BLSS extension cards

The thin-film BLSS package adds five `life_support.thinfilm.*` cards and matching source-registry seeds. These are marked `equation_traceable` because the supplied report, BibTeX file, and Rust implementation preserve explicit equation-to-function-to-source mappings. They are not reference-validated or experiment-validated until numerical reproduction cases and calibration data are added.

Stage 4 Chunk 8A handoff adds validation/source metadata for the formula-vault M00 vector-equation expansion; status remains `research_required` and no readiness claim is made.
