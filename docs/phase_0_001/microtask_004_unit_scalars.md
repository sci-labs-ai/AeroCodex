# Microtask 4 Unit-Safe Scalar Review

Status: complete in this interactive session.

## Scope

Microtask 4 reviewed and refined the first shared engineering scalar wrappers in `aero-codex-core`:

- `Angle`
- `Mach`
- `Gamma`
- `Pressure`
- `Temperature`
- `Density`
- `Length`
- `Area`
- `Mass`
- `Time`
- `Velocity`
- `Acceleration`
- `Force`
- `HeatFlux`

The wrappers are intentionally lightweight Phase 0.001 types. They do not introduce external dependencies, unit-conversion frameworks, procedural macros, or native libraries.

## Canonical storage units

| Type | Canonical storage | Construction rule |
|---|---|---|
| `Angle` | radians | infallible degree/radian constructors for trigonometric use |
| `Mach` | dimensionless | finite `M >= 0` |
| `Gamma` | dimensionless | finite `gamma > 1` |
| `Pressure` | pascals | finite value `>= 0` |
| `Temperature` | kelvin | finite value `>= 0` |
| `Density` | kg/m^3 | finite value `>= 0` |
| `Length` | metres | finite value `>= 0` |
| `Area` | m^2 | finite value `>= 0` |
| `Mass` | kg | finite value `>= 0` |
| `Time` | seconds | finite value `>= 0` |
| `Velocity` | m/s | finite value `>= 0` for Phase 0.001 speed-style use |
| `Acceleration` | m/s^2 | finite value `>= 0` for Phase 0.001 magnitude-style use |
| `Force` | newtons | signed value; checked finite constructor also available |
| `HeatFlux` | W/m^2 | signed value; checked finite constructor also available |

## Implementation notes

- `Angle` stores radians internally and exposes `from_degrees`, `from_radians`, `as_degrees`, `as_radians`, `sin`, `cos`, and `tan`.
- `Mach::new` delegates to the shared nonnegative validation helper, so negative, NaN, and infinite inputs are rejected.
- `Gamma::new` delegates to the shared greater-than validation helper, so `gamma <= 1`, NaN, and infinite inputs are rejected.
- Nonnegative SI wrappers use shared validation and return `AeroResult<Self>`.
- `Force` and `HeatFlux` remain signed because loads and heat fluxes often carry direction/sign conventions. Their original infallible constructors are preserved, and checked finite constructors were added for untrusted input.
- No source-registry or validation-card status was upgraded; this microtask is API hygiene and unit wrapper scoped.

## Tests added or confirmed

The unit tests in `crates/aero-codex-core/src/units.rs` now cover:

- degree/radian round-trip conversion,
- trigonometric helpers using internal radians,
- invalid gamma rejection including NaN,
- negative pressure rejection,
- Mach zero acceptance,
- negative and nonfinite Mach rejection,
- zero acceptance and canonical SI getters for all nonnegative scalar wrappers,
- negative/nonfinite rejection across nonnegative wrappers,
- signed-force and signed-heat-flux preservation plus checked finite constructors.

## Checks run in this environment

- Static scan confirmed all required Microtask 4 scalar type names are present.
- Static scan confirmed required constructor/getter/trigonometric method names are present.
- Static scan confirmed the new unit test markers are present.
- Static scan confirmed `aero-codex-core` still has no external dependencies.
- Static Cargo manifest forbidden-dependency token scan was re-run.

## Checks deferred to the deployment agent

Rust compilation, formatting, clippy, tests, docs, and `xtask` validation were not run because this environment does not provide `cargo` or `rustc`. The deployment agent must run the full command set before merging or deploying.
