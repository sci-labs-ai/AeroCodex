# Microtask 5 — Constants and Source Registry Seeds

Status: complete in this interactive session.

## Scope reviewed

Microtask 5 reviewed the `aero-codex-constants` crate and `validation/source_registry/` seeds. The goal was to ensure shared constants exist, are dependency-free, and are connected to conservative source-registry research targets without implying source verification, validation, certification, or operational readiness.

## Constants exposed

The constants crate exports these Phase 0.001 seed values:

| Constant | Unit | Status |
|---|---|---|
| `STANDARD_GRAVITY_M_S2` | `m/s^2` | `research_required` |
| `UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K` | `J/(mol*K)` | `research_required` |
| `STANDARD_SEA_LEVEL_PRESSURE_PA` | `Pa` | `research_required` |
| `STANDARD_SEA_LEVEL_TEMPERATURE_K` | `K` | `research_required` |
| `STANDARD_SEA_LEVEL_DENSITY_KG_M3` | `kg/m^3` | `research_required` |
| `STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K` | `J/(kg*K)` | `research_required` |
| `STANDARD_GAMMA_DRY_AIR` | dimensionless | `research_required` |
| `STEFAN_BOLTZMANN_W_PER_M2_K4` | `W/(m^2*K^4)` | `research_required` |
| `EARTH_GRAVITATIONAL_PARAMETER_M3_S2` | `m^3/s^2` | `research_required` |
| `EARTH_MEAN_RADIUS_M` | `m` | `research_required` |
| `SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER` | `m^3/s^2` | `research_required` |

The solar gravitational-parameter item remains explicitly a placeholder. It is not marked verified and is not used by Phase 0.001 equation implementations.

## Metadata added

The constants crate now includes a dependency-free `ConstantSeed` metadata table:

```text
PHASE_0_001_CONSTANT_SEEDS
constant_seed(symbol)
RESEARCH_REQUIRED_STATUS
SOURCE_ID_* source-registry hints
```

This provides a simple bridge between public constants and source-registry research targets without adding serialization dependencies or claiming verified provenance.

## Source-registry seeds

The required Microtask 5 source-registry seeds are present:

| File | Status |
|---|---|
| `validation/source_registry/us_standard_atmosphere_1976.yaml` | `research_required` |
| `validation/source_registry/naca_report_1135.yaml` | `research_required` |
| `validation/source_registry/nasa_glenn_thermo_cea.yaml` | `research_required` |
| `validation/source_registry/nasa_jpl_astrodynamics_parameters.yaml` | `research_required` |
| `validation/source_registry/nasa_life_support_bvad_eclss.yaml` | `research_required` |

An additional conservative seed was added for physical constants used by the constants crate:

```text
validation/source_registry/nist_codata_physical_constants.yaml
```

All source-registry seeds remain `research_required` and include review-needed fields rather than page, equation, or table claims.

## Verification notes

Static checks confirmed:

- the constants crate still has no dependencies;
- the required Microtask 5 constants are present;
- the `ConstantSeed` metadata table covers the public constants;
- all constant seed metadata remains `research_required`;
- all required source-registry seed files exist;
- no source-registry seed was upgraded beyond `research_required`;
- the solar gravitational-parameter placeholder is explicitly not verified;
- Cargo manifests do not contain forbidden native-wrapper dependency tokens.

Rust compilation, formatting, clippy, unit tests, docs, and `xtask` execution must still be run by the deployment agent in an environment with `cargo` and `rustc`.

## Definition-of-done result

Shared constants exist, source-registry seeds exist, and source status remains conservative without overclaiming.
