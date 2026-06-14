# Microtask 9 — Gas Dynamics v0.001 Isentropic Flow

Status: complete in this session by static review and source edit. Rust compile/test execution remains pending in a Rust-enabled environment.

## Scope

Microtask 9 reviews and hardens the direct isentropic perfect-gas relations in `aero-codex-gas-dynamics`:

- `temperature_ratio_t0_over_t(mach, gamma)`
- `pressure_ratio_p0_over_p(mach, gamma)`
- `density_ratio_rho0_over_rho(mach, gamma)`
- `area_mach_ratio(mach, gamma)`
- `mass_flow_parameter(mach, gamma)`
- `verification_record(codex_id)` for the Microtask 9 isentropic Codex IDs

The crate still contains later shock, expansion, and oblique-shock helpers from the imported baseline, but Microtask 9 does not claim review completion for those later features.

## Phase 0.001 model boundary

The isentropic helpers are scalar calorically perfect-gas relations. They assume steady, one-dimensional, adiabatic, inviscid, isentropic flow with constant `gamma`.

The direct relation domains are:

```text
Mach >= 0 for T0/T, p0/p, rho0/rho, and mass-flow parameter
Mach > 0 for A/A*
gamma > 1
```

Inverse area-Mach solving is intentionally deferred because it requires explicit subsonic/supersonic branch selection. Computed nonfinite outputs are reported as `AeroError::NumericalFailure` rather than being returned silently.

## Source and validation status

The gas-dynamics crate exposes conservative `VerificationRecord` metadata for the Microtask 9 isentropic Codex IDs. Every Microtask 9 gas-dynamics record remains `VerificationStatus::ResearchRequired`.

A validation-planning card was added: `validation/cards/gasdyn_isentropic_flow.yaml`. The card references `source.gasdynamics.naca_report_1135.research_required`. No source-registry entry or validation card was upgraded from `research_required`.

## Rust changes

Updated:

```text
crates/aero-codex-gas-dynamics/Cargo.toml
crates/aero-codex-gas-dynamics/src/lib.rs
```

Key refinements:

- added a pure-Rust workspace dependency on `aero-codex-constants` for the NACA/source-registry ID constant;
- added isentropic Codex ID constants, including `CODEX_ID_ISENTROPIC_MASS_FLOW_PARAMETER`;
- added `verification_record(codex_id)` for conservative trace metadata;
- documented isentropic assumptions and direct-relation domains in rustdoc;
- kept `area_mach_ratio` direct only and documented inverse branch solving as deferred;
- added finite-output checks for isentropic ratios, area-Mach, and mass-flow parameter;
- expanded tests for Mach-zero ratios, monotonic ratios, sonic area-Mach behavior, representative off-sonic area ratios, mass-flow parameter behavior, invalid inputs, nonfinite outputs, and conservative verification metadata.

## Checks not run in this environment

The full Rust toolchain checks remain mandatory for the deployment agent because `cargo`, `rustc`, `rustfmt`, and `clippy-driver` are unavailable here.

## Definition-of-done result

The gas-dynamics crate now has hardened Phase 0.001 direct isentropic relations, documented validity limits, conservative trace metadata, a validation-planning card, and expanded tests. Source and validation status remain conservative pending source review.
