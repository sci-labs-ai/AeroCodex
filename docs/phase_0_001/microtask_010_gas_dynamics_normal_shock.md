# Microtask 10 — Gas Dynamics v0.001 Normal Shock

Status: complete in this session by static review and source edit. Rust compile/test execution remains pending in a Rust-enabled environment.

## Scope

Microtask 10 reviews and hardens the direct normal-shock perfect-gas relations in `aero-codex-gas-dynamics`:

- `normal_shock_mach2(mach1, gamma)`
- `normal_shock_pressure_ratio_p2_p1(mach1, gamma)`
- `normal_shock_density_ratio_rho2_rho1(mach1, gamma)`
- `normal_shock_temperature_ratio_t2_t1(mach1, gamma)`
- `normal_shock_total_pressure_ratio_p02_p01(mach1, gamma)`
- `verification_record(codex_id)` for the Microtask 10 normal-shock Codex IDs

The crate still contains later Mach-angle, Prandtl-Meyer, and oblique-shock helpers from the imported baseline, but Microtask 10 does not claim review completion for those later features.

## Phase 0.001 model boundary

The normal-shock helpers are scalar calorically perfect-gas relations. They assume a steady, one-dimensional, stationary, plane normal shock with constant `gamma` and upstream Mach number strictly greater than 1.

The direct relation domains are:

```text
mach1 > 1
gamma > 1
```

The downstream Mach helper checks that its derived result is finite, positive, and subsonic. Static pressure, density, and temperature ratios are checked for positive finite values. The total-pressure ratio is checked as a positive finite pressure-loss ratio no greater than unity, with a tiny roundoff allowance near the sonic boundary.

## Source and validation status

The gas-dynamics crate now exposes conservative `VerificationRecord` metadata for the Microtask 10 normal-shock Codex IDs. Every Microtask 10 gas-dynamics record remains `VerificationStatus::ResearchRequired`.

A validation-planning card was added: `validation/cards/gasdyn_normal_shock.yaml`. The card references `source.gasdynamics.naca_report_1135.research_required`. No source-registry entry or validation card was upgraded from `research_required`.

## Rust changes

Updated:

```text
crates/aero-codex-gas-dynamics/src/lib.rs
```

Key refinements:

- added normal-shock Codex ID constants for downstream Mach, static-pressure ratio, density ratio, static-temperature ratio, and total-pressure ratio;
- added conservative `verification_record(codex_id)` branches for normal-shock Codex IDs;
- documented normal-shock assumptions and direct-relation domains in rustdoc;
- centralized validation for `mach1 > 1` and `gamma > 1`;
- added finite-output and overflow checks for derived normal-shock values;
- expanded tests for representative Mach 2 / gamma 1.4 values, physical direction checks, near-sonic behavior, invalid inputs, nonfinite outputs, and conservative verification metadata.

## Checks not run in this environment

The full Rust toolchain checks remain mandatory for the deployment agent because `cargo`, `rustc`, `rustfmt`, and `clippy-driver` are unavailable here.

## Definition-of-done result

The gas-dynamics crate now has hardened Phase 0.001 direct normal-shock relations, documented validity limits, conservative trace metadata, a validation-planning card, and expanded tests. Source and validation status remain conservative pending source review.
