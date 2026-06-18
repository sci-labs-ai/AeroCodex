# Recommended chunks 8C to 10E from the M07 classifier

This plan sequences future ingestion by risk. It is a planning document only. It does not authorize implementation, API promotion, validation-status promotion, source import, certification claims, Stage 5 Chunk 0, BioSim deployment, or Orekit deployment.

## Chunk 8C — remaining M00 unit/conversion review

Scope: the remaining low-risk or medium-low M00 unit/conversion rows not already represented by the current formula-vault metadata candidates.

Recommended rows include the `app_to_*` / `app_from_*` canonical distance, velocity, and time conversion helpers after an explicit unit contract verifies distance-unit, time-unit, velocity-unit, distance-unit-equivalent, and time-unit-equivalent conventions. `app_constants`, `app_canonical_units_from_mu_du`, and `app_ut_day_fraction` should stay contract-first because they depend on constant-source and time/fraction policy.

Do not include `wrap2pi` or `app_resolve_coplanar` in 8C.

## Chunk 8D — duplicate scalar/vector helper deduplication policy

Scope: module-local helper rows such as `dot`, `cross`, `norm`, `unit`, `clamp`, `safe_sqrt`, `deg2rad`, and `rad2deg` that appear repeatedly across M01–M06.

Recommended action: create a policy that decides whether these rows are provenance aliases to existing central M00 helper kernels or separate formula-vault candidates. Prefer aliasing/deduplication over public API multiplication.

Blocked until: endpoint policy for all wrap-to-pi/two-pi variants and zero-vector/domain policy for normalization variants.

## Chunk 8E — classical two-body algebra contracts

Scope: low-to-medium risk algebraic formulas from classical M01/M02/M03 families, such as circular speed, escape speed, vis-viva speed, mean motion, period, specific energy, semimajor-axis relations, and periapsis/apoapsis radius relations.

Required before implementation: gravitational-parameter units, inertial-frame assumptions, conic branch conventions, invalid-region definitions, and analytical test-vector tables.

## Chunk 9A — orbital elements and conic branch policy

Scope: state-to-elements, elements-to-state, eccentricity vector, node vector, conic classification, anomaly conversions, and true/eccentric/hyperbolic anomaly relations.

Blockers: circular/equatorial singularities, parabolic boundary policy, angle wrapping, sign conventions, and test oracle selection.

## Chunk 9B — coordinate transform, frame graph, and time-scale policy

Scope: perifocal-to-inertial transforms, topocentric-horizon-to-inertial transforms, right-ascension and declination, azimuth/elevation, earth-fixed frames, sidereal time, station vectors, and local-time or universal-time helpers.

Blockers: frame registry, rotation order, handedness, time scale, sidereal epoch, geodetic/geocentric policy, and round-trip tolerances.

## Chunk 9C — solver, least-squares, and root-selection policy

Scope: Kepler solvers, Lambert/Gauss solvers, Laplace optical, weighted least squares, numerical Jacobians, universal variables, bisection/Newton/root selection, and source-equivalence fixtures.

Blockers: iteration limits, convergence failure states, rank/singularity policy, root ordering, finite-difference step size, and tolerance recording.

## Chunk 9D — relative motion, CW, and LVLH policy

Scope: Clohessy-Wiltshire state transition, relative state components, LVLH frames, rendezvous helpers, and linearized relative motion.

Blockers: target/chaser ordering, circular-reference-orbit assumptions, LVLH frame axes, secular drift definitions, and validation fixture policy.

## Chunk 9E — rocket vehicle / finite-burn scalar subset

Scope: bounded scalar formulas only, such as mass ratio, ideal delta-v, propellant fraction, burn time, and thrust-to-weight after domain review.

Keep blocked: gravity-turn propagation, staging optimization, low-thrust spiral propagation, Runge-Kutta-Fehlberg and Runge-Kutta integrators, and trajectory summaries until the numerical policy exists.

## Chunk 10A — attitude quaternion/DCM/inertia policy

Scope: quaternion/DCM conversions, axis-angle, quaternion normalization, inertia tensors, parallel-axis theorem, attitude kinematics, and control primitives.

Blockers: quaternion scalar/vector order, sign equivalence, DCM active/passive convention, inertia positive-definite policy, saturation/control assumptions, and dynamic integration policy.

## Chunk 10B — J2, perturbations, and numerical propagation policy

Scope: J2 secular rates, perturbation accelerations, Cowell/Encke propagation, atmospheric drag models, third-body/lunar-solar perturbation helpers, and state-history conversions.

Blockers: model constants, inertial frame convention, step-size/tolerance policy, reference oracles, fixture hashes, and no flight/mission-readiness claims.

## Chunk 10C — SGP4/TEME hold

Scope: M06 Vallado SGP4/TLE/TEME rows.

Keep blocked until the dedicated SGP4/TEME frame/time policy and reference oracle exist. Do not ingest internal parsing/string helpers as public formula APIs.

## Chunk 10D — external data, tables, fixtures, and demo rows

Scope: planet tables, Bode tables, sample datasets, site-track datasets, report/plot/print helpers, and other non-formula rows.

Required action: establish data-registry hash policy and mark I/O/demo rows as provenance-only or do-not-import. Do not add generated outputs or external source archives.

## Chunk 10E — CR3BP and classifier refresh

Scope: CR3BP rows and any family not adequately covered by the current taxonomy.

Required action: decide whether to add a dedicated CR3BP family to the classifier vocabulary. Until then, CR3BP rows remain blocked under solver/coordinate/blocked-policy risk tiers.
