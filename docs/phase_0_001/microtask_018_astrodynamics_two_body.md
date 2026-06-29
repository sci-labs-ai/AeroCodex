# Microtask 18 — Astrodynamics v0.001 Two-Body Basics

Status: complete in this session.

## Scope

Microtask 18 reviews the Phase 0.001 `aero-codex-astrodynamics` crate for its first scalar two-body astrodynamics helpers:

- circular-orbit speed;
- circular-orbit period;
- two-body escape velocity;
- elliptic vis-viva speed with positive semi-major axis;
- elliptic specific orbital energy with positive semi-major axis.

This is not an orbit propagator, ephemeris model, finite-burn model, perturbation model, navigation model, mission-design optimizer, flight-readiness artifact, or certification artifact. Hohmann-transfer and sphere-of-influence helpers are present from the imported baseline but are intentionally deferred to Microtask 19 review.

## Rust API reviewed

```text
circular_orbit_speed(mu, radius)
orbital_period_circular(mu, radius)
escape_velocity(mu, radius)
vis_viva_speed(mu, radius, semi_major_axis)
specific_orbital_energy(mu, semi_major_axis)
verification_record(codex_id)
```

## Implementation notes

- The crate remains dependency-light and depends only on `aero-codex-core` and `aero-codex-constants`.
- `mu` must be finite and strictly positive for reviewed helpers.
- `radius` must be finite and strictly positive for circular, escape-speed, and vis-viva helpers.
- `semi_major_axis` must be finite and strictly positive for the elliptic vis-viva and specific-energy helpers.
- `vis_viva_speed` returns an out-of-domain error when the radicand is negative for real elliptic speed.
- Overflow, underflow-to-zero for strictly positive outputs, and nonfinite derived arithmetic return `AeroError::NumericalFailure`.
- Conservative `VerificationRecord::research_required` metadata was added only for the Microtask 18 two-body Codex IDs.
- Hohmann-transfer and sphere-of-influence verification metadata is deliberately not added until Microtask 19 review.

## Validation metadata

Added:

```text
validation/cards/astrodynamics_two_body_basics.yaml
```

Added/updated source-registry seeds:

```text
validation/source_registry/astrodynamics_two_body_basics.yaml
validation/source_registry/nasa_jpl_astrodynamics_parameters.yaml
```

The validation card references the two-body equation source seed; the Rust verification records also retain the NASA/JPL parameter source seed because the representative Earth-orbit tests use seeded body constants.

Both remain:

```text
status: research_required
```

No validation card or source-registry entry was upgraded to `equation_traceable`, `implementation_verified`, `reference_validated`, or `experiment_validated`.

## Source verification gaps

Future source review must still document:

- exact equation source edition or dataset;
- equation numbers, page or section ranges, and symbol definitions;
- central-body gravitational-parameter source, epoch, and convention;
- radius definition and body-centre convention;
- treatment of positive semi-major axis and elliptic-orbit scope;
- representative values for circular speed, period, escape speed, vis-viva, and specific energy;
- numerical tolerances for validation tests.

## Checks completed in this environment

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-astrodynamics` depends only on `aero-codex-core` and `aero-codex-constants`.
- Confirmed required Microtask 18 public function names are present.
- Confirmed astrodynamics two-body Codex/source/verification metadata markers are present.
- Confirmed domain-validation markers are present for `mu`, `radius`, and `semi_major_axis`.
- Confirmed vis-viva negative-radicand handling is present.
- Confirmed finite-output and `AeroError::NumericalFailure` guard markers are present.
- Confirmed the astrodynamics validation card links to an existing source-registry seed.
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

The astrodynamics crate has checked first two-body equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade. Transfer-orbit and celestial-helper review remains deferred to Microtask 19.
