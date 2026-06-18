# Orekit Reference-Oracle Plan

Stage 6 prep extends the existing Stage 4 Orekit reference-oracle boundary into a concrete, family-level validation plan. This document is planning and validation metadata only. It does not import Orekit source code, does not add Java or Rust bindings, does not generate fixtures, does not copy an external class hierarchy, and does not add public AeroCodex astrodynamics APIs.

AeroCodex remains research and preliminary-design software. This plan is not certification evidence, not operational approval, not flight readiness, not mission readiness, not habitat-safety approval, not medical-use approval, and not regulated-use approval.

## Relationship to the current Stage 4 boundary

The repository already records Orekit as an external Apache-2.0 reference artifact under source ID `stage4.orekit_reference.2026_06_15`, with the archive intentionally outside the repo. The existing boundary allows reference-oracle planning, family-level architecture comparison, test-family selection, and AeroCodex-native documentation vocabulary only.

This Stage 6 plan adds a deployment-ready map of what future AeroCodex checks may compare against Orekit-derived outputs. It intentionally stays at the family and evidence-contract level.

## Non-goals

- No Java execution harness is added.
- No Orekit source, comments, tests, fixture files, generated logs, or external data are imported.
- No Rust wrapper, foreign-function interface, subprocess integration, or build dependency is added.
- No AeroCodex module is shaped to mirror an external package tree.
- No validation status is promoted beyond `research_required`.
- No operational, mission, safety, medical, certification, or regulated-use claim is made.

## Oracle evidence record required before future comparisons

Every future Orekit-backed comparison must create a slice-specific evidence record before expected values are trusted. The record must include:

1. AeroCodex capability, equation, parser, or policy ID under test.
2. AeroCodex source IDs plus `stage4.orekit_reference.2026_06_15` as an external oracle source.
3. Orekit archive hash, archive date, and generation environment.
4. Scenario inputs with units, frames, time scale, epoch, domain, and assumptions.
5. Expected output quantities with units and sign conventions.
6. Absolute and relative tolerance strategy with rationale.
7. Fixture or generated-output manifest and SHA256, if any expected-value artifact is kept.
8. Procedure or command used outside the repo to generate the oracle output.
9. Boundary statement confirming no source text, comments, class hierarchy, or implementation structure were copied.
10. Validation status, reviewer notes, and explicit research/preliminary-design caveat.

## Oracle family plan

| Oracle family | AeroCodex capability under test | Orekit comparison role | Units and frames | Tolerance strategy | Fixture/hash requirement | Blocked prerequisites | Boundary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Time scale policies | Future AeroCodex epoch/time-scale policy contracts | Independent reference for epoch normalization and time-scale offset scenarios | Inputs: calendar-like epochs, elapsed seconds; outputs: normalized epochs and offsets in seconds | Exact where integer offset policy applies; otherwise explicit absolute tolerance in seconds chosen by future contract | Required for any external expected-value table, including leap-second source hash | Time-scale policy, leap-second source governance, fixture manifest | Compare outputs only; do not copy time-system implementation or data-loader design |
| Frame graph policies | Future frame graph and transform contracts | Independent reference for named-frame transform scenarios | Inputs: epoch, frame names, position in meters, velocity in meters per second; outputs: transform quantities and transformed state | Blocked until frame accuracy budget and data-source policy are set | Required for any Earth orientation or transform fixture | Time-scale policy, frame naming policy, data-source registry | Keep AeroCodex frame graph native; no external class hierarchy or package tree |
| TLE parsing | Future TLE field parser contract | Reference for field interpretation and epoch normalization | Inputs: two text lines; outputs: parsed scalar fields, angles, rates, epoch | Exact for textual fields; numeric tolerances stated per parsed quantity | Required; TLE examples and expected parsed fields need license/provenance/hash | TLE fixture licensing, parser field policy, time-scale policy | Parse contract only; do not copy parser code or test bodies |
| SGP4/TEME | Future SGP4/TEME state comparison lane | Independent state-vector oracle for registered TLE scenarios | Inputs: TLE, elapsed time in minutes or seconds; outputs: TEME position meters and velocity meters per second | Explicit absolute and relative state tolerances chosen before implementation review | Required for each scenario output table | TLE parser policy, TEME frame policy, time-scale policy, fixture manifest | Compare state outputs only; no source translation or binding |
| Two-body propagation | Existing/future two-body research kernels and state propagator contracts | Independent sanity oracle for simple Keplerian or Cartesian scenarios | Inputs: central-body gravitational parameter in m^3/s^2, state in m and m/s, duration in seconds; outputs: state or orbital scalars | Analytical double-precision tolerance; future evidence record must define singularity cases | Optional if analytical closed-form evidence is sufficient; required if external expected table is kept | Scenario contract and singularity policy | Do not infer architecture from the oracle; keep AeroCodex formulas independently derived |
| Hohmann transfer | Existing Hohmann transfer research helpers and future formula-vault contracts | Cross-check for circular coplanar transfer scalars | Inputs: radii in meters and gravitational parameter in m^3/s^2; outputs: delta-v in m/s and transfer time in seconds | Tight analytical scalar tolerance; future record must state circular/coplanar assumptions | Optional if analytical test vectors are generated independently; required for any external oracle CSV | Unit/sign convention confirmation | Formula-level comparison only; no operational trajectory-planning claim |
| J2 later | Future J2 secular-rate or perturbation contracts | Later external reference for perturbation scenario families | Inputs: orbital elements, central constants, duration; outputs: rates or state deltas | Blocked until perturbation tolerance and constants policy exist | Required for all external expected values | Constants registry, frame/time policy, perturbation model scope | Later lane only; no force-model architecture cloning |
| Event checks later | Future event-detection test-harness design | Later reference for event timing and predicate behavior | Inputs: state/epoch sequence and predicate definition; outputs: event time, event flag, convergence notes | Blocked until root-finding and non-convergence policy exist | Required for generated event timing fixtures | Solver/root-bracketing policy and frame/time policy | Compare event outcomes only; do not copy event detector design |
| Coordinate transforms later | Future coordinate/orbit transform contracts | Later reference for transform output and round-trip scenarios | Inputs: position/state/orbital elements with declared frames and units; outputs: transformed quantities | Tolerance depends on singularity policy and representation pair | Required for nontrivial transform fixture tables | Singularity policy, frame/time policy, representation conventions | No one-for-one transform class or method mapping |

## Recommended promotion gates

A future AeroCodex astrodynamics slice may use Orekit as one reference oracle only after all of these gates are present:

- The target AeroCodex equation or capability has its own contract, source registry seed, validation card, domain, units, assumptions, and non-claim language.
- Any external expected-value fixture has a manifest, source ID, license note, generation command/procedure, and SHA256.
- The comparison uses AeroCodex-native inputs, outputs, and tolerance policy.
- The source-boundary scan confirms no external source text, comments, hierarchy, implementation expression, generated logs, or raw archive files entered the repo.
- Validation remains `research_required` unless a later review explicitly upgrades it through the normal AeroCodex status vocabulary.

## Recommended first safe slice

The first low-risk Stage 6 comparison should be scalar and frame-free: Hohmann transfer or simple two-body scalar checks. Time-scale, frame graph, TLE, SGP4/TEME, J2, event, and coordinate-transform lanes should remain blocked until their prerequisite policies and fixture manifests exist.
