# Microtask 19 — Astrodynamics v0.001 Hohmann and Celestial Mechanics Helpers

Status: complete in this session.

## Scope

Microtask 19 reviews the Phase 0.001 `aero-codex-astrodynamics` crate for its first scalar transfer-orbit and celestial-helper equations:

- first Hohmann-transfer impulse magnitude;
- second Hohmann-transfer impulse magnitude;
- total two-impulse Hohmann-transfer delta-v magnitude;
- Hohmann half-transfer-ellipse time of flight;
- scalar sphere-of-influence radius.

This is not an orbit propagator, patched-conics validation package, ephemeris model, finite-burn model, trajectory optimizer, mission-design tool, navigation model, flight-readiness artifact, or certification artifact.

## Rust API reviewed

```text
hohmann_transfer_delta_v1(mu, r1, r2)
hohmann_transfer_delta_v2(mu, r1, r2)
hohmann_transfer_total_delta_v(mu, r1, r2)
hohmann_transfer_time(mu, r1, r2)
sphere_of_influence_radius(primary_distance, secondary_mass, primary_mass)
verification_record(codex_id)
```

## Implementation notes

- The crate remains dependency-light and depends only on `aero-codex-core` and `aero-codex-constants`.
- Hohmann helpers require finite, strictly positive `mu`, `r1`, and `r2`.
- Hohmann helpers assume coplanar circular start/end orbits around the same point-mass central body and impulsive two-burn transfers.
- Hohmann delta-v helpers return burn magnitudes, so transfer direction is not represented by the sign.
- `hohmann_transfer_total_delta_v` returns zero when `r1 == r2`, subject to floating-point roundoff.
- `hohmann_transfer_time` computes `pi*sqrt(a_t^3/mu)` with `a_t = 0.5*(r1 + r2)`.
- `sphere_of_influence_radius` requires positive finite `primary_distance`, `secondary_mass`, and `primary_mass`, and computes `primary_distance*(secondary_mass/primary_mass)^(2/5)`.
- Overflow, underflow-to-zero for strictly positive derived values, and nonfinite derived arithmetic return `AeroError::NumericalFailure`.
- Conservative `VerificationRecord::research_required` metadata now covers the Microtask 19 Hohmann and sphere-of-influence Codex IDs.

## Validation metadata

Added:

```text
validation/cards/astrodynamics_transfer_celestial_basics.yaml
validation/source_registry/astrodynamics_transfer_celestial_basics.yaml
```

Updated:

```text
validation/cards/astrodynamics_two_body_basics.yaml
validation/source_registry/astrodynamics_two_body_basics.yaml
validation/source_registry/nasa_jpl_astrodynamics_parameters.yaml
```

The new card references the transfer/celestial equation source seed. The Rust verification records also retain the two-body and NASA/JPL parameter source seeds because Hohmann transfer and sphere-of-influence validation will need equation references plus parameter conventions.

All remain:

```text
status: research_required
```

No validation card or source-registry entry was upgraded to `equation_traceable`, `implementation_verified`, `reference_validated`, or `experiment_validated`.

## Source verification gaps

Future source review must still document:

- exact Hohmann-transfer source edition;
- equation numbers, page or section ranges, and symbol definitions;
- circular-orbit radius convention and central-body gravitational-parameter convention;
- Hohmann burn sign or magnitude convention;
- same-radius transfer boundary convention;
- exact sphere-of-influence source form and exponent convention;
- mass and primary-distance definitions for sphere-of-influence examples;
- representative transfer and planet/star examples;
- numerical tolerances for validation tests.

## Checks completed in this environment

- Parsed all Cargo manifests with Python `tomllib`.
- Confirmed `aero-codex-astrodynamics` depends only on `aero-codex-core` and `aero-codex-constants`.
- Confirmed required Microtask 19 public function names are present.
- Confirmed Hohmann and sphere-of-influence Codex/source/verification metadata markers are present.
- Confirmed positive-domain validation markers are present for `mu`, `r1`, `r2`, `primary_distance`, `secondary_mass`, and `primary_mass`.
- Confirmed finite-output and `AeroError::NumericalFailure` guard markers are present.
- Confirmed the new astrodynamics transfer/celestial validation card links to an existing source-registry seed.
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

The astrodynamics crate has checked first transfer and celestial helper equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.
