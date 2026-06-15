# Stage 4 Master Plan

Stage 4 is a planning and governance freeze for the next AeroCodex expansion wave. It does not import feature code, add crates, certify existing research kernels, or promote external workspaces into public APIs.

## Safety and certification caveat

AeroCodex is research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved. Stage 4 does not change that status.

## Repository governance

- Current GitHub `main` is the source of truth.
- Keep one canonical `main` version. Do not maintain competing long-running branches or a branch maze.
- Use a short-lived branch only when a pull-request or review workflow needs one; merge it back to `main` after checks pass and delete it.
- The deployment agent owns the mechanical update loop for each chunk: sync, inspect, diff, edit, test, commit, merge or fast-forward, push, and verify the final remote state.
- Cargo package versions remain conservative semantic versions. Stage and milestone labels are human planning labels, not package-version numbers.

## Stage 4 objectives

1. Preserve the existing pure-Rust, source-traceable AeroCodex core while opening governed intake lanes for larger source families.
2. Treat external bundles as source material, planning material, or quarantined candidates until their licensing, traceability, and validation gates are satisfied.
3. Require every math or code capability to carry an equation contract, units, domains, singularities or invalid regions, source IDs, tests, tolerances, validation status, and user-facing docs before promotion.
4. Make BioSim-RS a first-class Stage 4 workstream while preserving its license boundary from the dual MIT/Apache AeroCodex core.
5. Treat the M07 Scilab-to-Rust astrodynamics release candidate as a formula-vault candidate until Rust checks, Scilab equivalence, and SGP4 certification pass.
6. Use Orekit as a reference oracle and architecture guide, not as a Java class hierarchy to clone.

## Intake lanes

### Lane A: Core AeroCodex governance and docs

This lane contains Stage 4 planning, math-assurance policy, merge/release policy, and source-intake inventory. It is documentation-only in Chunk 0.

### Lane B: M07 astrodynamics formula-vault candidate

The M07 workspace reports 1,350 represented function rows and 188 Scilab equivalence jobs. It remains a release candidate and is not certified. It must not overwrite `crates/aero-codex-astrodynamics`, must not be bulk-merged into public APIs, and must not be treated as production math until the release gates in `docs/assurance/math_correctness_policy.md` and `docs/assurance/merge_and_release_policy.md` pass.

### Lane C: BioSim-RS clean rewrite workstream

BioSim-RS is a first-class Stage 4 candidate for life-support and habitat digital-twin modeling. It is also license-boundaried because the original Java BioSim source is GPL-3.0-or-later. No Java implementation code, translated code, or derivative GPL-bound implementation detail may be mixed into the current dual MIT/Apache AeroCodex core unless the project deliberately changes the licensing path or obtains appropriate permission.

Chunk 4 records that boundary in `docs/assurance/biosim_rs_license_architecture.md`, `docs/source_intake/biosim_rs_source_boundary.md`, and the README-only `biosim-rs/` placeholder. Those files select no implementation path by themselves; they define the GPL-compatible, permissioned, and clean-room gates that later BioSim-RS slices must satisfy before code promotion.

Chunk 6A starts the clean-room path with generic resource-identity and tick-validation primitives in `crates/aero-codex-life-support`. It adds validation metadata and `research_required` traceability only; it does not import GPL source, add BioSim scaffold crates, commit transactions, prove replay, create a resource ledger, or validate conservation behavior.

### Lane D: Orekit reference-oracle lane

Orekit is an Apache-2.0 reference source for space-flight-dynamics architecture and validation thinking. It may guide terminology, test-oracle selection, and architecture review. It must not be copied class-for-class or used to preserve Java inheritance structures in Rust.

Chunk 5 records that boundary in `docs/assurance/orekit_reference_oracle_mapping.md` and `docs/source_intake/orekit_reference_oracle_boundary.md`. Those files define allowed oracle-planning use, future evidence fields, reference-oracle test families, and non-copying architecture rules; they do not run Orekit, generate fixtures, port Java classes, or promote new astrodynamics APIs.

## Chunk sequence

| Chunk | Name | Scope | Public code impact |
| --- | --- | --- | --- |
| 0 | Planning and governance freeze | Add Stage 4 planning, assurance, merge, and source-intake docs; update indexes. | None. Docs only. |
| 1 | Data/source governance skeleton | Define source IDs, intake manifests, quarantine states, evidence-card schema, and license-boundary labels. Chunk 1 adds the data/source governance policy, `data-governance/DATA_REGISTRY.yaml`, and the dependency-free `cargo run -p xtask -- verify data-registry` gate. | No feature code unless explicitly approved. |
| 2 | Validation/status vocabulary normalization | Add the canonical status vocabulary registry, human guidance, and `cargo run -p xtask -- verify status-vocabulary` gate across validation cards, source-registry seeds, and data-governance fields. | None. Governance and verifier only. |
| 3 | Formula-vault staging design | Plan quarantined M07 ingestion shape, equation contracts, tolerance policy, and certification queue. Chunk 3 adds `docs/assurance/formula_vault_staging.md`, `docs/source_intake/m07_formula_vault_intake.md`, and the empty `formula-vault/` skeleton. | No public API promotion; no M07 source-code import. |
| 4 | BioSim-RS license-bound architecture | Define clean-room, GPL-compatible, and permissioned paths; record workspace boundary and validation plan in `docs/assurance/biosim_rs_license_architecture.md`, `docs/source_intake/biosim_rs_source_boundary.md`, and the README-only `biosim-rs/` placeholder. | No dual-core mixing without a licensing decision; no implementation import. |
| 5 | Orekit reference-oracle mapping | Define reference-oracle use, test families, future oracle evidence records, and non-copying architecture notes in `docs/assurance/orekit_reference_oracle_mapping.md` and `docs/source_intake/orekit_reference_oracle_boundary.md`. | No class hierarchy cloning; no Orekit execution, fixtures, Java source import, or new APIs. |
| 6A | BioSim-RS resource identity and tick validation | Add clean-room generic resource identities, positive-duration tick validation, consecutive tick-transition validation, and validation/source-registry metadata. | Public life-support primitives only; no GPL source import, scaffold crate import, transaction commit, replay proof, ledger, conservation model, or readiness claim. |
| 6B | BioSim-RS atomic transaction commit | Future slice for resource-delta transaction staging after Chunk 6A closes. | Not started; must add separate tests, source IDs, validation card updates, and no GPL mixing. |
| 6C | BioSim-RS deterministic ordering, digest, and replay proof | Future slice for deterministic module order and replay evidence. | Not started; no replay claim before evidence. |
| 6D | BioSim-RS resource ledger and minimal O2-loop conservation | Future slice for ledger accounting and minimal conservation checks. | Not started; no habitat-safety or biological validation claim. |
| 6E | BioSim-RS CLI/API smoke tests and friend-test report | Future slice for user-facing smoke tests after the kernel slices exist. | Not started. |
| 7+ | Formula-vault and implementation candidates | Continue only after formula-vault gates and per-slice source/validation records exist. | Bounded by per-slice gates. |

## Promotion gates for any future capability

A future capability may move from source material to public AeroCodex API only after it has:

- a durable source ID and license classification;
- an equation contract with input/output units, domains, singularities, branch behavior, and invalid-region behavior;
- reviewed Rust implementation and tests;
- numerical tolerances with rationale;
- validation status that distinguishes static provenance, unit tests, reference-oracle tests, source-equivalence tests, and certification claims;
- documentation that repeats the research/preliminary-design caveat and avoids flight, mission, habitat-safety, medical, operational, or regulated-use claims.
