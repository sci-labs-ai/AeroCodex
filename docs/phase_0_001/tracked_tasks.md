# Phase 0.001 Tracked Task Breakdown

Status: planning baseline for the merged `main` branch after Microtasks 001-020 and the thin-film BLSS extension.

This document converts the Phase 0.001 founder baseline into repo-native tracked tasks. It does not create GitHub Issues. Each task is source-grounded in the current repository documentation and preserves the AeroCodex constraints: pure Rust core, conservative validation status, source-registry/evidence-card traceability, and no certification, flight-readiness, mission-readiness, habitat-safety, life-support-readiness, or regulated-use claims.

## Grounding sources

- `docs/phase_0_001/final_microtasks_001_020_report.md` — completed Microtasks 001-020 and recommended Microtasks 021-040.
- `docs/phase_0_001/source_research_backlog.md` — source-review gaps by discipline.
- `docs/phase_0_001/microtask_log.md` — per-microtask files, checks, and conservative status notes.
- `docs/roadmap/milestones.md` — Phase 0.001 through Phase 1.0 roadmap expectations and caveats.
- `docs/phase_0_001/microtask_021_thinfilm_biofilm_models.md` — thin-film BLSS implementation scope and limitations.
- `docs/phase_0_001/thinfilm_implementation_plan.md` — thin-film BLSS solver, calibration, validation, and orchestration follow-up phases.
- `README.md` — current `for research purposes only` disclaimer and pure-Rust policy.

## Global acceptance rules

Every task below must keep these gates intact unless a task explicitly narrows the gate to docs-only work:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```

When touching thin-film BLSS artifact-tracked files, also run:

```bash
cargo run -p xtask -- verify data-registry
```

## Tasks

### AC-P0.001-T001 — Source registry review checklist

- **Title:** Add a source-registry review checklist for future promotions.
- **Objective:** Convert the validation-workflow backlog item into a repeatable checklist for reviewing source editions, equation identifiers, table/page pointers, validation values, tolerances, and licensing notes before any source-status upgrade.
- **Files/areas:** `validation/README.md`; `docs/phase_0_001/source_research_backlog.md`; optional `docs/phase_0_001/source_registry_review_checklist.md`.
- **Verification gate:** `cargo run -p xtask -- verify source-registry`; `cargo run -p xtask -- verify cards`; `cargo run -p xtask -- dependency-policy`.
- **Dependencies:** None.
- **Acceptance criteria:** Checklist exists, references the conservative status ladder, requires exact source identifiers before status upgrades, and explicitly blocks certification/flight/mission/readiness claims.

### AC-P0.001-T002 — Constants and unit-source research pass

- **Title:** Review constants provenance for Phase 0.001 scalar equations.
- **Objective:** Identify exact editions and values for NIST/CODATA constants, U.S. Standard Atmosphere constants, standard gravity, dry-air gas constant convention, and NASA/JPL astrodynamics parameters.
- **Files/areas:** `crates/aero-codex-constants/src/lib.rs`; `validation/source_registry/*constants*.yaml`; `docs/phase_0_001/source_research_backlog.md`.
- **Verification gate:** Full global Rust gate plus source-registry/card verification.
- **Dependencies:** AC-P0.001-T001.
- **Acceptance criteria:** Source-registry entries capture edition/value/unit/uncertainty notes; placeholder solar gravitational parameter remains explicitly unverified unless a reviewed source justifies promotion.

### AC-P0.001-T003 — Atmosphere source review and layer plan

- **Title:** Ground the simplified atmosphere scaffold in exact source conventions.
- **Objective:** Document U.S. Standard Atmosphere 1976 source edition, geometric/geopotential altitude convention, layer boundaries, constants, and tolerances before expanding beyond the troposphere scaffold.
- **Files/areas:** `crates/aero-codex-atmosphere/src/lib.rs`; `validation/cards/atmosphere_standard_troposphere.yaml`; `validation/source_registry/*atmosphere*.yaml`; `docs/phase_0_001/source_research_backlog.md`.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T001, AC-P0.001-T002.
- **Acceptance criteria:** Current troposphere assumptions are explicit; next-layer work has reviewed source references and test tolerances before implementation.

### AC-P0.001-T004 — Gas-dynamics source review

- **Title:** Review NACA Report 1135/equivalent gas-dynamics equation sources.
- **Objective:** Record exact equations, tables, branch conventions, angle units, inverse-solver references, and tolerances for isentropic flow, normal shocks, Mach angle, Prandtl-Meyer, and oblique shocks.
- **Files/areas:** `crates/aero-codex-gas-dynamics/src/lib.rs`; `validation/cards/gasdyn_*.yaml`; `validation/source_registry/*gas*.yaml`; `docs/phase_0_001/source_research_backlog.md`.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T001.
- **Acceptance criteria:** Each gas-dynamics helper has reviewed source notes sufficient to design reference-value tests without loosening existing conservative status rules.

### AC-P0.001-T005 — Thermodynamics and propulsion source review

- **Title:** Review perfect-gas, rocket-equation, nozzle, and choked-flow sources.
- **Objective:** Identify exact references, sign conventions, standard-gravity convention, units, station definitions, and representative examples while preserving the no-CEA-wrapper/no-native-dependency policy.
- **Files/areas:** `crates/aero-codex-thermo/src/lib.rs`; `crates/aero-codex-propulsion/src/lib.rs`; `validation/cards/thermo_*.yaml`; `validation/cards/propulsion_*.yaml`; `validation/source_registry/*thermo*.yaml`; `validation/source_registry/*propulsion*.yaml`.
- **Verification gate:** Full global Rust gate plus dependency-policy scan.
- **Dependencies:** AC-P0.001-T001, AC-P0.001-T002.
- **Acceptance criteria:** Source entries distinguish educational/reference equations from any advanced real-gas or CEA/native work; no foreign runtime or native wrapper is introduced.

### AC-P0.001-T006 — Aerodynamics and flight-dynamics source review

- **Title:** Review basic aerodynamics and flight-dynamics formula sources.
- **Objective:** Capture references and assumptions for dynamic pressure, force coefficients, induced drag, level coordinated turns, stall speed, and specific excess power.
- **Files/areas:** `crates/aero-codex-aerodynamics/src/lib.rs`; `crates/aero-codex-flight-dynamics/src/lib.rs`; `validation/cards/aerodynamics_*.yaml`; `validation/cards/flight_dynamics_*.yaml`; relevant source-registry YAML.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T001.
- **Acceptance criteria:** Reference-area, CLmax, bank-angle, sign, and station conventions are explicit; helper scope remains preliminary-design only.

### AC-P0.001-T007 — Structures and heat-transfer source review

- **Title:** Review elementary structures and heat-transfer source conventions.
- **Objective:** Document source references, equation IDs, sign conventions, axis conventions, end-condition factor usage, view-factor assumptions, and representative examples.
- **Files/areas:** `crates/aero-codex-structures/src/lib.rs`; `crates/aero-codex-heat-transfer/src/lib.rs`; validation cards/source-registry entries for structures and heat transfer.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T001.
- **Acceptance criteria:** Scalar helpers remain separate from finite-element validation, material allowables, fatigue/fracture, design-code margins, and certification evidence.

### AC-P0.001-T008 — Astrodynamics source review

- **Title:** Review two-body, Hohmann, sphere-of-influence, and parameter sources.
- **Objective:** Record gravitational-parameter epochs, radius conventions, equation identifiers, sign conventions, Hohmann burn conventions, sphere-of-influence source form, and reference examples.
- **Files/areas:** `crates/aero-codex-astrodynamics/src/lib.rs`; `validation/cards/astrodynamics_*.yaml`; `validation/source_registry/astrodynamics_*.yaml`; `validation/source_registry/*nasa*jpl*.yaml`.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T001, AC-P0.001-T002.
- **Acceptance criteria:** Reviewed notes are sufficient to build reference tests while preserving non-mission/non-flight caveats.

### AC-P0.001-T009 — Bio-regenerative life-support source review

- **Title:** Review BVAD/ECLSS and crop-productivity source assumptions.
- **Objective:** Identify exact source editions, accounting boundaries, crew metabolic demand values, crop productivity assumptions, O2/CO2/water examples, storage conventions, and validation tolerances.
- **Files/areas:** `crates/aero-codex-life-support/src/lib.rs`; `validation/cards/life_support_*.yaml`; `validation/source_registry/life_support_*.yaml`; `docs/phase_0_001/source_research_backlog.md`.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T001.
- **Acceptance criteria:** Life-support helpers remain research-purpose mass-balance primitives and are not represented as validated ECLSS, habitat-safety, medical, or mission models.

### AC-P0.001-T010 — Robust scalar root-solver utilities

- **Title:** Add pure-Rust root/bracket utilities for future inverse equations.
- **Objective:** Implement reviewed, dependency-free numerical utilities needed by Microtasks 022, 033, and 034 without adding BLAS/LAPACK, native code, or foreign runtimes.
- **Files/areas:** `crates/aero-codex-core/src/` or a focused internal module in the consuming crate; tests in the same crate; docs in `docs/phase_0_001/`.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T004 for gas-dynamics tolerances; AC-P0.001-T008 for astrodynamics tolerances.
- **Acceptance criteria:** Tests cover bracket success/failure, monotonic assumptions, nonfinite values, iteration limits, and deterministic error reporting through `AeroError`.

### AC-P0.001-T011 — Gas dynamics area-Mach inverse

- **Title:** Add branch-explicit area-Mach inverse helper.
- **Objective:** Implement a pure-Rust, branch-explicit area-Mach inverse using reviewed source equations and the root-solver utilities.
- **Files/areas:** `crates/aero-codex-gas-dynamics/src/lib.rs`; `validation/cards/gasdyn_isentropic_flow.yaml`; gas-dynamics docs.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T004, AC-P0.001-T010.
- **Acceptance criteria:** Subsonic and supersonic branch behavior is explicit; invalid branch/domain inputs fail deterministically; source/card metadata remains conservative unless reviewed evidence justifies promotion.

### AC-P0.001-T012 — Rayleigh-flow primitives

- **Title:** Add Phase 0.001 Rayleigh-flow scalar helpers.
- **Objective:** Add a minimal, source-reviewed, pure-Rust Rayleigh-flow API surface with conservative trace metadata and validation-card seed.
- **Files/areas:** `crates/aero-codex-gas-dynamics/src/`; `validation/cards/gasdyn_rayleigh_flow.yaml`; `validation/source_registry/*gas*.yaml`.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T004.
- **Acceptance criteria:** Public helpers are bounded to scalar preliminary-design use, include invalid-input tests, and do not imply comprehensive compressible-flow solver coverage.

### AC-P0.001-T013 — Fanno-flow primitives

- **Title:** Add Phase 0.001 Fanno-flow scalar helpers.
- **Objective:** Add a minimal, source-reviewed, pure-Rust Fanno-flow API surface with conservative trace metadata and validation-card seed.
- **Files/areas:** `crates/aero-codex-gas-dynamics/src/`; `validation/cards/gasdyn_fanno_flow.yaml`; `validation/source_registry/*gas*.yaml`.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T004, AC-P0.001-T010.
- **Acceptance criteria:** Equations and domain limits are documented; tests include representative valid and invalid inputs; no external solver dependency is introduced.

### AC-P0.001-T014 — Nozzle flow with choking

- **Title:** Extend propulsion/gas-dynamics nozzle helpers for choking checks.
- **Objective:** Add source-reviewed pure-Rust helpers for nozzle pressure ratio/choking decisions while keeping ideal/nozzle-loss assumptions explicit.
- **Files/areas:** `crates/aero-codex-propulsion/src/lib.rs`; optional shared gas-dynamics helpers; propulsion validation cards/source registry.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T005, AC-P0.001-T011.
- **Acceptance criteria:** Choking logic has domain/error tests, clear sign/station conventions, and no CEA/native/real-gas dependency.

### AC-P0.001-T015 — Standard atmosphere layers and Sutherland viscosity

- **Title:** Expand atmosphere and transport primitives after source review.
- **Objective:** Add reviewed standard-atmosphere layer support and Sutherland viscosity/transport-property helpers with documented temperature/domain limits.
- **Files/areas:** `crates/aero-codex-atmosphere/src/lib.rs`; `crates/aero-codex-thermo/src/lib.rs` or a new pure-Rust transport module; relevant validation cards/source registry.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T003, AC-P0.001-T005.
- **Acceptance criteria:** Layer transitions and viscosity reference values are tested; current troposphere behavior remains backward-compatible or migration-noted.

### AC-P0.001-T016 — Basic aerodynamic model expansion

- **Title:** Add thin-airfoil, finite-wing, and drag-polar helpers.
- **Objective:** Implement Microtasks 028-030 as source-reviewed scalar helpers for thin-airfoil theory, finite-wing lift-curve slope, and drag-polar bookkeeping.
- **Files/areas:** `crates/aero-codex-aerodynamics/src/lib.rs`; aerodynamics validation cards/source registry; docs examples.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T006.
- **Acceptance criteria:** Aspect-ratio, efficiency, compressibility/nonlinear exclusions, and reference-area assumptions are explicit; helpers remain preliminary-design only.

### AC-P0.001-T017 — Rocket nozzle expansion and ramjet scaffold

- **Title:** Add ideal nozzle expansion ratio and ramjet-cycle scaffolds.
- **Objective:** Implement Microtasks 031-032 with pure-Rust scalar equations and conservative validation metadata, without adding chemistry/property wrappers.
- **Files/areas:** `crates/aero-codex-propulsion/src/lib.rs`; propulsion docs/cards/source registry.
- **Verification gate:** Full global Rust gate plus dependency-policy check.
- **Dependencies:** AC-P0.001-T005, AC-P0.001-T014.
- **Acceptance criteria:** Ideal assumptions are explicit; no CEA, Cantera, REFPROP, native binary, or foreign runtime dependency appears.

### AC-P0.001-T018 — Kepler, state-vector, and J2 scaffold

- **Title:** Add early astrodynamics numerical helpers.
- **Objective:** Implement Microtasks 033-035 for Kepler anomaly solving, orbital elements/state-vector conversion, and a bounded J2 perturbation scaffold.
- **Files/areas:** `crates/aero-codex-astrodynamics/src/lib.rs`; astrodynamics validation cards/source registry; docs examples.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T008, AC-P0.001-T010.
- **Acceptance criteria:** Frame/convention assumptions are explicit; solvers have convergence and invalid-domain tests; no mission-planning validity is implied.

### AC-P0.001-T019 — Life-support stoichiometry and crop-productivity source research

- **Title:** Add reviewed O2/CO2 stoichiometry and crop productivity research tasks.
- **Objective:** Implement Microtasks 036-037 after source review, separating scalar stoichiometric bookkeeping from any validated crop-growth/ECLSS model claim.
- **Files/areas:** `crates/aero-codex-life-support/src/lib.rs`; `validation/cards/life_support_*.yaml`; `validation/source_registry/life_support_*.yaml`; docs examples.
- **Verification gate:** Full global Rust gate.
- **Dependencies:** AC-P0.001-T009.
- **Acceptance criteria:** Stoichiometric equations have reviewed citations and tests; crop-productivity entries are documented as research targets unless exact values and validation evidence are reviewed.

### AC-P0.001-T020 — Validation report generator

- **Title:** Add a pure-Rust validation report generator.
- **Objective:** Implement Microtask 038 by generating local reports from validation cards/source registry without weakening conservative statuses or committing generated reports by default.
- **Files/areas:** `xtask/src/`; `validation/`; optional docs under `docs/phase_0_001/`.
- **Verification gate:** Full global Rust gate; generated-output smoke test in `xtask`.
- **Dependencies:** AC-P0.001-T001.
- **Acceptance criteria:** Report generator is deterministic, dependency-free, redacts local paths/secrets, and makes generated reports opt-in/outside-repo unless explicitly approved.

### AC-P0.001-T021 — Public docs landing page and examples

- **Title:** Add public-facing docs/examples for the merged baseline.
- **Objective:** Implement Microtask 040 by improving examples and docs navigation while preserving research-purpose-only and non-certification language.
- **Files/areas:** `README.md`; `docs/index.md`; crate examples; `docs/phase_0_001/`.
- **Verification gate:** `git diff --check`; full global Rust gate when examples compile/run.
- **Dependencies:** AC-P0.001-T020 optional for generated validation summaries.
- **Acceptance criteria:** Examples are copy-pasteable, compile under CI, and avoid claims of flight, mission, operational, or regulated-use suitability.

### AC-P0.001-T022 — Thin-film BLSS solver and scenario layer

- **Title:** Plan optional thin-film BLSS solver/scenario layer without core dependency drift.
- **Objective:** Convert `docs/phase_0_001/thinfilm_implementation_plan.md` Phase 2 into implementable tasks for optional ODE/DAE, nonlinear algebraic solving, tank-in-series wrappers, ROM calibration interfaces, and scenario cards.
- **Files/areas:** `crates/aero-codex-life-support/src/`; `data/thinfilm/`; `docs/phase_0_001/thinfilm_implementation_plan.md`; validation cards/source registry.
- **Verification gate:** Full global Rust gate plus `cargo run -p xtask -- verify data-registry` if artifact-tracked files change.
- **Dependencies:** AC-P0.001-T009, AC-P0.001-T010.
- **Acceptance criteria:** Solver/scenario layer remains optional and pure Rust; no life-support readiness, habitat-safety, or mission suitability is implied.

### AC-P0.001-T023 — Thin-film BLSS calibration and validation plan

- **Title:** Add calibration/validation tasks for thin-film BLSS research kernels.
- **Objective:** Convert thin-film implementation plan Phase 3 into tasks for reference examples, parameter calibration, and validation-status promotion rules.
- **Files/areas:** `data/thinfilm/`; `validation/cards/life_support_thinfilm_*.yaml`; `validation/source_registry/life_support_*.yaml`; `docs/phase_0_001/thinfilm_citation_verification.md`.
- **Verification gate:** Full global Rust gate plus artifact verifier when tracked files change.
- **Dependencies:** AC-P0.001-T001, AC-P0.001-T009, AC-P0.001-T022.
- **Acceptance criteria:** Validation promotion requires numerical reproduction with tolerances; calibration is framed as research, not operational/habitat-safety validation.

### AC-P0.001-T024 — Thin-film BLSS orchestration boundary

- **Title:** Define future BLSS orchestration boundaries and state schemas.
- **Objective:** Convert thin-film implementation plan Phase 4 into a bounded design for state schemas and subsystem interfaces without adding agentic optimization or control claims.
- **Files/areas:** `docs/phase_0_001/thinfilm_implementation_plan.md`; future life-support trait/docs; validation cards/source registry if public interfaces are added.
- **Verification gate:** Docs-only: `git diff --check`, `cargo run -p xtask -- verify --all`, `cargo run -p xtask -- dependency-policy`; full global Rust gate if code is added.
- **Dependencies:** AC-P0.001-T022.
- **Acceptance criteria:** State schemas are named and traceable; any control/MPC/ROM/learning layer remains future-bound and clearly outside validated life-support readiness.

## Triage order

1. AC-P0.001-T001 source-registry review checklist.
2. AC-P0.001-T002 through AC-P0.001-T009 source-review passes.
3. AC-P0.001-T010 root-solver utilities.
4. AC-P0.001-T011 through AC-P0.001-T019 equation/API expansion tasks.
5. AC-P0.001-T020 and AC-P0.001-T021 validation/reporting/docs polish.
6. AC-P0.001-T022 through AC-P0.001-T024 thin-film BLSS solver, calibration, and orchestration follow-ups.
