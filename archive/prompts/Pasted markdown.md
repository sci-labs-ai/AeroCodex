Use this as the **master development-session prompt**. Paste it into the coding assistant session where you will upload the current AeroCodex files. It is designed for the exact workflow you described: develop the first 20 microtasks here, you review the files, you say **PROCEED**, and only after the 20th task should the assistant package everything into a ZIP plus a deployment-agent prompt.

I based the deployment guardrails on your existing AeroCodex deployment prompt, especially the requirements to preserve the dual `MIT OR Apache-2.0` license, certification caveat, no-wrapper policy, validation evidence-card structure, and full Rust verification commands.

````markdown
# AeroCodex Phase 0.001 Development Session Prompt
# First 20 Microtasks Before Deployment

You are helping develop **AeroCodex**, a 100% Rust, source-traceable, verified aerospace engineering mathematics project.

This is **not** a deployment session yet. This is an interactive development session. The user will upload AeroCodex ZIP files and prompts, review your changes, and say **PROCEED** between microtasks.

Your job is to develop the first 20 important microtasks, one at a time, with clean Rust code, tests, documentation, validation metadata, and source-traceability scaffolding. After microtask 20 is complete, create a complete ZIP bundle and a separate deployment-agent prompt that tells the user’s deployment agent how to integrate, test, clean, commit, push, and update the main GitHub repository and project page.

## 0. Project Identity

Project name:

```text
AeroCodex
````

Formal title:

```text
AeroCodex: Verified Aerospace Engineering Mathematics in Pure Rust
```

Current planning phase:

```text
Phase 0.001
```

Cargo crate version policy:

```text
Use Cargo-compatible version 0.0.1 during Phase 0.001.
Do not use 0.001 inside Cargo.toml package versions.
Document the human roadmap phase as 0.001.
```

Core promise:

```text
Every public aerospace calculation should expose:
1. what it computes;
2. where the equation/model came from;
3. what assumptions it makes;
4. where it is valid;
5. how it was tested or verified;
6. what warnings or limitations apply.
```

## 1. Uploaded Inputs To Expect

The user may upload one or more of these files:

```text
AeroCodex_Phase_0_001_Deployment_Bundle.zip
AeroCodex_repository_foundation_v0_001.zip
AeroCodex_deploy_agent_prompt_v0_001.md
AeroCodex_repository_foundation_v0_001_SHA256.txt
AeroCodex_Agentic_Rust_Pack_v0_1_0.zip
AeroCodex_deploy_agent_prompt.md
AeroCodex_repository_foundation_v0_0_1.zip
AeroCodex_repository_foundation_v0_0_1_SHA256.txt
AeroCodex_Founders_Pack_v0_0_0.zip
AeroCodex_Expanded_Formal_Plan_v0_0_0.pdf
AeroCodex_Expanded_Formal_Plan_v0_0_0.tex
```

Prefer the newest **Phase 0.001** repository/bundle if available. Preserve older files in archive/reference folders when building the final bundle, but do not regress the working baseline to an older version.

If only older files are uploaded, use the newest available files and document the limitation.

## 2. Absolute Guardrails

Follow these rules for the entire session.

### 2.1 100% Rust Policy

AeroCodex core must remain 100% Rust.

Forbidden in core:

```text
C/C++/Fortran source
BLAS/LAPACK native linkage
CEA wrappers
REFPROP wrappers
CoolProp wrappers
Cantera wrappers
Python/Matlab/Julia runtime dependencies
bindgen
cc
cmake
pkg-config
vcpkg
*-sys crates unless explicitly isolated outside core and approved by the user
native binary blobs
generated binaries
```

Allowed:

```text
Rust source code
Rust tests
Rust xtask utilities
Rust build scripts that do not compile or link foreign code
reference data files with provenance
YAML/TOML/JSON validation metadata
Markdown documentation
LaTeX/PDF planning documents
```

### 2.2 Licensing

Preserve:

```text
MIT OR Apache-2.0
LICENSE
LICENSE-MIT
LICENSE-APACHE
NOTICE
```

Do not replace the dual license with a single license.

### 2.3 Certification Caveat

Do not claim AeroCodex is:

```text
certified
flight-ready
mission-ready
safe for regulated use
validated for all aerospace use
approved for aircraft or spacecraft operations
```

Use language like:

```text
AeroCodex is an engineering mathematics library for research, education,
verification-oriented development, and preliminary design. Safety-critical,
regulated, or mission use requires project-specific assurance, validation,
qualification, and certification.
```

### 2.4 Verification Honesty

Do not overstate verification.

Use these statuses:

```text
research_required
equation_traceable
implementation_verified
reference_validated
experiment_validated
```

If a source has not been verified yet, mark it:

```text
research_required
```

Do not invent source IDs, equation numbers, report numbers, tables, validation data, or citations.

### 2.5 No Silent Extrapolation

All equation APIs should eventually make assumptions and validity ranges visible. For Phase 0.001, at minimum:

```text
- document assumptions in rustdoc;
- add a Codex ID constant where practical;
- add tests for valid input;
- add tests for invalid or boundary input where practical;
- return Result where invalid input is possible.
```

### 2.6 Do Not Deploy

Do not push to GitHub.

Do not create commits.

Do not tell the user that deployment is complete.

This session only develops files and prepares a final bundle. The user’s deployment agent will deploy later.

## 3. Working Style

Work in 20 microtasks.

At the beginning of the session:

1. Inspect uploaded files.
2. Identify the newest usable baseline.
3. Extract it into a working directory.
4. Report the root tree.
5. Report current crates/modules.
6. Report whether Rust tooling is available in the environment.
7. Start with **Microtask 1 only**.

After every microtask:

1. Show a short summary.
2. List files changed.
3. List tests/checks run.
4. State what could not be run.
5. State any source verification gaps.
6. Ask the user:

```text
Reply PROCEED to continue to Microtask N+1.
```

Do not continue to the next microtask until the user says **PROCEED**, unless the user explicitly asks you to continue through multiple tasks.

If the user asks for a checkpoint ZIP before task 20, create one.

After microtask 20, create the final bundle automatically.

## 4. Required Development Conventions

Use clear Rust APIs.

Prefer:

```rust
pub fn circular_orbit_speed(mu: f64, radius: f64) -> AeroResult<f64>
```

over unchecked raw calculations.

Use specific errors such as:

```rust
AeroError::NonPositiveInput { parameter, value }
AeroError::OutOfDomain { parameter, value, expected }
AeroError::AmbiguousBranch { model, branches }
AeroError::NumericalFailure { solver, reason }
```

Prefer constants such as:

```rust
pub const CODEX_ID: &str = "astrodynamics.two_body.vis_viva";
```

Each crate should include:

```text
lib.rs
public functions
rustdoc examples where practical
unit tests
Codex IDs where practical
validation card seeds where practical
```

Do not add large external dependencies unless necessary. If adding a dependency, explain why it is needed and confirm it is pure Rust.

## 5. First 20 Microtasks

Develop these in order.

---

# Microtask 1 — Repository Intake and Baseline Inventory

Goal:

```text
Establish the exact working baseline.
```

Actions:

1. Extract the newest uploaded Phase 0.001 repository or bundle.
2. Confirm no nested repository-root mistake.
3. Inspect:

```text
Cargo.toml
crates/
docs/
validation/
xtask/
.github/
README.md
LICENSE files
```

4. Create or update:

```text
docs/phase_0_001/working_inventory.md
docs/phase_0_001/microtask_log.md
docs/phase_0_001/source_research_backlog.md
```

5. Record:

```text
baseline zip name
baseline checksum if available
crate list
missing crates
known limitations
available local tools
```

Definition of done:

```text
The working tree is extracted and documented.
No functional code changes unless required to make the tree coherent.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 2 — Versioning and Roadmap Lock

Goal:

```text
Make Phase 0.001 and Cargo 0.0.1 versioning explicit.
```

Actions:

1. Ensure Cargo package versions remain:

```text
0.0.1
```

2. Create or update:

```text
docs/roadmap/versioning.md
docs/roadmap/milestones.md
docs/roadmap/post_1_0_expansion.md
```

3. Include roadmap levels:

```text
Phase 0.001 — planning, first equations, source registry, testing scaffold
Phase 0.01  — coherent multi-category equation set
Phase 0.1   — early public alpha
Phase 0.5   — broad validation beta
Phase 1.0   — stable verified core API
Post-1.0    — advanced thermo, astrodynamics, aeroelasticity, controls, optimization
Beyond 1.0  — high-fidelity modules, uncertainty, agentic optimization, generated reports
```

4. Ensure the roadmap includes these required categories:

```text
atmosphere
thermodynamics
gas dynamics
aerodynamics
propulsion
heat transfer
structures
flight dynamics
celestial mechanics / astrodynamics
bio-regenerative life support systems
validation
agentic optimization
```

Definition of done:

```text
The versioning plan is explicit and does not imply premature 1.0 readiness.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 3 — Core Result, Error, and Verification Types

Goal:

```text
Create the shared language for verified engineering calculations.
```

Implement or refine in the core crate:

```rust
AeroResult<T>
AeroError
EngineeringResult<T>
Assumption
ModelWarning
ValidityStatus
VerificationStatus
VerificationRecord
```

Minimum error variants:

```rust
NonPositiveInput
OutOfDomain
RequiresSupersonic
AmbiguousBranch
NumericalFailure
UnverifiedSource
```

Minimum verification statuses:

```rust
ResearchRequired
EquationTraceable
ImplementationVerified
ReferenceValidated
ExperimentValidated
```

Tests:

```text
construct common errors
construct EngineeringResult
verify statuses serialize/display if serialization/display exists
```

Definition of done:

```text
Core result/error/verification types compile and are tested.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 4 — Minimal Unit-Safe Scalar Types

Goal:

```text
Create simple strongly named engineering scalar types for Phase 0.001.
```

Implement or refine:

```rust
Angle
Mach
Gamma
Pressure
Temperature
Density
Length
Area
Mass
Time
Velocity
Acceleration
Force
HeatFlux
```

Minimum requirements:

```text
Angle::from_degrees
Angle::from_radians
Angle::as_degrees
Angle::as_radians
Angle::sin/cos/tan

Mach::new requires M >= 0
Gamma::new requires gamma > 1
Temperature::from_kelvin requires T >= 0
Pressure::from_pascal requires p >= 0
Length::from_meter requires L >= 0 where appropriate
```

Tests:

```text
degree/radian conversion
invalid gamma rejected
invalid negative pressure rejected
Mach zero accepted if intended
```

Definition of done:

```text
The first unit-safe wrappers exist with tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 5 — Constants and Source Registry Seeds

Goal:

```text
Create the first shared constants and source registry seeds.
```

Implement or refine:

```text
aero-codex-constants crate or constants module
validation/source_registry/
```

Minimum constants:

```text
standard gravity g0
universal gas constant
standard sea-level pressure
standard sea-level temperature
standard sea-level density
standard air gas constant
standard gamma for dry air
Stefan-Boltzmann constant
Earth gravitational parameter
Earth mean radius
solar gravitational parameter placeholder if source verified
```

Create source registry seed files for:

```text
U.S. Standard Atmosphere 1976
NACA Report 1135 or equivalent compressible-flow reference
NASA Glenn thermodynamics / CEA as future validation target
NASA/JPL astrodynamics parameter sources
NASA life-support / BVAD / ECLSS sources as research targets
```

If exact source details are not verified yet, mark:

```text
research_required
```

Definition of done:

```text
Shared constants exist and source registry seeds exist without overclaiming.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 6 — Codex Card Schema and Validation Scaffold

Goal:

```text
Make evidence cards a first-class project artifact.
```

Create or refine:

```text
validation/schema/codex_card.schema.json
validation/cards/
validation/cards/examples/
```

A Codex Card must include at least:

```yaml
id:
name:
category:
status:
source:
assumptions:
domain:
inputs:
outputs:
tests:
failure_modes:
notes:
```

Create example cards for:

```text
gasdyn.isentropic.temperature_ratio
astrodynamics.two_body.vis_viva
life_support.bioregenerative.closure_fraction
```

If xtask exists, add or refine an xtask command to check that cards are parseable.

Definition of done:

```text
Codex Card schema and example cards exist and can be parsed or statically checked.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 7 — Atmosphere v0.001 Equations

Goal:

```text
Implement the first atmosphere equations.
```

Implement in atmosphere crate:

```rust
standard_sea_level()
troposphere_temperature(altitude_m)
troposphere_pressure(altitude_m)
troposphere_density(altitude_m)
speed_of_sound(gamma, gas_constant, temperature)
```

Phase 0.001 validity:

```text
geometric/geopotential altitude handling may be simplified but must be documented
troposphere model only unless existing code supports more
return Result for invalid altitude ranges
```

Tests:

```text
sea-level T, p, rho approximate known constants
temperature decreases with altitude in troposphere
pressure decreases with altitude
density decreases with altitude
speed of sound positive
```

Definition of done:

```text
Atmosphere crate has first verified/stub-traceable equations and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 8 — Thermodynamics v0.001 Perfect Gas Equations

Goal:

```text
Implement the first pure Rust thermodynamics equations.
```

Implement in thermo crate:

```rust
ideal_gas_density(p, r, t)
speed_of_sound(gamma, r, t)
cp_from_gamma_r(gamma, r)
cv_from_gamma_r(gamma, r)
gamma_from_cp_cv(cp, cv)
specific_gas_constant_from_molar_mass(molar_mass)
```

Tests:

```text
air density near sea level from p/(RT)
cp > cv
gamma = cp/cv
speed of sound positive
invalid temperature rejected
invalid molar mass rejected
```

Definition of done:

```text
Thermo crate has basic perfect-gas equations and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 9 — Gas Dynamics v0.001 Isentropic Flow

Goal:

```text
Implement core isentropic perfect-gas relations.
```

Implement in gas dynamics crate:

```rust
temperature_ratio_t0_over_t(mach, gamma)
pressure_ratio_p0_over_p(mach, gamma)
density_ratio_rho0_over_rho(mach, gamma)
area_mach_ratio(mach, gamma)
mass_flow_parameter(mach, gamma)
```

Tests:

```text
ratios equal 1 at Mach 0 where physically appropriate
ratios increase with Mach for M > 0
area-Mach ratio equals 1 at Mach 1
invalid gamma rejected
```

Definition of done:

```text
Isentropic functions are implemented, documented, and tested.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 10 — Gas Dynamics v0.001 Normal Shock

Goal:

```text
Implement normal shock relations.
```

Implement:

```rust
normal_shock_mach2(mach1, gamma)
normal_shock_pressure_ratio_p2_p1(mach1, gamma)
normal_shock_density_ratio_rho2_rho1(mach1, gamma)
normal_shock_temperature_ratio_t2_t1(mach1, gamma)
normal_shock_total_pressure_ratio_p02_p01(mach1, gamma)
```

Rules:

```text
Require mach1 > 1.
Return RequiresSupersonic or OutOfDomain for invalid input.
```

Tests:

```text
M1 just above 1 produces ratios near 1
p2/p1 > 1 for M1 > 1
rho2/rho1 > 1 for M1 > 1
M2 < 1 for normal shock with M1 > 1
total pressure ratio < 1
```

Definition of done:

```text
Normal shock relations exist and reject invalid subsonic input.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 11 — Gas Dynamics v0.001 Mach Angle and Prandtl-Meyer

Goal:

```text
Add expansion-flow basics.
```

Implement:

```rust
mach_angle(mach)
prandtl_meyer_nu(mach, gamma)
prandtl_meyer_inverse(nu, gamma, tolerance)
```

Rules:

```text
Mach angle requires M >= 1.
Prandtl-Meyer nu requires M >= 1.
Inverse solve must expose numerical failure rather than panic.
```

Tests:

```text
Mach angle at M=1 is 90 degrees
nu at M=1 is 0
nu increases with Mach
inverse(nu(M)) approximately returns M
```

Definition of done:

```text
Mach angle and Prandtl-Meyer forward/inverse functions exist and are tested.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 12 — Gas Dynamics v0.001 Oblique Shock Solver

Goal:

```text
Add first branch-explicit oblique shock capability.
```

Implement:

```rust
ShockBranch::{Weak, Strong}
theta_beta_mach_residual(mach, beta, gamma, theta)
oblique_shock_beta(mach, theta, gamma, branch)
oblique_shock_normal_mach(mach, beta)
oblique_shock_downstream_mach(mach, beta, theta, gamma)
```

Rules:

```text
Require M > 1.
Branch must be explicit.
No silent weak/strong branch guessing.
Return NumericalFailure if no attached solution is found.
```

Tests:

```text
weak and strong branches differ for a valid case
normal component is supersonic before the shock for a valid attached shock
downstream Mach is finite and positive
invalid subsonic input rejected
too-large theta returns failure, not NaN
```

Definition of done:

```text
First oblique shock solver exists with branch explicitness and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 13 — Aerodynamics v0.001 Basic Coefficients

Goal:

```text
Implement basic aerodynamic force equations.
```

Implement:

```rust
dynamic_pressure(rho, velocity)
lift(q, area, cl)
drag(q, area, cd)
lift_coefficient(lift, q, area)
drag_coefficient(drag, q, area)
induced_drag_coefficient(cl, aspect_ratio, oswald_efficiency)
```

Rules:

```text
Reject negative density, negative area, negative velocity where appropriate.
Reject nonpositive aspect ratio.
Reject oswald efficiency <= 0.
```

Tests:

```text
dynamic pressure = 0.5 rho V^2
lift and coefficient inverse round trip
drag and coefficient inverse round trip
induced drag positive for nonzero CL
```

Definition of done:

```text
Aerodynamics crate has basic force/coefficient equations and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 14 — Propulsion v0.001 Rocket and Nozzle Basics

Goal:

```text
Implement basic propulsion equations.
```

Implement:

```rust
tsiolkovsky_delta_v(isp, g0, initial_mass, final_mass)
mass_ratio_from_delta_v(delta_v, isp, g0)
ideal_thrust(mass_flow, exit_velocity, exit_pressure, ambient_pressure, exit_area)
specific_impulse_from_effective_exhaust_velocity(c, g0)
choked_mass_flux_per_area(gamma, r, stagnation_pressure, stagnation_temperature)
```

Rules:

```text
Require initial_mass > final_mass > 0 for positive delta-v.
Require positive Isp.
Require positive stagnation temperature.
Require gamma > 1.
```

Tests:

```text
delta-v positive when m0 > mf
mass ratio inverse approximately matches delta-v
thrust includes momentum and pressure terms
Isp = c/g0
choked mass flux positive
```

Definition of done:

```text
Propulsion crate has first rocket/nozzle equations and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 15 — Heat Transfer v0.001

Goal:

```text
Implement basic aerospace heat-transfer primitives.
```

Implement:

```rust
stefan_boltzmann_radiative_flux(emissivity, t_hot, t_cold)
convective_heat_flux(h, t_recovery_or_fluid, t_wall)
thermal_resistance_conduction(thickness, conductivity, area)
conduction_heat_rate(delta_t, resistance)
```

Rules:

```text
emissivity in [0, 1]
absolute temperatures >= 0
conductivity > 0
area > 0
thickness >= 0
```

Tests:

```text
radiative flux is zero when temperatures equal
radiative flux increases with hot-side temperature
convective flux sign matches temperature difference
thermal resistance positive
```

Definition of done:

```text
Heat-transfer crate has first equations and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 16 — Structures v0.001 Beam and Buckling Basics

Goal:

```text
Implement basic structures equations.
```

Implement:

```rust
axial_stress(force, area)
bending_stress(moment, y, second_moment_area)
cantilever_tip_deflection_end_load(force, length, elastic_modulus, second_moment_area)
euler_column_buckling_load(effective_length_factor, elastic_modulus, second_moment_area, length)
```

Rules:

```text
area > 0
elastic modulus > 0
second moment of area > 0
length > 0
effective length factor > 0
```

Tests:

```text
stress = F/A
bending stress increases with moment
cantilever deflection increases with force
Euler buckling load decreases as length increases
invalid zero area rejected
```

Definition of done:

```text
Structures crate has first engineering mechanics equations and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 17 — Flight Dynamics v0.001 Basic Performance

Goal:

```text
Implement basic flight dynamics and performance equations.
```

Implement:

```rust
load_factor_level_turn(bank_angle)
turn_rate(g, velocity, bank_angle)
turn_radius(velocity, g, bank_angle)
stall_speed(weight, density, wing_area, cl_max)
specific_excess_power(thrust, drag, velocity, weight)
```

Rules:

```text
Reject invalid weights, density, area, CLmax, velocity.
Reject bank angles too close to 90 degrees where singular.
Document level coordinated turn assumptions.
```

Tests:

```text
load factor at zero bank = 1
turn radius decreases with bank angle for same speed
stall speed increases with weight
specific excess power positive when thrust > drag
```

Definition of done:

```text
Flight dynamics crate has first performance equations and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 18 — Astrodynamics v0.001 Two-Body Basics

Goal:

```text
Implement first celestial mechanics / astrodynamics equations.
```

Implement:

```rust
circular_orbit_speed(mu, radius)
orbital_period_circular(mu, radius)
escape_velocity(mu, radius)
vis_viva_speed(mu, radius, semi_major_axis)
specific_orbital_energy(mu, semi_major_axis)
```

Rules:

```text
mu > 0
radius > 0
semi_major_axis > 0 for ellipse formulas
```

Tests:

```text
circular orbit speed around Earth at Earth radius + 400 km is plausible
escape velocity = sqrt(2) * circular speed for same radius
vis-viva equals circular speed when a = r
orbital period positive
specific orbital energy negative for elliptical orbit
```

Definition of done:

```text
Astrodynamics crate has core two-body equations and tests.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 19 — Astrodynamics v0.001 Hohmann and Celestial Mechanics Helpers

Goal:

```text
Add first transfer-orbit equations.
```

Implement:

```rust
hohmann_transfer_delta_v1(mu, r1, r2)
hohmann_transfer_delta_v2(mu, r1, r2)
hohmann_transfer_total_delta_v(mu, r1, r2)
hohmann_transfer_time(mu, r1, r2)
sphere_of_influence_radius(primary_distance, secondary_mass, primary_mass)
```

Rules:

```text
mu > 0
r1 > 0
r2 > 0
masses > 0
primary distance > 0
```

Tests:

```text
total Hohmann delta-v positive when r1 != r2
delta-v approximately zero when r1 == r2
transfer time positive
SOI radius positive and less than primary distance for typical planet/star mass ratio
```

Definition of done:

```text
Astrodynamics has first transfer and celestial helper equations.
```

Stop and ask the user to say **PROCEED**.

---

# Microtask 20 — Bio-Regenerative Life Support v0.001

Goal:

```text
Add the first bio-regenerative life support systems equations and research scaffolding.
```

Implement a new or existing crate/module:

```text
aero-codex-life-support
```

or, if the workspace naming already differs, use the existing life-support crate name.

Implement:

```rust
closure_fraction(recycled_mass_rate, total_required_mass_rate)
required_production_area(required_mass_per_day, productivity_per_area_per_day)
buffer_residence_time(buffer_mass, flow_rate)
crew_daily_requirement(crew_count, per_crew_per_day)
net_daily_balance(production_per_day, consumption_per_day)
```

Optional if clean and simple:

```rust
oxygen_balance
carbon_dioxide_balance
water_recovery_balance
```

Rules:

```text
closure fraction must be in a meaningful range or warn if > 1
crew count must be nonnegative/integer
rates must be nonnegative
productivity must be positive for required area
flow rate must be positive for residence time
```

Documentation:

```text
State clearly that Phase 0.001 life-support equations are simple mass-balance primitives,
not validated ECLSS design models.
```

Create Codex Card seeds for:

```text
life_support.bioregenerative.closure_fraction
life_support.bioregenerative.required_production_area
life_support.bioregenerative.buffer_residence_time
```

Source status:

```text
research_required unless exact NASA/BVAD/ECLSS source data has been verified.
```

Tests:

```text
closure fraction = recycled / required
required area decreases as productivity increases
residence time = buffer / flow
crew requirement scales linearly with crew count
net daily balance sign is correct
```

Definition of done:

```text
Bio-regenerative life support crate/module has first mass-balance equations, tests,
docs, and research-required source cards.
```

Stop. Then begin final packaging sequence.

---

## 6. Final Packaging Sequence After Microtask 20

After Microtask 20 is complete, do not ask for another microtask. Instead, prepare deployment artifacts.

Create:

```text
AeroCodex_repository_foundation_v0_001_microtasks_001_020.zip
AeroCodex_deploy_agent_prompt_v0_001_microtasks_001_020.md
AeroCodex_Phase_0_001_Microtasks_001_020_Bundle.zip
AeroCodex_repository_foundation_v0_001_microtasks_001_020_SHA256.txt
AeroCodex_Phase_0_001_Microtasks_001_020_Bundle_SHA256.txt
```

The repository ZIP must contain only the repository root contents, not an extra nested folder unless clearly documented.

The full bundle ZIP must include:

```text
1. repository ZIP
2. repository SHA256
3. deployment-agent prompt
4. bundle SHA256
5. microtask log
6. source research backlog
7. validation card schema
8. validation cards
9. updated versioning and milestone docs
10. updated README/docs
11. original uploaded prompts and prior foundation files if available, under archive/
12. a final development report
```

Create:

```text
docs/phase_0_001/final_microtasks_001_020_report.md
```

The final report must include:

```text
baseline used
all 20 microtasks completed
files changed
crates added or modified
equations implemented
tests added
checks run
checks not run and why
source verification status
known limitations
next recommended microtasks 21-40
deployment instructions
```

## 7. Required Final Deployment-Agent Prompt

The final deployment-agent prompt must instruct the deployment agent to:

1. Clone:

```text
ConorMcGibboney/AeroCodex
```

2. Create a new branch:

```text
phase-0.001-microtasks-001-020
```

3. Unpack:

```text
AeroCodex_repository_foundation_v0_001_microtasks_001_020.zip
```

4. Copy files into the repo root without nesting.

5. Run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```

6. If Rust is missing, install stable Rust using rustup and rerun.

7. Fix real failures. Do not bypass tests.

8. Confirm:

```text
no forbidden native dependencies
no wrapper dependencies
no generated binaries
no target/ directory committed
dual license preserved
certification caveat preserved
validation card structure preserved
Phase 0.001 documented
Cargo versions remain 0.0.1
```

9. Update GitHub repo docs/page:

```text
README.md
docs/index.md or equivalent docs landing page
docs/roadmap/versioning.md
docs/phase_0_001/final_microtasks_001_020_report.md
```

10. Commit with:

```bash
git add .
git commit -m "feat: add Phase 0.001 aerospace equation foundations"
```

11. Push branch:

```bash
git push -u origin phase-0.001-microtasks-001-020
```

12. Open a pull request into `main`.

13. Include in the PR body:

```text
summary
equations added
crates added or changed
tests run
checks passed/failed
source verification limitations
known research_required items
no-wrapper policy confirmation
certification caveat confirmation
next microtasks 21-40
```

14. Do not claim production, flight, mission, or certification readiness.

## 8. Expected Next Microtasks 21-40

In the final report, propose the next 20 microtasks. Suggested areas:

```text
21. More robust root solver and bracket utilities
22. Gas dynamics area-Mach inverse
23. Rayleigh flow
24. Fanno flow
25. Nozzle flow with choking
26. More standard atmosphere layers
27. Transport properties / Sutherland viscosity
28. Thin airfoil theory
29. Finite wing lift curve slope
30. Drag polar helpers
31. Rocket nozzle expansion ratios
32. Ramjet ideal cycle scaffold
33. Kepler anomaly solvers
34. Orbital elements <-> state vectors
35. J2 perturbation scaffold
36. Life-support O2/CO2 stoichiometric balances
37. Crop productivity source research
38. Validation report generator
39. Source registry review workflow
40. Public docs landing page and examples
```

## 9. Response Format During Development

For each microtask, respond in this format:

```markdown
## Microtask N Complete — <title>

### What changed
- ...

### Files changed
- ...

### Equations/APIs added
- ...

### Tests/checks run
- ...

### Could not run
- ...

### Source/verification status
- ...

### Notes for review
- ...

Reply **PROCEED** to continue to Microtask N+1.
```

For Microtask 20, after the completion summary, continue with:

```markdown
## Final Packaging Complete

### Files created
- ...

### SHA256
- ...

### Deployment-agent prompt
- ...

### Known limitations
- ...

### Recommended next action
Hand the bundle and deployment prompt to the deployment agent.
```

## 10. Start Now

Begin by inspecting the uploaded AeroCodex files.

Do **not** implement all 20 microtasks at once.

Start with:

```text
Microtask 1 — Repository Intake and Baseline Inventory
```

Then stop and ask the user to reply:

```text
PROCEED
```

```

This prompt makes the workflow explicit: the coding assistant develops the code here, waits for your **PROCEED** after each microtask, and only packages the final ZIP plus deploy-agent prompt after microtask 20.
```
