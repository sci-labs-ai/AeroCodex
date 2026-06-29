# Microtask 14 — Propulsion v0.001 Rocket and Nozzle Basics

## Status

Complete in this session.

## Scope

Microtask 14 reviewed the Phase 0.001 `aero-codex-propulsion` crate and hardened the first rocket/nozzle scalar helpers. The work remains preliminary-design bookkeeping only; it does not introduce a propulsion-performance engine model, CEA wrapper, combustion solver, native dependency, foreign runtime dependency, or certification evidence.

## Public API reviewed

```rust
tsiolkovsky_delta_v(isp, g0, initial_mass, final_mass)
mass_ratio_from_delta_v(delta_v, isp, g0)
ideal_thrust(mass_flow, exit_velocity, exit_pressure, ambient_pressure, exit_area)
specific_impulse_from_effective_exhaust_velocity(c, g0)
choked_mass_flux_per_area(gamma, r, stagnation_pressure, stagnation_temperature)
verification_record(codex_id)
```

## Implementation notes

- Added conservative Codex IDs and `VerificationRecord::research_required` metadata for each reviewed propulsion helper.
- Added `SOURCE_ID_PROPULSION_ROCKET_NOZZLE_BASICS` and linked it to a new source-registry seed.
- Preserved explicit caller-supplied `g0`; the helper does not silently substitute a standard-gravity constant.
- `tsiolkovsky_delta_v` requires `initial_mass > final_mass > 0` and positive finite `isp` and `g0` for positive ideal delta-v.
- `mass_ratio_from_delta_v` accepts zero delta-v as the boundary case returning mass ratio 1 and rejects negative delta-v.
- `ideal_thrust` includes both momentum thrust and pressure thrust; pressure thrust may be negative when ambient pressure exceeds exit pressure.
- `choked_mass_flux_per_area` requires `gamma > 1`, positive gas constant, nonnegative stagnation pressure, and positive stagnation temperature.
- Added finite-output checks so overflow or nonfinite derived values return `AeroError::NumericalFailure`.

## Validation artifacts

Added:

```text
validation/cards/propulsion_rocket_nozzle_basics.yaml
validation/source_registry/propulsion_rocket_nozzle_basics.yaml
```

Both remain:

```text
status: research_required
```

No source-registry entry or validation card was upgraded from `research_required`.

## Tests added or confirmed in source

The source-level unit-test scaffold now covers:

- delta-v positive when `initial_mass > final_mass`;
- mass-ratio inverse round trip against delta-v;
- invalid mass, impulse, and delta-v domains;
- ideal thrust momentum plus pressure terms;
- negative pressure-thrust contribution while inputs remain nonnegative;
- `Isp = c/g0`;
- positive choked mass flux for valid stagnation conditions;
- invalid gamma, gas constant, stagnation pressure, and stagnation temperature;
- nonfinite derived-output `NumericalFailure` cases;
- propulsion verification records remaining `research_required`.

## Source verification gaps

Exact source edition, equation identifiers, standard-gravity convention, pressure-thrust sign convention, stagnation-state notation, representative examples, applicability limits, and numerical tolerances remain pending source review.

## Checks performed here

- Parsed Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-propulsion` depends only on `aero-codex-core`.
- Confirmed required Microtask 14 public function names are present.
- Confirmed propulsion Codex/source metadata markers are present.
- Confirmed domain-validation markers are present.
- Confirmed finite-output and `NumericalFailure` guard markers are present.
- Confirmed the propulsion validation card links to an existing source-registry seed.
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

The propulsion crate has checked first rocket/nozzle equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.
