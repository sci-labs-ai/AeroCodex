# Formula vault

This directory contains formula-vault intake metadata, contracts, and explicit links to governed AeroCodex runtimes. It intentionally contains no imported M07 source code, external archives, generated binaries, or implementation source; executable mathematics remains in the Rust crates.

AeroCodex remains research and preliminary-design software. This skeleton does not imply certification, flight readiness, mission readiness, habitat-safety approval, medical-use approval, operational approval, or regulated-use approval.

## Current state

Stage 4 Chunk 3 defines the vault shape in documentation only:

- `docs/assurance/formula_vault_staging.md`
- `docs/source_intake/m07_formula_vault_intake.md`

Stage 4 Chunk 7A adds the first candidate-gate metadata package without selecting or implementing a formula:

- `docs/assurance/formula_vault_candidate_gate.md`
- `formula-vault/templates/implementation_candidate_slice.yaml`
- validation card `validation.formula_vault.candidate_gate`
- source seed `source.validation.formula_vault_candidate_gate.research_required`

Stage 4 Chunk 7B adds the first bounded metadata-only candidate slice without implementing formulas:

- `formula-vault/candidates/m00_angle_unit_conversions.yaml`
- `docs/assurance/formula_vault_m00_angle_unit_candidate.md`
- validation card `validation.formula_vault.m00_angle_unit_conversions`
- source seed `source.formula_vault.m00_angle_unit_conversions.research_required`

The Chunk 7B slice is limited to three M00 release-gate rows: `app_deg2rad`, `app_rad2deg`, and `app_wrap2pi`. It does not promote exact expressions, wrap endpoint behavior, tolerances, executable code, or public application programming interfaces.

Stage 4 Chunk 8A handoff expands the formula-vault with a bounded M00 vector-algebra slice and implementation-ready research-kernel patch:

- `formula-vault/candidates/m00_vector_algebra.yaml`
- `formula-vault/contracts/m00_vector_algebra_contract.yaml`
- `docs/assurance/formula_vault_m00_vector_equation_expansion.md`
- validation card `validation.formula_vault.m00_vector_algebra`
- source seed `source.formula_vault.m00_vector_algebra.research_required`

The handoff covers fourteen finite 3-vector helpers, plus implementation of the already-contracted `deg2rad` and `rad2deg` helpers. Post-Stage-5 adds only the bounded `m00_wrap2pi` Rust runtime for `formula_vault.m00.angle.wrap2pi`; `app_resolve_coplanar` remains blocked for a separate least-squares/rank/tolerance policy chunk.

Stage 4 Chunk 7C adds the first dependency-free candidate metadata verifier without implementing formulas:

- command `cargo run -p xtask -- verify formula-vault`
- assurance note `docs/assurance/formula_vault_candidate_verifier.md`
- validation card `validation.formula_vault.candidate_verifier`
- source seed `source.validation.formula_vault_candidate_verifier.research_required`

The Chunk 7C verifier checks required metadata fields, cross-links, duplicate slice/formula identifiers, blocked promotion state, non-claim booleans, and absence of local evidence paths. It is included in `cargo run -p xtask -- verify --all`.

Stage 4 Chunk 7D adds the first per-candidate manifest/reference-link package without implementing formulas:

- `formula-vault/manifests/m00_angle_unit_conversions_manifest.yaml`
- `docs/assurance/formula_vault_m00_reference_manifest.md`
- validation card `validation.formula_vault.m00_reference_manifest`
- source seed `source.formula_vault.m00_reference_manifest.research_required`

The Chunk 7D manifest links each selected formula identifier to row/function/source-file aliases, pending source-expression review status, and assurance/validation/source/intake records. It does not copy source expressions, import M07 source, execute Scilab, promote fixtures, implement formulas, or create public application programming interfaces.

The M07 source artifact remains registered externally as `stage4.m07_rust_port_v14.2026_06_15` in `data-governance/DATA_REGISTRY.yaml`.


## A10 runtime resolution

A10 resolves every existing formula-vault intake formula ID without adding or duplicating a Rust kernel:

- `formula-vault/resolutions/m00_runtime_links.tsv` links all 27 candidate formula IDs to the existing `m00-angle-vector` and `m00-canonical-units` equation batches;
- 3 angle/unit, 14 vector-algebra, and 10 canonical-unit records have disposition `linked_to_existing_runtime`;
- unresolved candidate formula IDs: 0;
- validation remains `research_required`;
- the candidate YAML files remain metadata/provenance records and are not implementation source;
- no M07/Scilab parity, certification, flight readiness, mission readiness, operational approval, or regulated-use approval is claimed.

Run the dependency-free resolution check with:

```text
cargo run -p xtask -- verify formula-vault
```

The complete governance wrapper runs this check before the existing xtask gate:

```text
cargo run -p xtask -- verify --all
```

## A11 external unit-conversion resolution

A11 processes 38 low-risk external unit-conversion classifier rows without adding a formula node or Rust kernel. Thirty-seven rows are recorded as deduplicated aliases of existing governed M00 runtimes; `earth_rotation_rate_canonical` remains contract-blocked. The remaining external backlog is 1,285 rows.

- disposition manifest: `formula-vault/resolutions/m07_unit_conversion_wave1.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- validation status: `research_required`;
- new validation cards or source seeds: none.


## A12 external vector-helper resolution

A12 processes the first 40 low-risk vector-helper classifier rows without adding a formula node or Rust kernel. Thirty rows are deduplicated aliases of existing governed M00 vector runtimes, eight shape/identity helpers are excluded as internal utilities, and two rows remain contract-blocked. The aggregate terminal-disposition count is 78 and the remaining external backlog is 1,245 rows.

- disposition manifest: `formula-vault/resolutions/m07_vector_helper_wave1.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- validation status: `research_required`;
- new validation cards or source seeds: none.

## Future allowed contents

Future chunks may add reviewed metadata such as:

- formula inventory records;
- equation contract drafts;
- source artifact identifiers;
- source equation/table/page/function-row references;
- variables, units, coordinate/time assumptions, domains, exclusions, and singularities;
- tolerance and reference-oracle plans;
- promotion-gate checklists.

Future chunks must not add raw M07 source code or public application-programming-interface implementation files here unless the active prompt explicitly authorizes that bounded scope and the required gates pass.


Stage 4 Chunk 7F adds a metadata-only source-expression and test-vector contract for the existing M00 angle/unit candidate:

- `formula-vault/contracts/m00_angle_unit_conversions_contract.yaml`
- `docs/assurance/formula_vault_m00_source_expression_test_vectors.md`
- validation card `validation.formula_vault.m00_source_expression_test_vectors`
- source seed `source.formula_vault.m00_source_expression_test_vectors.research_required`

The contract records independent mathematical summaries, finite-input domains, tolerance metadata, and endpoint-sensitive `wrap2pi` expectations. Post-Stage-5 deploys the single public `m00_wrap2pi` Rust API with research_required status, finite-input validation, `rem_euclid(std::f64::consts::TAU)`, [0, TAU) output, canonical positive zero, nonfinite rejection, no epsilon/ordinary-value clamping, and no M07/Scilab parity claim. It does not import M07 source, generate Rust from source material, import Scilab outputs or fixtures, or promote alternate public aliases.

## A13 external vector-helper completion

A13 processes the remaining 34 rows in `8D_helper_deduplication_then_low_risk_vector_contracts` without adding a formula node or Rust kernel. Twenty-six rows are deduplicated aliases of existing governed M00 vector runtimes, five column-shape helpers are excluded as internal utilities, and three rows remain contract-blocked. A12-A13 now cover the complete 74-row vector-helper group. Aggregate terminal dispositions are 112 and the remaining external backlog is 1,211 rows.

- disposition manifest: `formula-vault/resolutions/m07_vector_helper_wave2.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- validation status: `research_required`;
- new validation cards or source seeds: none.

## A14 external classical two-body algebra Wave 1

A14 processes the first 40 rows in `8E_or_9A_classical_two_body_algebra_contracts` without adding a formula node or Rust kernel. Sixteen exact-name aliases reuse five existing governed A7 formulas: circular speed, circular period, escape speed, vis-viva speed, and mean motion. Twenty-four rows remain contract-blocked because source aliases alone do not establish the required input-shape, conic-branch, reference-radius, or inverse-relation contracts. Nine rows remain for a later wave, and the classifier risk tier remains `medium_risk_requires_contract_review`.

- disposition manifest: `formula-vault/resolutions/m07_two_body_algebra_wave1.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions after A14: 152;
- remaining external backlog after A14: 1,171;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A15 external classical two-body algebra Wave 2

A15 processes the remaining 9 rows in `8E_or_9A_classical_two_body_algebra_contracts` without adding a formula node or Rust kernel. Six exact aliases reuse existing governed A7 circular-speed, escape-speed, vis-viva, or mean-motion runtimes. Two specific-energy rows and one AU/TU mean-motion row remain contract-blocked. A14-A15 now cover all 49 rows in the group while preserving `medium_risk_requires_contract_review`.

- disposition manifest: `formula-vault/resolutions/m07_two_body_algebra_wave2.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 161;
- remaining external backlog: 1,162;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A16 external orbital-geometry and conic-branch Wave 1

A16 processes the first 40 rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. Two exact aliases reuse the governed A7 specific-angular-momentum and eccentricity-vector runtimes. Ten generic math, parameter-lookup, force/acceleration, or state-derivative helpers are excluded from formula scope. Twenty-eight rows remain contract-blocked pending explicit geometry, frame, angle, conic-branch, apsis/ellipse, parabolic, or hyperbolic mission-design contracts. The classifier risk tier remains `medium_risk_requires_contract_review`, and 337 rows remain in the group.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave1.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 201;
- remaining external backlog: 1,122;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A17 external orbital-geometry and conic-branch Wave 2

A17 processes the next 40 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. Three exact aliases reuse governed A7 specific-angular-momentum, eccentricity-vector, and node-vector runtimes. Fifteen generic math, state/element conversion, orbit-determination, or composite-summary helpers are excluded from formula scope. Twenty-two rows remain contract-blocked. The selected risk tiers remain unchanged: 38 medium-risk contract-review rows and 2 high-risk numerical-policy rows. A16-A17 now cover 80 rows, leaving 297 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave2.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 241;
- remaining external backlog: 1,082;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A18 external orbital-geometry and conic-branch Wave 3

A18 processes the third 40 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. One exact alias reuses the governed A7 eccentricity-vector runtime. Seven generic math or composite maneuver helpers are excluded from formula scope. Thirty-two rows remain contract-blocked. The selected risk tiers remain unchanged: 33 medium-risk contract-review rows and 7 high-risk numerical-policy rows. A16-A18 now cover 120 rows, leaving 257 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave3.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 281;
- remaining external backlog: 1,042;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A19 external orbital-geometry and conic Wave 4

A19 processes the fourth 40 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. Three exact aliases reuse governed A7 runtimes, eight internal/composite helpers are excluded from formula scope, and twenty-nine rows remain contract- or policy-blocked. The selected risk tiers remain unchanged: 37 medium-risk contract-review rows and 3 high-risk numerical-policy rows. A16-A19 now cover 160 rows, leaving 217 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave4.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 321;
- remaining external backlog: 1,002;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A20 external orbital-geometry and conic Wave 5

A20 processes the fifth 40 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. No exact runtime alias is asserted, ten internal/intermediate or composite helpers are excluded from formula scope, and thirty rows remain contract- or policy-blocked. The selected risk tiers remain unchanged: 26 medium-risk contract-review rows and 14 high-risk numerical-policy rows. A16-A20 now cover 200 rows, leaving 177 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave5.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 361;
- remaining external backlog: 962;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A21 external orbital-geometry and conic Wave 6

A21 processes the sixth 40 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. One exact alias reuses the governed A7 eccentricity-vector runtime, thirteen internal/composite helpers are excluded from formula scope, and twenty-six rows remain contract- or policy-blocked. The selected risk tiers remain unchanged: 18 medium-risk contract-review rows and 22 high-risk numerical-policy rows. A16-A21 now cover 240 rows, leaving 137 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave6.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 401;
- remaining external backlog: 922;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A22 external orbital-geometry and conic Wave 7

A22 processes the seventh 40 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. One exact alias reuses the governed A7 sphere-of-influence runtime, seventeen internal/composite helpers are excluded from formula scope, and twenty-two rows remain contract- or policy-blocked. The selected risk tiers remain unchanged: 38 medium-risk contract-review rows and 2 high-risk numerical-policy rows. A16-A22 now cover 280 rows, leaving 97 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave7.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 441;
- remaining external backlog: 882;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A23 external orbital-geometry and conic Wave 8

A23 processes the eighth 40 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. One exact alias reuses the governed A7 sphere-of-influence runtime, ten internal/composite helpers are excluded from formula scope, and twenty-nine rows remain contract- or policy-blocked. The selected risk tiers remain unchanged: 30 medium-risk contract-review rows and 10 high-risk numerical-policy rows. A16-A23 now cover 320 rows, leaving 57 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave8.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 481;
- remaining external backlog: 842;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A24 external orbital-geometry and conic Wave 9

A24 processes the ninth 40 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. All 40 rows remain contract- or policy-blocked pending explicit orbit-geometry/conic branch conventions, frame/unit contracts, numerical policy, source registry, and independent validation oracles. The selected risk tiers remain unchanged: 34 medium-risk contract-review rows and 6 high-risk numerical-policy rows. A16-A24 now cover 360 rows, leaving 17 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave9.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 521;
- remaining external backlog: 802;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A25 external orbital-geometry and conic Wave 10

A25 processes the final 17 source-ordered rows in `9A_classical_elements_and_9E_mission_design_contracts` without adding a formula node or Rust kernel. All 17 rows remain contract- or policy-blocked pending explicit orbit-geometry/conic branch conventions, frame/unit contracts, numerical policy, source registry, and independent validation oracles. The selected risk tiers remain unchanged: 14 medium-risk contract-review rows and 3 high-risk numerical-policy rows. A16-A25 now cover all 377 rows in the group, leaving 0 group rows.

- disposition manifest: `formula-vault/resolutions/m07_orbital_geometry_conic_wave10.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 538;
- remaining external backlog: 785;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A26 external coordinate-transform / frame-graph / time-scale policy Wave 1

A26 processes the first 40 source-ordered rows in the governed 9B coordinate-transform, frame-graph, and time-scale policy backlog without adding a formula node or Rust kernel. All 40 rows remain contract- or policy-blocked pending explicit frame/sign/rotation-order contracts, epoch/time-scale and sidereal policy, source registry, and independent validation oracles. The selected risk tiers remain unchanged: 29 medium-risk contract-review rows and 11 frame/time-policy blocked rows. This leaves 45 rows in the 9B candidate pool.

- disposition manifest: `formula-vault/resolutions/m07_coordinate_transform_frame_time_policy_wave1.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 578;
- remaining external backlog: 745;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.

## A27 external coordinate-transform / frame-graph / time-scale policy Wave 2

A27 processes the remaining 45 source-ordered rows in the governed 9B coordinate-transform, frame-graph, and time-scale policy backlog without adding a formula node or Rust kernel. All 45 rows remain contract- or policy-blocked pending explicit frame/sign/rotation-order contracts, epoch/time-scale and sidereal policy, relative-frame conventions, source registry, and independent validation oracles. The selected risk tiers remain unchanged: 29 medium-risk contract-review rows and 16 frame/time-policy blocked rows. This leaves 0 rows in the 9B candidate pool.

- resolution manifest: `formula-vault/resolutions/m07_coordinate_transform_frame_time_policy_wave2.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 623;
- remaining external backlog: 700;
- no runtime, source-seed, validation-card, parity, certification, or operational-readiness claim is made.


## A28 external solver / least-squares / root-selection policy Wave 1

A28 processes the first 40 source-ordered rows in the governed 9C solver-policy backlog without adding a formula node or Rust kernel. All 40 rows remain contract- or policy-blocked pending explicit iteration/root-selection, rank/tolerance, convergence/failure-state, source-registry, and independent numerical-oracle policies. The selected risk tiers remain unchanged: 40 solver-policy blocked rows. This leaves 83 rows in the solver-policy candidate pool.

- resolution manifest: `formula-vault/resolutions/m07_solver_policy_wave1.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 663;
- remaining external backlog: 660;
- validation status: `research_required`;
- no raw Rust-port, M07, or Scilab source review or parity claim.


## A29 external solver / numerical propagation policy Wave 2

A29 processes the second 40 source-ordered rows in the governed 9C/10B solver-policy backlog without adding a formula node or Rust kernel. All 40 rows remain contract- or policy-blocked pending explicit iteration, root-selection, integration tolerance, convergence/failure-state, source-registry, and independent numerical-oracle policies. The selected risk tiers remain unchanged: 40 solver-policy blocked rows. This leaves 43 rows in the solver-policy candidate pool.

- resolution manifest: `formula-vault/resolutions/m07_solver_policy_wave2.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 703;
- remaining external backlog: 620;
- no M07/Scilab parity, certification, operational-readiness, or public api claim is made.

## A30 external solver / numerical propagation policy Wave 3

A30 processes the remaining 43 source-ordered rows in the governed 9C solver-policy backlog without adding a formula node or Rust kernel. All 43 rows remain contract- or policy-blocked pending explicit iteration, root-selection, integration tolerance, convergence/failure-state, source-registry, and independent numerical-oracle policies. The selected risk tiers remain unchanged: 43 solver-policy blocked rows. This closes the solver-policy candidate pool.

- resolution manifest: `formula-vault/resolutions/m07_solver_policy_wave3.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 746;
- remaining external backlog: 577;
- no runtime implementation, parity, certification, or operational-readiness claim is made.


## A31 external relative-motion and finite-burn scalar policy Wave 1

A31 processes the first 40 source-ordered rows in the governed relative-motion and finite-burn scalar policy backlog without adding a formula node or Rust kernel. All 40 rows remain contract- or policy-blocked pending explicit relative-frame, finite-burn, rocket-vehicle, unit-domain, source-registry, and independent validation-oracle policies. The selected risk tiers remain unchanged: 19 frame/time-policy blocked rows, 19 high-risk numerical-policy rows, and 2 medium-risk contract-review rows.

- resolution manifest: `formula-vault/resolutions/m07_relative_motion_finite_burn_policy_wave1.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 786;
- remaining external backlog: 537;
- no runtime implementation, parity, certification, or operational-readiness claim is made.

## A32 external relative-motion and finite-burn scalar policy Wave 2

A32 processes the second 40 source-ordered rows in the governed relative-motion and finite-burn scalar policy backlog without adding a formula node or Rust kernel. All 40 rows remain contract- or policy-blocked pending explicit relative-frame, finite-burn, rocket-vehicle, unit-domain, source-registry, and independent validation-oracle policies. The selected risk tiers remain unchanged: 10 frame/time-policy blocked rows, 23 high-risk numerical-policy rows, and 7 medium-risk contract-review rows.

- resolution manifest: `formula-vault/resolutions/m07_relative_motion_finite_burn_policy_wave2.tsv`;
- public verification: `cargo run -p xtask -- verify formula-vault`;
- cumulative external terminal dispositions: 826;
- remaining external backlog: 497;
- no runtime implementation, parity, certification, or operational-readiness claim is made.


## A33 external relative-motion and finite-burn scalar policy Wave 3

A33 closes the governed relative-motion and finite-burn scalar policy backlog by processing the final 29 source-ordered rows without adding a formula node or Rust kernel. All 29 rows remain contract- or policy-blocked pending explicit finite-burn, rocket-vehicle, unit-domain, source-registry, and independent validation-oracle policies. The selected risk tiers remain unchanged: 28 high-risk numerical-policy rows and 1 medium-risk contract-review row.

A33 records:

- selected classifier rows: 29;
- exact aliases: 0;
- helper exclusions: 0;
- contract or policy blocks: 29;
- cumulative external terminal dispositions: 855;
- remaining external backlog: 468;
- relative-motion and finite-burn candidate-pool tail after A33: 0.

- `formula-vault/resolutions/m07_attitude_frame_policy_wave1.tsv` records A34 metadata-only terminal dispositions for 40 attitude representation / inertia / quaternion policy rows.

- `formula-vault/resolutions/m07_attitude_frame_policy_wave2.tsv` records A35 metadata-only terminal dispositions for the remaining 19 attitude representation / inertia / quaternion policy rows.

- `formula-vault/resolutions/m07_attitude_dynamics_control_policy_wave1.tsv` records A36 metadata-only terminal dispositions for 38 attitude dynamics/control policy rows.

- `formula-vault/resolutions/m07_j2_perturbation_policy_wave1.tsv` records A37 metadata-only terminal dispositions for the first bounded J2 perturbation / numerical propagation slice.

- `formula-vault/resolutions/m07_j2_perturbation_policy_wave2.tsv` records A38 metadata-only terminal dispositions for the second bounded J2 perturbation / numerical propagation slice.
- `formula-vault/resolutions/m07_j2_perturbation_policy_wave3.tsv` records A39 metadata-only terminal dispositions for the remaining J2 perturbation / numerical propagation policy slice, closing that governed candidate pool.

- `formula-vault/resolutions/m07_sgp4_teme_policy_wave1.tsv` records A40 metadata-only terminal dispositions for the SGP4 / TEME frame-time policy and helper-exclusion candidate pool, closing that governed candidate pool.
- `formula-vault/resolutions/m07_cr3bp_external_data_policy_wave1.tsv` records A41 metadata-only terminal dispositions for CR3BP family/oracle policy, external-data fixture governance, and input/output demonstration-row exclusions.
### A42 classifier-refresh / manual source-review policy Wave 1

Records 45 research-required terminal dispositions in `m07_classifier_refresh_manual_source_review_wave1.tsv`. No runtime formulas, source scraping, or parity claims are introduced. External M07 processed/backlog after A42: 1215/108.

### A43 scalar/unit helper policy Wave 1

Records 45 research-required terminal dispositions in `m07_scalar_unit_helper_policy_wave1.tsv`. No runtime formulas, source scraping, or parity claims are introduced. External M07 processed/backlog after A43: 1260/63.

### A44 residual scalar/unit/helper policy Wave 1

Records 45 research-required terminal dispositions in `m07_residual_scalar_unit_helper_policy_wave1.tsv`. Helper/test utilities remain excluded, and angle/unit rows remain policy blocked. External M07 processed/backlog after A44: 1305/18.

### A45 final residual backlog closure Wave 1

Records 18 research-required terminal dispositions in `m07_final_residual_backlog_closure_wave1.tsv`. No runtime formulas, source scraping, or parity claims are introduced. External M07 processed/backlog after A45: 1323/0.
