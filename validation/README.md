# AeroCodex Validation Scaffold

This directory contains Phase 0.001 validation-planning artifacts. These files are governance and traceability scaffolding; they are not certification, flight-readiness, mission-readiness, or operational-approval artifacts.

## Layout

- `schema/codex_card.schema.json`: JSON Schema for Codex Cards after YAML-to-JSON conversion by downstream tooling.
- `cards/`: validation-planning cards for implemented or planned engineering primitives.
- `source_registry/`: conservative source-research seeds that cards can reference by source ID.

## Verification status ladder

Cards and source-registry entries use the same canonical status strings as the Rust `VerificationStatus` type:

1. `research_required`: source and validation details still need review.
2. `equation_traceable`: exact source edition and equation/table/page identifiers have been reviewed.
3. `implementation_verified`: implementation has been checked against traceable equations and unit tests.
4. `reference_validated`: implementation has been compared against reference values or tables with documented tolerance.
5. `experiment_validated`: implementation has been compared against experimental data with documented applicability.

Phase 0.001 keeps the current cards and source-registry entries at `research_required` unless a later source-review pull request supplies the necessary evidence.

Current non-example cards include atmosphere troposphere planning, thermodynamics perfect-gas planning, gas-dynamics isentropic-flow, normal-shock, Mach-angle/Prandtl-Meyer expansion-flow, branch-explicit oblique-shock planning, aerodynamics basic force/coefficient planning, propulsion rocket/nozzle bookkeeping planning, heat-transfer radiation/convection/conduction planning, structures beam/buckling planning, flight-dynamics level-turn/performance planning, astrodynamics two-body and Hohmann/celestial-helper planning, and bio-regenerative life-support closure-fraction, required-production-area, buffer-residence-time, crew/daily-balance, and optional O2/CO2/water mass-balance planning. Their presence does not imply source validation.

## Local scaffold checks

Run these from the repository root in a Rust-enabled environment:

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- dependency-policy
```

The `xtask` checks intentionally avoid YAML/JSON parser dependencies in Phase 0.001. They check required top-level fields, nonempty list sections, known status/category strings, card-to-source-registry ID links, schema markers, forbidden readiness markers, and native-dependency policy. They are a scaffold, not a replacement for later schema validation, source review, numerical validation, or certification evidence.

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
