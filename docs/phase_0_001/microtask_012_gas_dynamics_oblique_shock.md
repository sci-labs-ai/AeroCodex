# Microtask 12 — Gas Dynamics v0.001 Oblique Shock Solver

Status: complete in the current interactive review session.

## Scope

Microtask 12 reviews the first branch-explicit attached oblique-shock helper set in `aero-codex-gas-dynamics`. The functions remain scalar, dependency-free, constant-gamma perfect-gas helpers.

Reviewed public API:

```rust
ShockBranch::{Weak, Strong}
theta_beta_mach_residual(mach, beta, gamma, theta)
oblique_shock_beta(mach, theta, gamma, branch)
oblique_shock_normal_mach(mach, beta)
oblique_shock_downstream_mach(mach, beta, theta, gamma)
```

## Implementation notes

- `ShockBranch` is required by `oblique_shock_beta`; weak/strong branch selection is never guessed silently.
- Primary oblique-shock helpers require `mach > 1`.
- The theta-beta-Mach residual requires `gamma > 1`, `Mach angle < beta < pi/2`, and `0 <= theta < pi/2`.
- The beta solver requires `0 < theta < pi/2` and scans the attached-shock beta interval between Mach angle and 90 degrees, then refines sign-changing brackets by bisection.
- If no attached solution is found, the beta solver returns `AeroError::NumericalFailure` instead of returning `NaN` or silently choosing a branch.
- The downstream Mach helper composes the reviewed normal-shock downstream-normal-Mach relation with oblique-shock geometry and requires `beta > theta`.

## Traceability status

The implementation exposes conservative `VerificationRecord::research_required` metadata for the oblique-shock residual, beta solve, normal Mach component, and downstream Mach relation. The validation card `validation/cards/gasdyn_oblique_shock.yaml` also remains `research_required`.

No source-registry status, validation-card status, certification status, flight-readiness status, operational-readiness status, or mission-readiness status was upgraded. Exact report edition, equation numbers, branch conventions, theta-max conventions, reference examples, and tolerances remain pending source review.

## Local checks in this environment

Static checks confirmed the required function names, branch selector, domain checks, numerical-failure path for missing attached solutions, validation-card linkage, and conservative statuses. Rust compile/test/doc commands still need to be run in a Rust-equipped environment.
