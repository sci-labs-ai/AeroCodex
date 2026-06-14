# Source Research Backlog

All source entries below are intentionally conservative. Microtasks 1-20 did not upgrade any source-verification status; entries remain `research_required` until exact publication identifiers, equation numbers, table numbers, page ranges, data provenance, and licensing constraints are reviewed.

## Intake-level source status

| Area | Current status | Session action so far |
|---|---|---|
| Atmosphere | `research_required` | Microtask 7 hardened the simplified troposphere equations and added a validation-planning card, but no source claim was upgraded. |
| Thermodynamics / propulsion | `research_required` | Microtask 8 hardened perfect-gas helpers and added a conservative thermodynamics card; Microtask 14 hardened ideal rocket/nozzle helpers and added a conservative propulsion card/source seed; no source claim upgraded. |
| Gas dynamics | `research_required` | Microtasks 9-12 hardened direct isentropic, normal-shock, Mach-angle/Prandtl-Meyer expansion-flow, and branch-explicit oblique-shock relations and added conservative gas-dynamics cards; no source claim upgraded. |
| Aerodynamics / flight dynamics | `research_required` | Microtask 13 hardened basic aerodynamic force/coefficient helpers and added a conservative aerodynamics card/source seed; Microtask 17 hardened level-turn, stall-speed, and specific-excess-power helpers and added a conservative flight-dynamics card/source seed; no source claim upgraded. |
| Structures / heat transfer | `research_required` | Microtask 15 hardened scalar heat-transfer helpers and added a conservative heat-transfer card/source seed; Microtask 16 hardened elementary structures helpers and added a conservative structures card/source seed; no source claim upgraded. |
| Astrodynamics | `research_required` | Microtask 18 reviewed two-body scalar helpers; Microtask 19 reviewed Hohmann-transfer and sphere-of-influence helpers. Source editions, parameter definitions, equation IDs, and reference values remain pending. |
| Bio-regenerative life support | `research_required` | Microtask 20 hardened scalar closure, production-area, buffer, crew-requirement, oxygen, carbon-dioxide, and water-balance helpers and added a conservative life-support card/source seed; no source claim upgraded. |
| Validation workflow | `research_required` | Schema/cards inventoried; full Rust `xtask` validation not run because Rust tooling is unavailable here. Microtask 3 added dependency-free status parsing/display helpers only; Microtask 4 reviewed scalar wrappers only; Microtask 5 refined constants/source-registry seeds only; Microtask 6 tightened validation schema/scaffold checks only; Microtask 7 hardened atmosphere equations and added a conservative atmosphere card only; Microtasks 9-12 hardened gas-dynamics scaffolding and added conservative gas-dynamics cards only; Microtask 13 hardened basic aerodynamic coefficient bookkeeping and added a conservative aerodynamics card/source seed only; Microtask 14 hardened rocket/nozzle bookkeeping and added a conservative propulsion card/source seed only; Microtask 15 hardened heat-transfer primitives and added a conservative heat-transfer card/source seed only; Microtask 16 hardened elementary structures helpers and added a conservative structures card/source seed only; Microtask 17 hardened flight-dynamics basic-performance helpers and added a conservative flight-dynamics card/source seed only; Microtask 18 hardened astrodynamics two-body helpers and added a conservative astrodynamics card/source seed only; Microtask 19 hardened Hohmann-transfer and sphere-of-influence helpers and added a conservative astrodynamics transfer/celestial card/source seed only; Microtask 20 hardened life-support mass-balance helpers and added conservative life-support card coverage only. |

## Constants

- NIST/CODATA physical constants: exact edition, value, uncertainty/exactness status, units, citation requirements, and redistribution/licensing notes for the universal gas constant and Stefan-Boltzmann constant.
- U.S. Standard Atmosphere 1976 constants: exact sea-level values, g0, dry-air gas constant convention, and acceptable tolerances for derived density checks.
- NASA/JPL astrodynamics parameters: exact source, epoch, body/radius definitions, and uncertainty conventions for Earth constants and any future solar gravitational-parameter use.
- Keep `SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER` explicitly unverified until source review justifies a non-placeholder value.

## Atmosphere

- U.S. Standard Atmosphere 1976: exact tables, layers, constants, geopotential altitude definitions, and acceptable derived constant tolerances.
- Confirm whether Phase 0.001 functions use geometric or geopotential altitude and document the simplification explicitly.

## Gas dynamics

- NACA Report 1135 or equivalent compressible-flow reference: exact equations and tables for isentropic flow, normal shocks, oblique shocks, Mach angle, and Prandtl-Meyer expansions.
- Identify Mach-angle and Prandtl-Meyer equation/table references, maximum-angle conventions, inverse-Mach examples, and acceptable tolerances for verification-card examples.
- Identify oblique-shock theta-beta-Mach equation/table references, weak/strong branch conventions, detachment-angle conventions, downstream-Mach examples, and acceptable tolerances for verification-card examples.

## Thermodynamics / propulsion

- NASA Glenn educational thermodynamics pages and NASA CEA documentation as future validation targets only; do not wrap CEA or introduce foreign runtime/native dependencies.
- Confirm exact perfect-gas density, speed-of-sound, heat-capacity, molar-mass gas-constant conventions, sign conventions, units, equation identifiers, examples, and source citations for Phase 0.001 functions.
- Confirm rocket/nozzle equation source references, standard-gravity convention, pressure-thrust sign convention, representative delta-v/mass-ratio examples, ideal-thrust examples, choked-flow examples, and tolerances.

## Aerodynamics / flight dynamics

- Identify source references for dynamic pressure, force coefficient inverses, and induced drag.
- Identify exact source references for level coordinated-turn load factor, turn rate, turn radius, stall speed, and specific excess power.
- Document assumptions such as incompressible dynamic pressure, steady level coordinated turns, scalar preliminary-design usage, bank-angle singularity handling, reference-area convention, CLmax convention, and specific-excess-power sign/unit convention.

## Propulsion

- Identify exact source references for Tsiolkovsky delta-v, mass-ratio inverse, ideal thrust, specific impulse from effective exhaust velocity, and perfect-gas choked mass flux.
- Confirm standard-gravity convention, nozzle station definitions, pressure sign conventions, ideal/nozzle-loss assumptions, representative scalar examples, and tolerances.
- Keep propulsion scalar helpers separate from engine design validation, launch readiness, combustion chemistry, real-gas property modeling, CEA/native wrappers, staging, trajectory, and mission performance.

## Structures / heat transfer

- Identify exact source references for elementary axial stress, bending stress, cantilever end-load tip deflection, and Euler elastic column buckling equations.
- Confirm force/moment sign conventions, section-axis conventions, area and second-moment definitions, end-condition factor conventions, ideal-column assumptions, representative examples, and tolerances for structures helpers.
- Keep structures scalar helpers separate from finite-element validation, material allowables, fatigue/fracture, local buckling, design-code margins, and certification evidence.
- Identify source references for Stefan-Boltzmann radiation, Newton-law convection, 1-D conduction thermal resistance, and heat-rate-from-resistance equations.
- Confirm sign conventions and units for heat flux and heat rate helpers.

## Astrodynamics

- NASA/JPL gravitational parameter and planetary constant source review.
- Identify exact references for circular orbit speed, circular orbital period, escape velocity, vis-viva speed, and specific orbital energy.
- Confirm gravitational-parameter epoch/uncertainty, radius convention, sign convention for bound-orbit energy, representative two-body examples, and tolerances.
- Identify exact references for Hohmann-transfer delta-v components, total delta-v, transfer time, and scalar sphere-of-influence radius.

## Bio-regenerative life support

- NASA BVAD, ECLSS, crop productivity, oxygen/carbon dioxide/water balance, closure-fraction, required production area, and buffer-residence-time references.
- Confirm whether Phase 0.001 examples are illustrative only or suitable for later reference validation.

## Validation workflow

- Add source registry review PR checklist.
- Add exact source IDs and equation/table/page pointers after review.
- Upgrade card statuses only when evidence justifies it.
- Keep certification and mission-readiness caveats visible in docs and generated reports.

## Microtask 2 note

Microtask 2 locked versioning and roadmap categories only. It did not add or validate new source references.

## Microtask 2 versioning note

Microtask 2 changed roadmap and versioning documentation only. It did not review source editions, equation numbers, table numbers, validation datasets, or licensing/provenance details. No source-registry entry or validation card status was upgraded during Microtask 2.

## Microtask 3 core-vocabulary note

Microtask 3 refined the shared `AeroError`, `EngineeringResult<T>`, `ValidityStatus`, `VerificationStatus`, and `VerificationRecord` vocabulary. This improves how future equations will report assumptions, warnings, validity, and verification maturity, but it does not constitute source review. No source editions, equation numbers, table numbers, validation datasets, or provenance details were upgraded during Microtask 3.

## Microtask 4 unit-scalar note

Microtask 4 refined the shared unit-scalar wrappers and tests in `aero-codex-core`. This did not review equation sources, reference tables, validation datasets, page numbers, source editions, or provenance. No source-registry entry or validation card status was upgraded during Microtask 4.

## Microtask 5 constants/source-registry note

Microtask 5 refined the constants crate, added dependency-free constant metadata, expanded source-registry seed details, and added a conservative NIST/CODATA physical-constants research target. All constants and source-registry entries remain `research_required`; no source edition, equation number, table number, validation dataset, uncertainty claim, or provenance detail was upgraded during Microtask 5.


## Microtask 6 Codex Card schema/scaffold note

Microtask 6 tightened the Codex Card schema, added validation-scaffold documentation, and expanded dependency-free `xtask` text checks for cards, source-registry files, schema markers, and card-to-source ID links. This is governance scaffolding only. It did not review source editions, equation numbers, table numbers, validation datasets, experiment data, uncertainty metadata, or provenance. No validation card or source-registry entry was upgraded from `research_required` during Microtask 6.


## Microtask 7 atmosphere-equation note

Microtask 7 refined the simplified Phase 0.001 atmosphere crate, documented the troposphere-only altitude domain, added conservative trace metadata, and added `validation/cards/atmosphere_standard_troposphere.yaml`. This did not review source editions, equation numbers, tables, validation datasets, uncertainty metadata, or geometric/geopotential altitude provenance. The atmosphere card and U.S. Standard Atmosphere source-registry seed remain `research_required`.


## Microtask 8 thermodynamics perfect-gas note

Microtask 8 refined the Phase 0.001 thermodynamics crate, added conservative trace metadata, and added `validation/cards/thermo_perfect_gas.yaml`. This did not review NASA Glenn/CEA source editions, equation numbers, example values, property definitions, uncertainty metadata, or tolerances. The thermodynamics card, NASA Glenn/CEA source-registry seed, and NIST/CODATA gas-constant seed remain `research_required`.

## Microtask 9 gas-dynamics isentropic-flow note

Microtask 9 refined the `aero-codex-gas-dynamics` direct isentropic perfect-gas helpers, added conservative trace metadata, and added `validation/cards/gasdyn_isentropic_flow.yaml`. This did not verify NACA Report 1135 source edition, equation numbers, tables, branch conventions, reference values, or tolerances. The gas-dynamics card and source-registry seed remain `research_required`; inverse area-Mach branch solving is intentionally deferred.


## Microtask 10 gas-dynamics normal-shock note

Microtask 10 refined the `aero-codex-gas-dynamics` direct normal-shock perfect-gas helpers, added conservative trace metadata, and added `validation/cards/gasdyn_normal_shock.yaml`. This did not verify NACA Report 1135 source edition, normal-shock equation numbers, table values, reference examples, total-pressure-ratio convention, or tolerances. The normal-shock card and NACA source-registry seed remain `research_required`.


## Microtask 11 gas-dynamics Mach-angle/Prandtl-Meyer note

Microtask 11 refined the `aero-codex-gas-dynamics` Mach-angle and Prandtl-Meyer forward/inverse helpers, added conservative trace metadata, and added `validation/cards/gasdyn_mach_angle_prandtl_meyer.yaml`. This did not verify NACA Report 1135 source edition, Mach-angle or Prandtl-Meyer equation numbers, angle conventions, inverse-solver reference values, maximum-angle conventions, table values, or tolerances. The expansion-flow card and NACA source-registry seed remain `research_required`.


## Microtask 12 gas-dynamics oblique-shock note

Microtask 12 refined the `aero-codex-gas-dynamics` branch-explicit oblique-shock residual, beta solve, normal-Mach component, and downstream-Mach projection helpers, added conservative trace metadata, and added `validation/cards/gasdyn_oblique_shock.yaml`. This did not verify NACA Report 1135 source edition, theta-beta-Mach equation numbers, detachment-limit examples, weak/strong branch table values, downstream-Mach reference values, shock-polar conventions, detached-shock behavior, or tolerances. The oblique-shock card and NACA/equivalent source-registry seed remain `research_required`.


## Microtask 12 oblique-shock review note

The branch-explicit attached oblique-shock implementation remains `research_required`. Later source review must document the exact theta-beta-Mach equation source, weak/strong branch convention, theta-max or detachment boundary convention, representative values, and numerical tolerances before any status upgrade.


## Microtask 13 aerodynamics basic-coefficients note

Microtask 13 refined the `aero-codex-aerodynamics` scalar dynamic-pressure, force, inverse coefficient, and induced-drag helpers, added conservative trace metadata, added `validation/cards/aerodynamics_basic_coefficients.yaml`, and added `validation/source_registry/aerodynamics_basic_coefficients.yaml`. This did not verify exact source edition, equation numbers, reference-area conventions, sign conventions, aspect-ratio conventions, Oswald-efficiency references, representative values, or tolerances. The aerodynamics card and source-registry seed remain `research_required`.


## Microtask 14 propulsion rocket/nozzle note

Microtask 14 refined the `aero-codex-propulsion` ideal rocket-equation, mass-ratio inverse, ideal thrust, specific impulse, and choked mass-flux helpers, added conservative trace metadata, added `validation/cards/propulsion_rocket_nozzle_basics.yaml`, and added `validation/source_registry/propulsion_rocket_nozzle_basics.yaml`. This did not verify exact rocket/nozzle source edition, equation IDs, pressure-thrust sign convention, standard-gravity convention, representative examples, nozzle-loss assumptions, discharge coefficients, or tolerances. The propulsion card and source-registry seed remain `research_required`.


## Microtask 15 heat-transfer basic-primitives note

Microtask 15 refined the `aero-codex-heat-transfer` scalar Stefan-Boltzmann radiation, Newton-law convection, one-dimensional conduction resistance, and conduction heat-rate helpers, added conservative trace metadata, added `validation/cards/heat_transfer_basic_primitives.yaml`, and added `validation/source_registry/heat_transfer_basic_primitives.yaml`. This did not verify exact heat-transfer source edition, equation IDs, radiative view-factor convention, radiative/convective sign convention, conduction geometry convention, representative values, material-property assumptions, or tolerances. The heat-transfer card and source-registry seed remain `research_required`.

## Microtask 16 structures beam/buckling note

Microtask 16 refined the `aero-codex-structures` scalar axial-stress, bending-stress, cantilever end-load deflection, and Euler column buckling helpers, added conservative trace metadata, added `validation/cards/structures_beam_buckling_basics.yaml`, and added `validation/source_registry/structures_basic_mechanics.yaml`. This did not verify exact structures source edition, equation IDs, section-axis conventions, force/moment sign conventions, effective-length-factor convention, small-deflection assumptions, slender-column applicability, representative values, structural margins, material allowables, finite-element comparisons, or tolerances. The structures card and source-registry seed remain `research_required`.

## Microtask 17 flight-dynamics level-turn/performance note

Microtask 17 refined the `aero-codex-flight-dynamics` scalar level coordinated-turn load-factor, turn-rate, turn-radius, stall-speed, and specific-excess-power helpers, added conservative trace metadata, added `validation/cards/flight_dynamics_basic_performance.yaml`, and added `validation/source_registry/flight_dynamics_basic_performance.yaml`. This did not verify exact flight-mechanics source edition, equation IDs, bank-angle sign convention, turn-rate sign convention, turn-radius magnitude convention, stall-speed configuration/reference-area convention, specific-excess-power thrust/drag station definitions, representative examples, flight-test data, certification evidence, or tolerances. The flight-dynamics card and source-registry seed remain `research_required`.

### Microtask 18 astrodynamics two-body note

Microtask 18 refined the `aero-codex-astrodynamics` circular-orbit speed, circular-period, escape-velocity, vis-viva, and specific-orbital-energy helpers, added conservative trace metadata, added `validation/cards/astrodynamics_two_body_basics.yaml`, and added `validation/source_registry/astrodynamics_two_body_basics.yaml`. This did not verify NASA/JPL source edition, gravitational-parameter epoch, body-radius convention, equation identifiers, representative reference values, uncertainty metadata, or tolerances. The astrodynamics two-body card, two-body source-registry seed, and NASA/JPL parameter source-registry seed remain `research_required`.

### Microtask 19 astrodynamics Hohmann/celestial note

Microtask 19 refined the `aero-codex-astrodynamics` Hohmann delta-v component, total delta-v, transfer-time, and sphere-of-influence helpers, added conservative trace metadata, added `validation/cards/astrodynamics_transfer_celestial_basics.yaml`, and added `validation/source_registry/astrodynamics_transfer_celestial_basics.yaml`. This did not verify exact Hohmann source edition, equation identifiers, burn sign/magnitude convention, circular-radius convention, same-radius boundary convention, sphere-of-influence source form, mass/distance convention, representative reference values, or tolerances. The astrodynamics transfer/celestial card and source-registry seed remain `research_required`.

### Microtask 20 bio-regenerative life-support note

Microtask 20 refined the `aero-codex-life-support` scalar closure-fraction, required-production-area, buffer-residence-time, crew-daily-requirement, net-balance, oxygen-balance, carbon-dioxide-balance, and water-recovery helpers. It added conservative trace metadata through `verification_record(codex_id)`, reviewed the closure-fraction, required-production-area, buffer-residence-time, daily-mass-balance, and bio-regenerative mass-balance validation-card seeds, and kept the generic life-support and NASA BVAD/ECLSS source-registry seeds at `research_required`. This did not verify exact NASA BVAD/ECLSS source edition, equation or table identifiers, accounting-boundary conventions, crop-productivity values, crew metabolic rates, oxygen/carbon-dioxide/water representative examples, crew health limits, system reliability, ECLSS architecture suitability, or tolerances. No life-support validation card or source-registry entry was upgraded.
