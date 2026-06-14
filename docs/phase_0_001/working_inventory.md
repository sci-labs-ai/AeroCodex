# Phase 0.001 Working Inventory

Generated/updated during Microtask 20 final packaging.

## Current reviewed scope

- Microtasks reviewed in this interactive session: 1-20.
- Latest reviewed crate: `aero-codex-life-support`.
- Latest validation cards reviewed/added: `validation/cards/life_support_closure_fraction.yaml`, `validation/cards/life_support_required_production_area.yaml`, `validation/cards/life_support_buffer_residence_time.yaml`, `validation/cards/life_support_daily_mass_balance.yaml`, and `validation/cards/life_support_bioregenerative_mass_balance.yaml`.
- Latest source-registry seed added: `validation/source_registry/life_support_bioregenerative_mass_balance.yaml`.
- Source-registry and validation-card statuses remain conservative `research_required`.
- Final packaging has been prepared for deployment-agent handoff.

## Repository counts

| Item | Count |
| --- | ---: |
| Cargo manifests | 14 |
| Workspace members including `xtask` | 13 |
| Library crates under `crates/` | 12 |
| Rust source files | 17 |
| Validation card YAML files | 21 |
| Source-registry YAML files | 14 |
| Total repository files | 109 |
| Files in manifest/inventory scope | 107 |

## Workspace crates

- `aero-codex-core`
- `aero-codex-constants`
- `aero-codex-atmosphere`
- `aero-codex-thermo`
- `aero-codex-gas-dynamics`
- `aero-codex-aerodynamics`
- `aero-codex-propulsion`
- `aero-codex-heat-transfer`
- `aero-codex-structures`
- `aero-codex-flight-dynamics`
- `aero-codex-astrodynamics`
- `aero-codex-life-support`
- `xtask`

## Microtask 20 notes

- `aero-codex-life-support` now has reviewed scalar bio-regenerative mass-balance helpers for closure fraction, production area, buffer residence time, crew daily requirement, net daily balance, oxygen balance, carbon-dioxide balance, and water recovery balance.
- Closure-style fractions greater than one are not clipped; they carry warning metadata and `ValidityStatus::BoundaryCase` so downstream callers review accounting boundaries.
- Life-support helpers use checked `AeroResult` returns and report nonfinite derived outputs as `AeroError::NumericalFailure`.
- Life-support trace metadata, validation cards, and source-registry seeds remain `research_required`.
- `file_manifest.md` and `file_inventory.csv` omit themselves from hash accounting to avoid self-referential churn.

## Persistent caveats

- Rust compiler, formatter, Clippy, test, and documentation commands must still be run in a Rust-enabled environment.
- Validation cards and source-registry entries are planning artifacts only.
- No card or source-registry seed has been upgraded beyond `research_required`.
- AeroCodex remains not certified, not flight-ready, not mission-ready, and not operationally approved.
