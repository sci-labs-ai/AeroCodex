# AeroCodex Documentation

AeroCodex Phase 0.001 establishes a pure-Rust workspace for source-traceable aerospace engineering mathematics.

Start with:

- [Nomenclature and acronym policy](nomenclature_policy.md)
- [Versioning](roadmap/versioning.md)
- [Milestones](roadmap/milestones.md)
- [Post-1.0 expansion concepts](roadmap/post_1_0_expansion.md)
- [Phase 0.001 working inventory](phase_0_001/working_inventory.md)
- [Microtask 2 versioning review](phase_0_001/microtask_002_versioning_review.md)
- [Microtask 3 core result/error/verification review](phase_0_001/microtask_003_core_result_error_verification.md)
- [Microtask 4 unit-safe scalar review](phase_0_001/microtask_004_unit_scalars.md)
- [Microtask 5 constants/source-registry review](phase_0_001/microtask_005_constants_source_registry.md)
- [Microtask 6 Codex Card schema/validation scaffold review](phase_0_001/microtask_006_codex_card_schema_validation.md)
- [Microtask 7 atmosphere equations review](phase_0_001/microtask_007_atmosphere_equations.md)
- [Microtask 8 thermodynamics perfect-gas equations review](phase_0_001/microtask_008_thermodynamics_perfect_gas.md)
- [Microtask 9 gas-dynamics isentropic-flow review](phase_0_001/microtask_009_gas_dynamics_isentropic.md)
- [Microtask 10 gas-dynamics normal-shock review](phase_0_001/microtask_010_gas_dynamics_normal_shock.md)
- [Microtask 11 gas-dynamics Mach-angle/Prandtl-Meyer review](phase_0_001/microtask_011_gas_dynamics_mach_angle_prandtl_meyer.md)
- [Microtask 12 gas-dynamics oblique-shock review](phase_0_001/microtask_012_gas_dynamics_oblique_shock.md)
- [Microtask 13 aerodynamics basic-coefficients review](phase_0_001/microtask_013_aerodynamics_basic_coefficients.md)
- [Microtask 14 propulsion rocket/nozzle basics review](phase_0_001/microtask_014_propulsion_rocket_nozzle_basics.md)
- [Microtask 15 heat-transfer primitives review](phase_0_001/microtask_015_heat_transfer.md)
- [Microtask 16 structures beam/buckling review](phase_0_001/microtask_016_structures_beam_buckling.md)
- [Microtask 17 flight-dynamics basic-performance review](phase_0_001/microtask_017_flight_dynamics_basic_performance.md)
- [Microtask 18 astrodynamics two-body basics review](phase_0_001/microtask_018_astrodynamics_two_body.md)
- [Microtask 19 astrodynamics Hohmann/celestial helpers review](phase_0_001/microtask_019_astrodynamics_transfer_celestial.md)
- [Microtask 20 bio-regenerative life-support review](phase_0_001/microtask_020_bioregenerative_life_support.md)
- [Validation scaffold README](../validation/README.md)
- [Phase 0.001 version-lock audit](phase_0_001/version_lock_audit.md)
- [Final microtasks 001-020 report](phase_0_001/final_microtasks_001_020_report.md)
- [Phase 0.001 tracked task breakdown](phase_0_001/tracked_tasks.md)
- [Source research backlog](phase_0_001/source_research_backlog.md)

## Version lock

The human roadmap phase is `Phase 0.001`; Cargo package versions remain `0.0.1`. The project must not use `0.001` as a Cargo package version.

Certification caveat: AeroCodex is for research, education, verification-oriented development, and preliminary design. Safety-critical, regulated, or mission use requires project-specific assurance, validation, qualification, and certification.

## Microtask 20 life-support review

The Phase 0.001 life-support crate has reviewed scalar closure-fraction, required production-area, buffer residence-time, crew daily-requirement, net daily-balance, and optional O2/CO2/water bookkeeping helpers. Closure fractions above 1 are returned with warning metadata and `OutsideDocumentedDomain` validity rather than clipped. These remain simple mass-balance primitives with `research_required` trace metadata and are not validated ECLSS design models.
