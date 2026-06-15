# BioSim-RS Boundary Placeholder

This directory is a Stage 4 boundary marker for future BioSim-RS work.

It intentionally contains no Rust crates, Java code, GPL-bound source, translated implementation detail, fixtures, generated binaries, or scenario archives. The current AeroCodex core remains dual `MIT OR Apache-2.0`; BioSim-RS implementation promotion is limited to explicitly documented clean-room slices until a GPL-compatible or permissioned path is approved.

Chunk 6A clean-room primitives are implemented in the existing life-support crate, not in this placeholder directory:

- `crates/aero-codex-life-support/src/biosim_resource_tick.rs`
- `docs/assurance/biosim_rs_resource_tick_validation.md`
- `validation/cards/life_support_biosim_rs_resource_tick.yaml`
- `validation/source_registry/life_support_biosim_rs_resource_tick_clean_room.yaml`

Authoritative planning docs:

- `docs/assurance/biosim_rs_license_architecture.md`
- `docs/assurance/biosim_rs_resource_tick_validation.md`
- `docs/source_intake/biosim_rs_source_boundary.md`
- `docs/roadmap/stage4_master_plan.md`
- `docs/source_intake/stage4_source_inventory.md`

Future code in this placeholder directory may only be added after a later chunk records source IDs, license path, validation status, deterministic replay evidence, resource-ledger expectations, and the research/preliminary-design caveat. Chunk 6A does not authorize adding crates or external-source material under `biosim-rs/`.
