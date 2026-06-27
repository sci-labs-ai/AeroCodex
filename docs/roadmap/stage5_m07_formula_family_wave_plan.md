# Stage 5 M07 formula-family wave plan

This Stage 5 Session C candidate sequences future M07 formula-family review by risk. It is planning material only. It does not authorize implementation, formula-vault promotion, API promotion, validation-status promotion, source import, fixture import, or certification claims.

The group labels below are classifier planning keys. They are not live chunk authorization and they do not supersede the Stage 5 deployment queue.

## Wave 1 - remaining M00 unit/conversion review

Scope: low-risk or medium-low M00 unit/conversion rows not already represented by current formula-vault metadata candidates.

Recommended rows include canonical distance, velocity, and time conversion helpers only after an explicit unit contract verifies canonical unit conventions. Keep constants, canonical-unit derivation, day-fraction helpers, angle endpoint behavior, and `app_resolve_coplanar` contract-first.

## Wave 2 - helper deduplication policy

Scope: repeated module-local scalar/vector helpers such as dot product, cross product, norm, unit-vector, clamp, safe square root, degrees-to-radians, and radians-to-degrees helpers.

Recommended action: decide whether each row is a provenance alias to existing central M00 helper kernels or a separate metadata candidate. Prefer aliasing and deduplication over public API multiplication.


### Post-Stage-5 A11 execution overlay

A11 executes the first bounded external-backlog wave against the low-risk `8D_deduplicated_unit_conversion_helpers` subset. It assigns terminal dispositions to 38 rows: 37 are provenance aliases to existing governed M00 conversion runtimes and one `earth_rotation_rate_canonical` row remains blocked pending an angular-rate contract and runtime. The wave adds no formula node or Rust kernel and reduces the unprocessed external backlog from 1,323 to 1,285 rows.

### Post-Stage-5 A12 execution overlay

A12 executes the first 40 rows, in source-row order, from `8D_helper_deduplication_then_low_risk_vector_contracts`. Thirty rows become provenance aliases to existing governed M00 dot, cross, norm, unit-vector, or vector-angle runtimes; eight shape/identity utilities are excluded from formula scope; determinant-column and latitude/longitude unit-vector rows remain contract-blocked. The wave adds no formula node or Rust kernel and reduces the unprocessed external backlog from 1,285 to 1,245 rows.

## Wave 3 - classical two-body algebra contracts

Scope: low-to-medium risk algebraic formulas from classical astrodynamics families, including circular speed, escape speed, vis-viva speed, mean motion, period, specific energy, semimajor-axis relations, and periapsis/apoapsis radius relations.

Required before implementation: gravitational-parameter units, inertial-frame assumptions, conic branch conventions, invalid-region definitions, and analytical test-vector tables.

## Wave 4 - orbital elements and conic branch policy

Scope: state-to-elements, elements-to-state, eccentricity vector, node vector, conic classification, anomaly conversions, and true/eccentric/hyperbolic anomaly relations.

Blockers: circular/equatorial singularities, parabolic boundary policy, angle wrapping, sign conventions, and test-oracle selection.

## Wave 5 - coordinate transform, frame graph, and time-scale policy

Scope: perifocal/inertial, local-frame/inertial, right-ascension/declination, azimuth/elevation, earth-fixed, sidereal time, station vectors, and local/time helpers.

Blockers: frame registry, rotation order, handedness, time scale, sidereal epoch, geodetic/geocentric policy, and round-trip tolerances.

## Wave 6 - solver, least-squares, and root-selection policy

Scope: Kepler solvers, Lambert/Gauss solvers, optical observation solvers, weighted least squares, numerical Jacobians, universal variables, and root-selection helpers.

Blockers: iteration limits, convergence failure states, rank/singularity policy, root ordering, finite-difference step size, and tolerance recording. `app_resolve_coplanar` remains blocked until this policy exists.

## Wave 7 - relative motion and finite-burn scalar subsets

Scope: relative-motion state transitions, local orbital frames, rendezvous helpers, bounded rocket scalar formulas, ideal delta-v, propellant fraction, burn time, and thrust-to-weight after domain review.

Keep trajectory propagation, staging optimization, gravity turns, low-thrust propagation, and numerical integrators blocked until numerical-policy gates exist.

## Wave 8 - attitude, perturbation, propagator-frame, external-data, and restricted-three-body holds

Scope: attitude conversions and dynamics, J2/perturbations, numerical propagation, propagator/frame-specific rows, external tables/fixtures/demo rows, and restricted-three-body rows.

Required action: keep these families policy-gated until dedicated source, frame/time, oracle, fixture, and data-governance reviews exist. Do not ingest internal parsing/string helpers or demo/report/plot rows as public formula APIs.

## Non-claims

This wave plan by itself does not promote any row. The later A11 overlay records metadata-only terminal dispositions, not formula implementations. It adds no validation card, source-registry seed, formula-vault candidate, executable research equation, helper algorithm, source archive, fixture, public API, generated equation code, or operational evidence. Every later modifying slice requires a separate prompt, live main sync, patch preflight, governed count verification, local gates, and exact-commit CI.

### Post-Stage-5 A13 execution overlay

A13 executes the remaining 34 rows, in source-row order, from `8D_helper_deduplication_then_low_risk_vector_contracts`. Twenty-six rows become provenance aliases to existing M00 vector-angle, cross, dot, norm, or unit-vector runtimes; five column-shape utilities are excluded from formula scope; two skew-matrix rows and one true-anomaly-from-r/rdot row remain contract-blocked. A12-A13 now cover all 74 rows in the vector-helper group. A13 adds no formula node or Rust kernel and reduces the unprocessed external backlog from 1,245 to 1,211 rows.

## A14 classical two-body algebra Wave 1

A14 processes the first 40 source-ordered rows from `8E_or_9A_classical_two_body_algebra_contracts`. Sixteen exact-name aliases reuse existing governed A7 circular-speed, circular-period, escape-speed, vis-viva, or mean-motion runtimes. Twenty-four rows remain contract-blocked because the alias alone does not establish a safe exact mapping for specific-energy input form, semilatus rectum, conic radius, apsis geometry, altitude/reference radius, general semimajor-axis period, or inverse energy-to-axis semantics.

The classifier risk tier remains `medium_risk_requires_contract_review`; A14 does not relabel these rows as low risk. Nine rows remain in the 49-row group for a later bounded wave. A14 adds no formula node or Rust kernel and reduces the unprocessed external backlog from 1,211 to 1,171 rows.

## A15 classical two-body algebra Wave 2

A15 processes the remaining 9 source-ordered rows from `8E_or_9A_classical_two_body_algebra_contracts`. Six exact aliases reuse existing governed A7 circular-speed, escape-speed, vis-viva, or mean-motion runtimes. Two specific-energy rows remain blocked because the alias does not establish the input form, and the AU/TU mean-motion row remains blocked pending an explicit astronomical-unit/time-unit input, output, and scaling contract.

A14-A15 now cover all 49 rows in the group with 22 aliases and 27 contract blocks. The classifier risk tier remains `medium_risk_requires_contract_review`. A15 adds no formula node or Rust kernel and reduces the unprocessed external backlog from 1,171 to 1,162 rows.

## A16 orbital geometry and conic branch Wave 1

A16 processes the first 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`. Two exact aliases reuse the governed A7 specific-angular-momentum and eccentricity-vector runtimes. Ten generic math, parameter lookup, vector-force/acceleration, or state-derivative helpers are excluded from formula scope. Twenty-eight rows remain contract-blocked pending explicit state/frame/angle, conic-branch, apsis/ellipse, parabolic, or hyperbolic mission-geometry contracts.

The classifier risk tier remains `medium_risk_requires_contract_review`; 337 rows remain in the group. A16 adds no formula node or Rust kernel and reduces the unprocessed external backlog from 1,162 to 1,122 rows.

### Post-Stage-5 A17 overlay

A17 processes the next 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 3 exact aliases reuse governed A7 runtimes, 15 internal/composite support helpers are excluded from formula scope, and 22 rows remain contract-blocked. The selected rows retain 38 `medium_risk_requires_contract_review` and 2 `high_risk_requires_numerical_policy` labels. A16-A17 cover 80 rows, leave 297 group rows, and update external accounting to 241 terminally processed rows and 1,082 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A18 overlay

A18 processes the next 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 1 exact alias reuses the governed A7 eccentricity-vector runtime, 7 internal/composite maneuver helpers are excluded from formula scope, and 32 rows remain contract-blocked. The selected rows retain 33 `medium_risk_requires_contract_review` and 7 `high_risk_requires_numerical_policy` labels. A16-A18 cover 120 rows, leave 257 group rows, and update external accounting to 281 terminally processed rows and 1,042 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A19 overlay

A19 processes the fourth 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 3 exact aliases reuse governed A7 runtimes, 8 internal/composite or universal-variable helpers are excluded from formula scope, and 29 rows remain contract- or policy-blocked. The selected rows retain 37 `medium_risk_requires_contract_review` and 3 `high_risk_requires_numerical_policy` labels. A16-A19 cover 160 rows, leave 217 group rows, and update external accounting to 321 terminally processed rows and 1,002 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A20 overlay

A20 processes the fifth 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: no exact runtime alias is asserted, 10 internal/intermediate or composite helpers are excluded from formula scope, and 30 rows remain contract- or policy-blocked. The selected rows retain 26 `medium_risk_requires_contract_review` and 14 `high_risk_requires_numerical_policy` labels. A16-A20 cover 200 rows, leave 177 group rows, and update external accounting to 361 terminally processed rows and 962 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A21 overlay

A21 processes the sixth 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 1 exact alias reuses the governed A7 eccentricity-vector runtime, 13 internal/composite helpers are excluded from formula scope, and 26 rows remain contract- or policy-blocked. The selected rows retain 18 `medium_risk_requires_contract_review` and 22 `high_risk_requires_numerical_policy` labels. A16-A21 cover 240 rows, leave 137 group rows, and update external accounting to 401 terminally processed rows and 922 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A22 overlay

A22 processes the seventh 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 1 exact alias reuses the governed A7 sphere-of-influence runtime, 17 internal/composite helpers are excluded from formula scope, and 22 rows remain contract- or policy-blocked. The selected rows retain 38 `medium_risk_requires_contract_review` and 2 `high_risk_requires_numerical_policy` labels. A16-A22 cover 280 rows, leave 97 group rows, and update external accounting to 441 terminally processed rows and 882 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A23 overlay

A23 processes the eighth 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 1 exact alias reuses the governed A7 sphere-of-influence runtime, 10 internal/composite helpers are excluded from formula scope, and 29 rows remain contract- or policy-blocked. The selected rows retain 30 `medium_risk_requires_contract_review` and 10 `high_risk_requires_numerical_policy` labels. A16-A23 cover 320 rows, leave 57 group rows, and update external accounting to 481 terminally processed rows and 842 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A24 overlay

A24 processes the ninth 40 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 0 exact aliases, 0 helper exclusions, and 40 rows remain contract- or policy-blocked. The selected rows retain 34 `medium_risk_requires_contract_review` and 6 `high_risk_requires_numerical_policy` labels. A16-A24 cover 360 rows, leave 17 group rows, and update external accounting to 521 terminally processed rows and 802 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A25 overlay

A25 processes the final 17 source-ordered rows from `9A_classical_elements_and_9E_mission_design_contracts`: 0 exact aliases, 0 helper exclusions, and 17 rows remain contract- or policy-blocked. The selected rows retain 14 `medium_risk_requires_contract_review` and 3 `high_risk_requires_numerical_policy` labels. A16-A25 cover all 377 rows in the group, leave 0 group rows, and update external accounting to 538 terminally processed rows and 785 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A26 overlay

A26 processes the first 40 source-ordered rows from the governed coordinate-transform / frame-graph / time-scale policy backlog: 0 exact aliases, 0 helper exclusions, and 40 rows remain contract- or policy-blocked. The selected rows span `9B_coordinate_transform_contracts_after_frame_policy` (29), `9B_time_scale_and_sidereal_policy` (9), and `9B_frame_graph_time_policy_before_coordinate_transforms` (2), while retaining 29 `medium_risk_requires_contract_review` and 11 `blocked_until_frame_time_policy` labels. A26 leaves 45 rows in this 9B candidate pool and updates external accounting to 578 terminally processed rows and 745 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.

### Post-Stage-5 A27 overlay

A27 processes the remaining 45 source-ordered rows from the governed coordinate-transform / frame-graph / time-scale policy backlog: 0 exact aliases, 0 helper exclusions, and 45 rows remain contract- or policy-blocked. The selected rows span `9B_coordinate_transform_contracts_after_frame_policy` (29), `9B_frame_graph_time_policy_before_coordinate_transforms` (13), and `9B_time_scale_and_sidereal_policy` (3), while retaining 29 `medium_risk_requires_contract_review` and 16 `blocked_until_frame_time_policy` labels. A27 leaves 0 rows in this 9B candidate pool and updates external accounting to 623 terminally processed rows and 700 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.


### Post-Stage-5 A28 overlay

A28 processes the first 40 source-ordered rows from the governed solver / least-squares / root-selection policy backlog: 0 exact aliases, 0 helper exclusions, and 40 rows remain contract- or policy-blocked. The selected rows span `9C_kepler_lambert_gauss_solver_policy_or_10B_numerical_propagation_policy` (36), `9C_solver_rank_tolerance_and_observation_policy` (3), and `9C_solver_rank_tolerance_policy_before_any_promotion` (1), while retaining 40 `blocked_until_solver_policy` labels. A28 leaves 83 rows in this solver-policy candidate pool and updates external accounting to 663 terminally processed rows and 660 remaining backlog rows. No raw Rust-port, M07, or Scilab source is imported or executed.
