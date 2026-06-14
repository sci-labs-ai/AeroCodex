# Microtask 11 — Gas Dynamics v0.001 Mach Angle and Prandtl-Meyer

Status: complete in this session by static review and source edit. Rust compile/test execution remains pending in a Rust-enabled environment.

## Scope

Microtask 11 reviews and hardens the Mach-angle and Prandtl-Meyer perfect-gas helpers in `aero-codex-gas-dynamics`:

- `mach_angle(mach)`
- `prandtl_meyer_nu(mach, gamma)`
- `prandtl_meyer_inverse(nu, gamma, tolerance)`
- `verification_record(codex_id)` for the Microtask 11 Mach-angle and Prandtl-Meyer Codex IDs

The crate still contains oblique-shock helpers from the imported baseline, but Microtask 11 does not claim review completion for those functions.

## Phase 0.001 model boundary

The reviewed Mach-angle and Prandtl-Meyer expansion-flow helpers are scalar calorically perfect-gas relations. They assume a constant `gamma` where applicable and use radians internally through the shared `Angle` wrapper.

The direct relation domains are:

```text
mach >= 1

gamma > 1 for Prandtl-Meyer helpers
```

The inverse Prandtl-Meyer helper accepts:

```text
0 <= nu < nu_max(gamma)
tolerance > 0
```

The sonic boundary is explicit: `mach_angle(1)` returns 90 degrees, `prandtl_meyer_nu(1, gamma)` returns zero, and `prandtl_meyer_inverse(Angle::ZERO, gamma, tolerance)` returns Mach 1.

## Source and validation status

The gas-dynamics crate now exposes conservative `VerificationRecord` metadata for the Microtask 11 Mach-angle and Prandtl-Meyer Codex IDs. Every Microtask 11 gas-dynamics record remains `VerificationStatus::ResearchRequired`.

A validation-planning card was added: `validation/cards/gasdyn_mach_angle_prandtl_meyer.yaml`. The card references `source.gasdynamics.naca_report_1135.research_required`. No source-registry entry or validation card was upgraded from `research_required`.

## Rust changes

Updated:

```text
crates/aero-codex-gas-dynamics/src/lib.rs
```

Key refinements:

- added Codex ID constants for Mach angle, Prandtl-Meyer nu, and Prandtl-Meyer inverse;
- added conservative `verification_record(codex_id)` branches for those Codex IDs;
- documented sonic-boundary behavior and inverse-domain limits in rustdoc;
- added finite-result checks for Mach-angle and Prandtl-Meyer derived terms;
- added explicit inverse bracketing and bisection failure reporting through `AeroError::NumericalFailure`;
- expanded tests for Mach-angle representative values, Prandtl-Meyer representative values, inverse round trips, invalid domains, bracketing failure, and conservative verification metadata.

## Checks not run in this environment

The full Rust toolchain checks remain mandatory for the deployment agent because `cargo`, `rustc`, `rustfmt`, and `clippy-driver` are unavailable here.

## Definition-of-done result

The gas-dynamics crate now has hardened Phase 0.001 Mach-angle and Prandtl-Meyer helpers, documented validity limits, conservative trace metadata, a validation-planning card, and expanded tests. Source and validation status remain conservative pending source review. Oblique-shock helpers remain pending Microtask 12 review.
