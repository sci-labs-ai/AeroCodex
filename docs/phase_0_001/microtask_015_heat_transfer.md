# Microtask 15 — Heat Transfer v0.001 Radiation, Convection, and Conduction

## Status

Complete in this session.

## Scope

Microtask 15 reviewed the Phase 0.001 `aero-codex-heat-transfer` crate and hardened the first scalar radiation, convection, and conduction helpers. The work remains preliminary-design heat-transfer bookkeeping only; it does not introduce a CFD solver, thermal-network solver, ablation model, material-property database, native dependency, foreign runtime dependency, or certification evidence.

## Public API reviewed

```rust
stefan_boltzmann_radiative_flux(emissivity, t_hot, t_cold)
convective_heat_flux(h, t_recovery_or_fluid, t_wall)
thermal_resistance_conduction(thickness, conductivity, area)
conduction_heat_rate(delta_t, resistance)
verification_record(codex_id)
```

## Implementation notes

- Added conservative Codex IDs and `VerificationRecord::research_required` metadata for each reviewed heat-transfer helper.
- Added `SOURCE_ID_HEAT_TRANSFER_BASIC_PRIMITIVES` and linked it to a new source-registry seed.
- `stefan_boltzmann_radiative_flux` implements `epsilon*sigma*(T_hot^4 - T_cold^4)` with `0 <= emissivity <= 1` and nonnegative absolute temperatures.
- Radiative flux is signed by the caller-supplied temperature ordering; the helper does not silently reorder temperatures.
- `convective_heat_flux` implements `h*(T_recovery_or_fluid - T_wall)` with nonnegative `h` and nonnegative absolute temperatures.
- `thermal_resistance_conduction` implements one-dimensional `thickness/(conductivity*area)` with `thickness >= 0`, `conductivity > 0`, and `area > 0`.
- `conduction_heat_rate` implements `DeltaT/R`, accepts signed finite `delta_t`, and requires positive thermal resistance.
- Added finite-output checks so overflow or nonfinite derived values return `AeroError::NumericalFailure`.

## Validation artifacts

Added:

```text
validation/cards/heat_transfer_basic_primitives.yaml
validation/source_registry/heat_transfer_basic_primitives.yaml
```

Both remain:

```text
status: research_required
```

No source-registry entry or validation card was upgraded from `research_required`.

## Tests added or confirmed in source

The source-level unit-test scaffold now covers:

- radiative flux zero when temperatures are equal;
- radiative flux increasing with hot-side temperature;
- signed radiative temperature-ordering behavior;
- emissivity, absolute-temperature, and nonfinite-output rejection;
- convective flux sign matching the supplied temperature difference;
- invalid convection inputs and nonfinite-output handling;
- positive conduction resistance for positive thickness;
- zero-thickness resistance boundary behavior;
- invalid conduction-resistance inputs and nonfinite-output handling;
- signed conduction heat rate from signed `delta_t`;
- invalid conduction-heat-rate inputs and nonfinite-output handling;
- heat-transfer verification records remaining `research_required`.

## Source verification gaps

Exact source edition, equation identifiers, radiative sign convention, view-factor convention, convective temperature convention, conduction geometry convention, representative examples, applicability limits, and numerical tolerances remain pending source review.

## Checks performed here

- Parsed Cargo manifests with Python `tomllib`.
- Confirmed `aero-codex-heat-transfer` depends only on `aero-codex-core` and `aero-codex-constants`.
- Confirmed required Microtask 15 public function names are present.
- Confirmed heat-transfer Codex/source metadata markers are present.
- Confirmed domain-validation markers are present.
- Confirmed finite-output and `NumericalFailure` guard markers are present.
- Confirmed the heat-transfer validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency token scans across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.

## Checks not performed here

Rust tooling is unavailable in this environment, so the deployment agent must run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```

## Definition of done

The heat-transfer crate has checked first radiation, convection, and conduction equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.
