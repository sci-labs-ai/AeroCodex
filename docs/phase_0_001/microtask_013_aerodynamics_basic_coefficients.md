# Microtask 13 — Aerodynamics v0.001 Basic Coefficients

Status: complete in this session.

## Scope

Microtask 13 reviews the first basic aerodynamic force and coefficient helper set in `aero-codex-aerodynamics`. The functions remain scalar, dependency-free, preliminary-design helpers.

Reviewed public API:

```rust
dynamic_pressure(rho, velocity)
lift(q, area, cl)
drag(q, area, cd)
lift_coefficient(lift, q, area)
drag_coefficient(drag, q, area)
induced_drag_coefficient(cl, aspect_ratio, oswald_efficiency)
verification_record(codex_id)
```

## Implementation notes

- `dynamic_pressure` implements `q = 0.5*rho*V^2` and rejects negative or nonfinite density and scalar speed.
- `lift` and `drag` implement `q*S*C` force definitions and reject negative or nonfinite dynamic pressure and reference area.
- The force helpers allow finite signed coefficient conventions; Phase 0.001 does not enforce a nonnegative drag-coefficient convention because sign conventions remain pending source review.
- `lift_coefficient` and `drag_coefficient` require positive finite `q` and reference area to avoid zero denominators.
- `induced_drag_coefficient` implements `CL^2/(pi*AR*e)` and rejects nonpositive aspect ratio and nonpositive Oswald efficiency.
- Derived outputs are checked for finite values and report `AeroError::NumericalFailure` on overflow or nonfinite intermediate values.
- No compressibility correction, viscous drag buildup, stall, lift-curve slope, trim, stability-derivative, Reynolds-number, or envelope model is included.

## Traceability status

The implementation exposes conservative `VerificationRecord::research_required` metadata for dynamic pressure, lift, drag, coefficient inverse helpers, and induced drag. The validation card `validation/cards/aerodynamics_basic_coefficients.yaml` and source-registry seed `validation/source_registry/aerodynamics_basic_coefficients.yaml` also remain `research_required`.

No source-registry status, validation-card status, certification status, flight-readiness status, operational-readiness status, or mission-readiness status was upgraded. These helpers are not flight-ready, not certification-ready, and not evidence of vehicle-level aerodynamic validity. Exact source edition, equation/page/table identifiers, sign conventions, reference-area conventions, finite-wing assumptions, reference examples, and tolerances remain pending source review.

## Local checks in this environment

Static checks confirmed the required function names, domain checks, finite-output numerical-failure guards, verification metadata, validation-card linkage, source-registry seed, conservative statuses, and dependency policy. Rust compile/test/doc commands still need to be run in a Rust-equipped environment.
