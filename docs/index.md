# AeroCodex Documentation

AeroCodex Phase 0.001 establishes a pure-Rust workspace for source-traceable aerospace engineering mathematics.

Start with:

- [Stage 4 master plan](roadmap/stage4_master_plan.md)
- [Stage 5 master plan](roadmap/stage5_master_plan.md)
- [Stage 5 deployment queue](roadmap/stage5_deployment_queue.md)
- [Stage 5 handoff inventory](source_intake/stage5_handoff_inventory.md)
- [Stage 5 BioSim/Orekit v3 serial plan](roadmap/stage5_bio_ore_v3_serial_plan.md)
- [Stage 5 agent operating rules](deployment/stage5_agent_operating_rules.md)
- [Data/source governance](../data-governance/)
- [Data/source registry](../data-governance/DATA_REGISTRY.yaml)
- [Status vocabulary](../validation/status_vocabulary.md)
- [Status vocabulary registry](../validation/status_vocabulary.yaml)
- [Formula-vault staging design](assurance/formula_vault_staging.md)
- [Formula-vault candidate gate](assurance/formula_vault_candidate_gate.md)
- [Formula-vault M00 angle/unit conversion metadata slice](assurance/formula_vault_m00_angle_unit_candidate.md)
- [Formula-vault M00 per-candidate manifest/reference-link depth](assurance/formula_vault_m00_reference_manifest.md)
- [Formula-vault M00 source-expression and test-vector contracts](assurance/formula_vault_m00_source_expression_test_vectors.md)
- [Formula-vault M00 vector-equation expansion](assurance/formula_vault_m00_vector_equation_expansion.md)
- [Formula-vault candidate verifier](assurance/formula_vault_candidate_verifier.md)
- [Equation inventory/readiness dashboard](assurance/equation_inventory_readiness_dashboard.md)
- [Formula-vault runtime resolution](assurance/formula_vault_runtime_resolution.md)
- [External M07 unit-conversion Wave 1 resolution](assurance/m07_unit_conversion_wave1_resolution.md)
- [External M07 vector-helper Wave 1 resolution](assurance/m07_vector_helper_wave1_resolution.md)
- [External M07 vector-helper Wave 2 resolution](assurance/m07_vector_helper_wave2_resolution.md)
- [External M07 classical two-body algebra Wave 1 resolution](assurance/m07_two_body_algebra_wave1_resolution.md)
- [External M07 classical two-body algebra Wave 2 resolution](assurance/m07_two_body_algebra_wave2_resolution.md)
- [External M07 orbital-geometry and conic-branch Wave 1 resolution](assurance/m07_orbital_geometry_conic_wave1_resolution.md)
- [External M07 orbital-geometry and conic-branch Wave 2 resolution](assurance/m07_orbital_geometry_conic_wave2_resolution.md)
- [External M07 orbital-geometry and conic-branch Wave 3 resolution](assurance/m07_orbital_geometry_conic_wave3_resolution.md)
- [External M07 orbital-geometry and conic Wave 4 resolution](assurance/m07_orbital_geometry_conic_wave4_resolution.md)
- [Equation batch compiler](../equation-batches/README.md)
- [Validation-card generation policy](assurance/validation_card_generation_policy.md)
- [Source-seed generation policy](assurance/source_seed_generation_policy.md)
- [M07 formula-family validation-card strategy](assurance/validation_family_card_strategy.md)
- [Stage 5 M07 formula-family wave plan](roadmap/stage5_m07_formula_family_wave_plan.md)
- [M07 formula-family classifier dataset](source_intake/m07_formula_family_classifier/m07_formula_family_classifier_summary.md)
- [M07 formula-vault intake](source_intake/m07_formula_vault_intake.md)
- [BioSim-RS license-bound architecture](assurance/biosim_rs_license_architecture.md)
- [BioSim-RS resource identity and tick validation](assurance/biosim_rs_resource_tick_validation.md)
- [BioSim-RS atomic transaction commit validation](assurance/biosim_rs_atomic_transaction_commit_validation.md)
- [BioSim-RS deterministic replay validation](assurance/biosim_rs_deterministic_replay_validation.md)
- [BioSim-RS resource ledger validation](assurance/biosim_rs_resource_ledger_validation.md)
- [BioSim-RS CLI/API smoke and friend-test report](assurance/biosim_rs_cli_api_smoke_friend_test_report.md)
- [BioSim-RS source boundary](source_intake/biosim_rs_source_boundary.md)

- [BioSim-plus clean-room runtime boundary](architecture/biosim_plus_clean_room_runtime_boundary.md)
- [BioSim-plus B2c replay-ledger report assurance](assurance/biosim_plus_clean_room_scenario_engine.md)
- [BioSim-RS boundary placeholder](../biosim-rs/)
- [Orekit reference-oracle mapping](assurance/orekit_reference_oracle_mapping.md)
- [Astrodynamics time/frame/state, elliptic-helper, oracle, and TLE-contract foundation](assurance/astrodynamics_time_frame_tle_foundation.md)
- [Astrodynamics TLE contract-only source policy](assurance/astrodynamics_tle_contract_source_policy.md)
- [Orekit non-copying Rust foundation boundary](assurance/orekit_non_copying_rust_foundation.md)
- [Orekit reference-oracle source boundary](source_intake/orekit_reference_oracle_boundary.md)
- [Stage 4 agent operating rules](deployment/stage4_agent_operating_rules.md)
- [Math correctness policy](assurance/math_correctness_policy.md)
- [Merge and release policy](assurance/merge_and_release_policy.md)
- [Stage 4 source inventory](source_intake/stage4_source_inventory.md)
- [Nomenclature and acronym policy](nomenclature_policy.md)
- [Versioning](roadmap/versioning.md)
- [Beta 1 concept](beta1/release_concept.md)
- [Beta 1 CLI quickstart](beta1/cli_quickstart.md)
- [Beta 1 release-candidate testing](beta1/release_testing.md)
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

- [External M07 orbital-geometry and conic Wave 5 resolution](assurance/m07_orbital_geometry_conic_wave5_resolution.md)

- [External M07 orbital-geometry/conic Wave 6 resolution](assurance/m07_orbital_geometry_conic_wave6_resolution.md)
- [External M07 orbital-geometry/conic Wave 7 resolution](assurance/m07_orbital_geometry_conic_wave7_resolution.md)

- [External M07 orbital-geometry/conic Wave 8 resolution](assurance/m07_orbital_geometry_conic_wave8_resolution.md)

- [External M07 orbital-geometry/conic Wave 9 resolution](assurance/m07_orbital_geometry_conic_wave9_resolution.md)

- [External M07 orbital-geometry/conic Wave 10 resolution](assurance/m07_orbital_geometry_conic_wave10_resolution.md)

- [External M07 coordinate-transform/frame-time policy Wave 1 resolution](assurance/m07_coordinate_transform_frame_time_policy_wave1_resolution.md)

- [External M07 coordinate-transform/frame-time policy Wave 2 resolution](assurance/m07_coordinate_transform_frame_time_policy_wave2_resolution.md)
- [A28 solver policy Wave 1 resolution](assurance/m07_solver_policy_wave1_resolution.md)

- [A29 solver policy Wave 2 resolution](assurance/m07_solver_policy_wave2_resolution.md)
- [A30 solver policy Wave 3 resolution](assurance/m07_solver_policy_wave3_resolution.md)

- [A31 relative-motion finite-burn policy Wave 1 resolution](assurance/m07_relative_motion_finite_burn_policy_wave1_resolution.md)
- [A32 relative-motion finite-burn policy Wave 2 resolution](assurance/m07_relative_motion_finite_burn_policy_wave2_resolution.md)
- [A33 relative-motion / finite-burn scalar policy Wave 3 resolution](assurance/m07_relative_motion_finite_burn_policy_wave3_resolution.md)

- [A34 external M07 attitude / inertia / quaternion policy Wave 1](assurance/m07_attitude_frame_policy_wave1_resolution.md)

- [A35 external M07 attitude / inertia / quaternion policy Wave 2](assurance/m07_attitude_frame_policy_wave2_resolution.md)

- [A36 external M07 attitude dynamics/control policy Wave 1](assurance/m07_attitude_dynamics_control_policy_wave1_resolution.md)

- [A37 external M07 J2 perturbation / numerical propagation policy Wave 1](assurance/m07_j2_perturbation_policy_wave1_resolution.md)

- [A38 external M07 J2 perturbation / numerical propagation policy Wave 2](assurance/m07_j2_perturbation_policy_wave2_resolution.md)
- [A39 external M07 J2 perturbation / numerical propagation policy Wave 3](assurance/m07_j2_perturbation_policy_wave3_resolution.md)

- [A40 external M07 SGP4 / TEME frame-time policy Wave 1](assurance/m07_sgp4_teme_policy_wave1_resolution.md)
- [A41 external M07 CR3BP / external-data / input-output policy Wave 1](assurance/m07_cr3bp_external_data_policy_wave1_resolution.md)
- A42 records classifier-refresh/manual source-review metadata-only terminal dispositions and leaves runtime/public API behavior unchanged.

- A43 records scalar-helper and unit/constants helper metadata-only terminal dispositions and leaves runtime/public API behavior unchanged.

- A44 records residual angle endpoint, scalar-helper, and unit/constants metadata-only dispositions and leaves runtime/public API behavior unchanged.

- [A45 final residual backlog closure Wave 1](assurance/m07_final_residual_backlog_closure_wave1_resolution.md)
