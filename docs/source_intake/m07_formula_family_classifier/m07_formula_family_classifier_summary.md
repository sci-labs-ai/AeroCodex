# M07 formula-family classifier summary

Session: **C2 — Stage 5 M07 formula-family classifier dataset**.

Historical intake classification: **ready_for_live_intake** as quarantined planning metadata, not implementation approval. Current status: deployed/completed as research/planning metadata; explicit C2 DATA_REGISTRY coverage is closed by the Stage 5 status reconciliation.

This handoff classifies the remaining M07 backlog from `PORT_STATUS_RELEASE_GATE.csv` using metadata fields only: milestone, Rust alias, source-file locator, module, and release-gate status columns. It does not copy raw M07 implementation text, comments, control-flow detail, generated code, external runtime output, archives, binaries, or fixtures. It does not add public APIs or implementation code.

## Inputs reviewed

| Input | Observation |
| --- | --- |
| Current repo snapshot SHA256 | `6d0bc8c6925d51e20232ce7302bc8b955e39c4b21b045c08133481608420641b` |
| M07 final bundle SHA256 | `15b1ca3a39267187167c43ea1228f28fd4736c4456f65d072dc42a32a7b19190` |
| M07 represented function rows | 1350 rows in `PORT_STATUS_RELEASE_GATE.csv` |
| Current formula-vault represented rows excluded | 17 rows: `deg2rad`, `rad2deg`, `wrap2pi`, and the 14 M00 vector-algebra candidate rows already represented in the current repo snapshot |
| Rows classified in this handoff | **1333** |
| Scilab equivalence source jobs available as metadata | 188 source-file-level jobs |

## Current repo baseline from live main

The classifier is planning metadata and does not change governed equation-inventory counts. Live verifier counts at this reconciliation are:

| Count key | Live main value |
| --- | ---: |
| executable_research_equations | 138 |
| metadata_only_candidates | 27 |
| external_m07_backlog_rows | 1323 |
| validation_cards | 44 |
| source_registry_seeds | 42 |
| validation_card_only_records | 44 |
| helper_algorithms | 138 |

C2 does not remove classifier rows from `external_m07_backlog_rows`, does not promote source or validation status, and remains research/planning metadata.


### Post-Stage-5 A11 overlay

The table above is the historical C2 reconciliation baseline. A11 subsequently processes 38 low-risk rows from `8D_deduplicated_unit_conversion_helpers` using classifier metadata only. Thirty-seven rows are deduplicated aliases to existing governed M00 runtimes; `earth_rotation_rate_canonical` remains contract-blocked. Current external accounting is 38 terminally processed rows and 1,285 remaining backlog rows. No raw M07 or Scilab source is imported or executed.

### Post-Stage-5 A12 overlay

A12 subsequently processes the first 40 rows from `8D_helper_deduplication_then_low_risk_vector_contracts` using classifier metadata only. Thirty rows deduplicate to existing M00 vector runtimes, eight internal shape/identity helpers are excluded from formula scope, and two rows remain contract-blocked. Current external accounting is 78 terminally processed rows and 1,245 remaining backlog rows. No raw M07 or Scilab source is imported or executed.

## Counts by formula family

| Formula family | Rows |
| --- | ---: |
| `ambiguous_source_or_contract` | 91 |
| `angle_normalization` | 27 |
| `blocked_until_policy` | 20 |
| `coordinate_transform_sensitive` | 157 |
| `external_data_required` | 15 |
| `frame_graph_sensitive` | 44 |
| `iterative_solver` | 129 |
| `least_squares_or_solver` | 4 |
| `low_risk_scalar_math` | 44 |
| `low_risk_vector_math` | 74 |
| `orbit_two_body` | 487 |
| `perturbation_or_J2` | 128 |
| `sgp4_teme_sensitive` | 39 |
| `time_scale_sensitive` | 12 |
| `unit_conversion` | 62 |

## Counts by risk tier

| Risk tier | Rows |
| --- | ---: |
| `blocked_until_frame_time_policy` | 95 |
| `blocked_until_solver_policy` | 123 |
| `blocked_until_source_review` | 15 |
| `do_not_import` | 33 |
| `high_risk_requires_numerical_policy` | 328 |
| `low_risk_candidate` | 154 |
| `medium_risk_requires_contract_review` | 585 |

## Counts by recommended chunk group

| Recommended group | Rows |
| --- | ---: |
| `9A_classical_elements_and_9E_mission_design_contracts` | 377 |
| `10B_J2_perturbation_and_numerical_policy` | 128 |
| `9C_kepler_lambert_gauss_solver_policy_or_10B_numerical_propagation_policy` | 105 |
| `8D_helper_deduplication_then_low_risk_vector_contracts` | 74 |
| `9E_rocket_vehicle_policy_then_bounded_scalar_slice` | 70 |
| `10A_attitude_quaternion_DCM_contracts` | 59 |
| `9B_coordinate_transform_contracts_after_frame_policy` | 58 |
| `10E_classifier_refresh_or_manual_source_review` | 58 |
| `8D_deduplicated_unit_conversion_helpers` | 51 |
| `8E_or_9A_classical_two_body_algebra_contracts` | 49 |
| `10C_sgp4_teme_oracle_and_frame_time_gate` | 39 |
| `10A_attitude_dynamics_and_control_policy` | 38 |
| `8D_helper_deduplication_then_low_risk_scalar_contracts` | 34 |
| `9D_relative_motion_CW_LVLH_policy` | 29 |
| `8B_or_8D_angle_endpoint_policy_then_deduplicate_wrappers` | 27 |
| `10E_CR3BP_family_policy_and_oracle_before_promotion` | 23 |
| `8D_deduplicate_helpers_and_test_utility_policy` | 20 |
| `9B_frame_graph_time_policy_before_coordinate_transforms` | 15 |
| `10D_external_data_table_and_fixture_governance` | 15 |
| `9C_or_10B_generic_numerical_method_policy` | 14 |
| `9B_time_scale_and_sidereal_policy` | 12 |
| `8C_remaining_M00_unit_conversion_contracts` | 11 |
| `9E_rocket_equation_scalar_subset_after_contract` | 10 |
| `10D_do_not_import_io_demo_utility_rows` | 7 |
| `10C_sgp4_hold_no_public_helper_import` | 6 |
| `9C_solver_rank_tolerance_and_observation_policy` | 3 |
| `9C_solver_rank_tolerance_policy_before_any_promotion` | 1 |

## Remaining rows by M07 module

| M07 module | Rows |
| --- | ---: |
| `attitude::rigid_body` | 96 |
| `perturbations::ch9` | 94 |
| `classical::ch3` | 89 |
| `attitude::control` | 87 |
| `classical::ch4` | 83 |
| `mission_design::ch7_lunar` | 79 |
| `classical::ch1 or units/constants` | 75 |
| `mission_design::gravity_assist` | 73 |
| `classical::ch6` | 72 |
| `rocket_vehicle` | 72 |
| `mission_design::interplanetary` | 67 |
| `classical::ch5` | 57 |
| `perturbations::j2` | 54 |
| `relative_motion::curtis` | 53 |
| `vallado_sgp4` | 47 |
| `mission_design::finite_burn` | 39 |
| `numerical_algorithms` | 34 |
| `cr3bp` | 33 |
| `classical::ch2_observations` | 26 |
| `classical::ch2::coordinates` | 25 |
| `appendix_projects` | 25 |
| `classical::ch2::core` | 19 |
| `classical::ch2::elements` | 14 |
| `relative_motion::cw` | 10 |
| `units/constants` | 9 |
| `appendix_vector` | 1 |

## Interpretation

- **Low-risk candidates are not automatically implementation-ready.** Many are duplicate scalar/vector helpers that should be deduplicated against the existing M00 central kernels before any public formula surface is proposed.
- **Medium-risk rows require a family contract first.** Typical blockers are conic branch conventions, units, singular elements, rotation order, or source-constant policy.
- **High-risk and blocked rows require policy chunks before formula promotion.** Solver/rank-sensitive, frame/time, SGP4/TEME, J2/perturbation, CR3BP, and external-data rows should stay blocked until their family policies and oracle fixtures exist.
- **`app_resolve_coplanar` remains deliberately blocked.** It is retained in the classifier with `blocked_until_solver_policy` because rank/tolerance behavior must be isolated before any implementation work.

## Handoff status

`ready_for_live_intake`

Reason: the output is documentation/data inventory only, uses quarantined source metadata locators rather than source expressions, and makes no validation, certification, operational, flight, mission, or regulated-use claims. Low-risk classification is prioritization metadata only and is not implementation approval.


## Live Stage 5 C2 deployment notes

This repository copy is the classifier dataset only. It preserves quarantined source metadata locators and keeps M07 rows in planning status. It does not import source implementation text, external runtime output, raw source archives, fixtures, generated Rust, public APIs, or validation-status promotion.

The deployed dataset contains 1,333 classifier rows. The core classifier contains exactly 115 `source_file_locator` values beginning with `source_scilab_extracted_locator/`, zero obsolete absolute extraction prefixes, and zero preparation-machine absolute paths. Existing logical `source_scilab/` and `source_scilab_extracted_locator/` values are logical metadata locators, not imported source.

The three CSV files have explicit DATA_REGISTRY coverage after the Stage 5 status reconciliation: the core classifier, low-risk derivative, and blocked/high-risk derivative each carry an exact SHA256 entry. DATA_MANIFEST.toml remains thin-film-specific and is not used to cover C2.

### Post-Stage-5 A13 overlay

A13 subsequently processes the remaining 34 rows from `8D_helper_deduplication_then_low_risk_vector_contracts` using classifier metadata only. Twenty-six rows deduplicate to existing M00 vector runtimes, five internal column-shape helpers are excluded from formula scope, and three rows remain contract-blocked. A12-A13 now cover the full 74-row group. A14-A15 then process all 49 rows of `8E_or_9A_classical_two_body_algebra_contracts`: 22 exact-name aliases reuse existing governed A7 runtimes and 27 rows remain contract-blocked. Both waves preserve the classifier risk tier `medium_risk_requires_contract_review`. Current external accounting is 161 terminally processed rows and 1,162 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

A16 then processes the first 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 2 exact aliases reuse governed A7 runtimes, 10 internal support helpers are excluded from formula scope, and 28 rows remain contract-blocked. A16 preserves the medium-risk classifier tier, leaves 337 group rows, and updates external accounting to 201 terminally processed rows and 1,122 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.
