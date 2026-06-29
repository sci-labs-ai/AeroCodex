# Microtask 8 — Thermodynamics v0.001 Perfect Gas Equations

Status: complete in this session.

## Scope

Microtask 8 reviews and tightens the `aero-codex-thermo` crate for basic Phase 0.001 perfect-gas and calorically perfect-gas helpers.

## Rust API reviewed or refined

- `ideal_gas_density(pressure, gas_constant, temperature)`
- `speed_of_sound(gamma, gas_constant, temperature)`
- `cp_from_gamma_r(gamma, gas_constant)`
- `cv_from_gamma_r(gamma, gas_constant)`
- `gamma_from_cp_cv(cp, cv)`
- `specific_gas_constant_from_molar_mass(molar_mass_kg_per_mol)`
- `verification_record(codex_id)`

## Implementation notes

- Density uses `rho = p/(R*T)` with `pressure >= 0`, `gas_constant > 0`, and `temperature > 0`.
- Speed of sound uses `a = sqrt(gamma*R*T)` with `gamma > 1`, `gas_constant > 0`, and `temperature > 0`.
- Heat-capacity helpers use the constant-gamma relations `cp = gamma*R/(gamma - 1)` and `cv = R/(gamma - 1)`.
- `gamma_from_cp_cv` requires `cp > cv > 0`.
- `specific_gas_constant_from_molar_mass` uses the Phase 0.001 universal gas constant seed and requires positive molar mass in kg/mol.
- Derived values are checked for finite, physically meaningful output; overflow/nonfinite derived values report `AeroError::NumericalFailure`.
- The crate remains dependency-free except for existing AeroCodex workspace crates.

## Validation and source status

Added:

- `validation/cards/thermo_perfect_gas.yaml`

The card remains:

```text
status: research_required
```

It references:

```text
source.thermo.nasa_glenn_cea.research_required
```

The thermodynamics source-registry seed remains `research_required`. Microtask 8 does not wrap NASA CEA, does not add a property database, and does not introduce native or foreign-runtime dependencies.

## Tests represented in source

The crate now includes unit-test scaffolding for:

- sea-level dry-air density from `p/(R*T)` near 1.225 kg/m^3;
- zero-pressure density boundary behavior;
- `cp > cv` and `gamma = cp/cv` round trip;
- `cp - cv = R`;
- standard dry-air speed of sound near 340.294 m/s;
- dry-air specific gas constant from molar mass;
- rejection of invalid pressure, gas constant, temperature, gamma, heat-capacity, and molar-mass inputs;
- numerical-failure handling for nonfinite derived outputs;
- conservative `verification_record` metadata staying at `research_required`.

## Checks completed in this environment

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-thermo` depends only on `aero-codex-core` and `aero-codex-constants`.
- Confirmed all required Microtask 8 public function names are present.
- Confirmed density and speed of sound use strictly positive temperature checks.
- Confirmed `gamma_from_cp_cv` requires `cp > cv`.
- Confirmed molar-mass conversion uses a positive molar-mass check.
- Confirmed `AeroError::NumericalFailure` guards are present for nonfinite derived values.
- Confirmed the thermodynamics validation card links to an existing source-registry seed.
- Confirmed validation cards and source-registry files remain `research_required`.
- Re-ran the static Cargo manifest native-dependency policy scan.
- Performed rough delimiter-balance checks on changed Rust source.

## Checks deferred to a Rust-enabled environment

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

Reason: this environment does not provide `cargo`, `rustc`, `rustfmt`, or `clippy-driver`.

## Definition-of-done result

The thermodynamics crate now has basic checked perfect-gas equations and source-level unit-test scaffolding, with conservative validation metadata and no upgraded source or readiness claim.
