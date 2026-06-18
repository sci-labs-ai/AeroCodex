# M07 formula-family classifier summary

Session: **C2 — Stage 5 M07 formula-family classifier dataset**.

Classification: **ready_for_live_intake** as quarantined planning metadata, not implementation approval.

This handoff classifies the remaining M07 backlog from `PORT_STATUS_RELEASE_GATE.csv` using metadata fields only: milestone, Rust alias, Scilab alias, source-file locator, module, and release-gate status columns. It does not copy raw M07 source text, comments, control flow, generated code, Scilab output, archives, binaries, or fixtures. It does not add public APIs or implementation code.

## Inputs reviewed

| Input | Observation |
| --- | --- |
| Current repo snapshot SHA256 | `6d0bc8c6925d51e20232ce7302bc8b955e39c4b21b045c08133481608420641b` |
| M07 final bundle SHA256 | `15b1ca3a39267187167c43ea1228f28fd4736c4456f65d072dc42a32a7b19190` |
| M07 represented function rows | 1350 rows in `PORT_STATUS_RELEASE_GATE.csv` |
| Current formula-vault represented rows excluded | 17 rows: `deg2rad`, `rad2deg`, `wrap2pi`, and the 14 M00 vector-algebra candidate rows already represented in the current repo snapshot |
| Rows classified in this handoff | **1333** |
| Scilab equivalence source jobs available as metadata | 188 source-file-level jobs |

## Current repo baseline from uploaded snapshot

The aggregate row-count inventory in `validation/equation_inventory.tsv` matches the governed Stage 4 baseline by count:

| Count key | Uploaded snapshot aggregate |
| --- | ---: |
| executable_research_equations | 128 |
| metadata_only_candidates | 17 |
| external_m07_backlog_rows | 1333 |
| validation_card_only_records | 38 |
| helper_algorithms | 89 |
| source_registry_seeds, file count | 36 |

Live deployment must keep these counts as classifier-source accounting only; governed equation-inventory counts remain controlled by `validation/equation_inventory.tsv`.

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

Reason: the output is documentation/data inventory only, uses quarantined source metadata locators rather than source expressions, and makes no validation, certification, operational, flight, mission, or regulated-use claims. The deployment agent must apply this serially, run repository checks, and review classifier heuristics before merging.


## Live Stage 5 C2 deployment notes

This repository copy is the classifier dataset only. It preserves quarantined source metadata locators and keeps M07 rows in planning status. It does not import source expressions, Scilab output, raw source archives, fixtures, generated Rust, public APIs, or validation-status promotion.

The authorized deterministic locator normalization rule for this deployment was: replace a `source_file_locator` prefix beginning exactly `/mnt/data/scilab_extracted/` with `source_scilab_extracted_locator/` and leave all other locator values unchanged. Live checks found zero `/mnt/data/scilab_extracted/` locators in the three deployed CSV files, so no data-cell replacements were needed. Existing logical `source_scilab/` and `source_scilab_extracted_locator/` values were left unchanged.
