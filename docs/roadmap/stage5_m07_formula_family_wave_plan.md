# Stage 5 M07 formula-family wave plan

This Stage 5 Session C candidate sequences future M07 formula-family review by risk. It is planning material only. It does not authorize implementation, formula-vault promotion, API promotion, validation-status promotion, source import, fixture import, or certification claims.

The group labels below are classifier planning keys. They are not live chunk authorization and they do not supersede the Stage 5 deployment queue.

## Wave 1 - remaining M00 unit/conversion review

Scope: low-risk or medium-low M00 unit/conversion rows not already represented by current formula-vault metadata candidates.

Recommended rows include canonical distance, velocity, and time conversion helpers only after an explicit unit contract verifies canonical unit conventions. Keep constants, canonical-unit derivation, day-fraction helpers, angle endpoint behavior, and `app_resolve_coplanar` contract-first.

## Wave 2 - helper deduplication policy

Scope: repeated module-local scalar/vector helpers such as dot product, cross product, norm, unit-vector, clamp, safe square root, degrees-to-radians, and radians-to-degrees helpers.

Recommended action: decide whether each row is a provenance alias to existing central M00 helper kernels or a separate metadata candidate. Prefer aliasing and deduplication over public API multiplication.

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

This wave plan does not promote any row out of the external M07 backlog. It does not add validation cards, source-registry seeds, formula-vault candidates, executable research equations, helper algorithms, source archives, fixtures, public APIs, generated code, or operational evidence. Every future modifying slice requires a separate prompt, live main sync, patch preflight, governed count verification, local gates, and exact-commit CI.
