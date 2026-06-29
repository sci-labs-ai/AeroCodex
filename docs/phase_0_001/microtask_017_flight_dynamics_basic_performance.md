# Microtask 17 — Flight Dynamics v0.001 Level Turn and Performance Basics

Status: complete in this session.

## Scope

Microtask 17 reviews the Phase 0.001 `aero-codex-flight-dynamics` crate. The scope is limited to scalar, preliminary-design flight-dynamics/performance helpers:

- level coordinated-turn load factor;
- level coordinated-turn rate;
- level coordinated-turn radius magnitude;
- lift-equals-weight stall-speed estimate;
- specific excess power bookkeeping.

This is not a trim model, stability model, flight-control model, maneuver-envelope model, aircraft-performance database, flight-test validation artifact, certification artifact, or mission-performance model.

## Rust API reviewed

```text
load_factor_level_turn(bank_angle)
turn_rate(g, velocity, bank_angle)
turn_radius(velocity, g, bank_angle)
stall_speed(weight, density, wing_area, cl_max)
specific_excess_power(thrust, drag, velocity, weight)
verification_record(codex_id)
```

## Implementation notes

- The crate remains dependency-light and depends only on `aero-codex-core`.
- `Angle` from `aero-codex-core` is used for bank-angle inputs.
- Level coordinated-turn helpers require finite bank angle strictly inside plus or minus 90 degrees.
- `turn_rate` preserves bank-angle sign and returns radians per second.
- `turn_radius` returns a positive radius magnitude and rejects zero bank angle.
- `stall_speed` requires positive weight, density, wing area, and maximum lift coefficient.
- `specific_excess_power` accepts finite signed thrust and drag bookkeeping scalars, nonnegative velocity, and positive weight.
- Overflow or nonfinite derived arithmetic returns `AeroError::NumericalFailure`.
- All reviewed helper metadata remains `VerificationStatus::ResearchRequired`.

## Validation metadata

Added:

```text
validation/cards/flight_dynamics_basic_performance.yaml
validation/source_registry/flight_dynamics_basic_performance.yaml
```

Both remain:

```text
status: research_required
```

No validation card or source-registry entry was upgraded to `equation_traceable`, `implementation_verified`, `reference_validated`, or `experiment_validated`.

## Source verification gaps

Future source review must still document:

- exact source edition or publication;
- equation numbers, page ranges, and symbol definitions;
- bank-angle and turn-rate sign conventions;
- whether turn radius is documented as a positive magnitude or signed curvature;
- stall-speed configuration and reference-area conventions;
- specific-excess-power thrust/drag station definitions;
- representative source examples and numerical tolerances.

## Checks completed in this environment

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-flight-dynamics` depends only on `aero-codex-core`.
- Confirmed required Microtask 17 public function names are present.
- Confirmed flight-dynamics Codex/source/verification metadata markers are present.
- Confirmed domain-validation markers are present for bank angle, gravity, velocity, stall-speed inputs, thrust, drag, and weight.
- Confirmed finite-output and `AeroError::NumericalFailure` guard markers are present.
- Confirmed the flight-dynamics validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency-token scan across Cargo manifests.
- Ran static forbidden readiness-marker scan across validation files.
- Ran rough delimiter-balance checks on changed Rust source.

## Checks not run here

This environment does not provide Rust tooling, so the deployment agent must run:

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

## Definition-of-done result

The flight-dynamics crate has checked first level-turn, stall-speed, and specific-excess-power helpers, source-test scaffolding, conservative validation metadata, and no source-status upgrade.
