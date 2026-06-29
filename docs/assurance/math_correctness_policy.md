# Math Correctness Policy

AeroCodex mathematical capabilities are research and preliminary-design artifacts until explicitly promoted by evidence. This policy applies to equations, algorithms, data-derived helpers, generated kernels, source-ingested functions, validation cards, and user-facing examples.

AeroCodex is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Required equation contract

Every math or code capability must have an equation contract before it can be treated as a stable AeroCodex surface. The contract must define:

- capability name and owning crate/module;
- source IDs and citation/provenance records;
- equation form or algorithm statement;
- input variables, output variables, and units;
- coordinate frames, sign conventions, reference epochs, and branch conventions where applicable;
- valid domains and invalid domains;
- singularities, discontinuities, convergence hazards, and near-boundary behavior;
- assumptions and simplifications;
- dimensional-analysis expectations;
- tolerance policy and numerical precision expectations;
- test set and reference-oracle set;
- validation status;
- docs and examples that preserve caveats.

## Minimum validation ladder

| Status | Meaning | Required evidence |
| --- | --- | --- |
| `source_material` | External or local source is identified but not implemented. | Source ID, license status, source inventory row. |
| `quarantined_candidate` | Candidate code or formulas exist outside the public API. | Intake record, license classification, source-boundary decision. |
| `implemented_research` | Rust implementation exists but remains research-only. | Equation contract, unit tests, source IDs, docs caveat. |
| `reference_checked` | Implementation has independent numerical checks. | Reference values, tolerances, test results, edge-case notes. |
| `source_equivalent` | Implementation has source-equivalence evidence. | Source-equivalence jobs, pass logs, tolerance rationale. |
| `release_candidate` | Candidate is ready for review but not certified. | Full local checks, docs, known-risk register, unresolved blockers. |
| `certified_for_scope` | A scoped assurance package exists. | Separate approval package. This status is not created by Stage 4 Chunk 0. |

## Tolerances

- Tolerances must be explicit, reviewed, and tied to the equation domain.
- Avoid one global tolerance for all functions.
- Record absolute and relative tolerance choices where both matter.
- Include singularity, near-zero, branch-switch, and convergence tests when those conditions exist.
- Distinguish formatting roundoff from numerical-method error.

## M07 formula-vault policy

The M07 Scilab-to-Rust workspace reports 1,350 represented function rows and 188 Scilab equivalence jobs. Its packaging includes static provenance and static invariant summaries, but it remains release-candidate material and is not certified.

M07 may be used as a quarantined formula-vault candidate only after intake records identify source IDs, license status, equation contracts, tolerance policy, and validation status. M07 must not replace `crates/aero-codex-astrodynamics` and must not become public API until Rust checks, Scilab equivalence, and SGP4 certification pass.

## BioSim-RS policy

BioSim-RS is first-class Stage 4 source material for deterministic life-support and habitat digital-twin modeling. It remains license-boundaried because the original Java BioSim source is GPL-3.0-or-later. Any future implementation must choose and document one path before code promotion: GPL-compatible integration, permissioned relicensing, or clean-room implementation with separate specification and implementation roles.

## Orekit policy

Orekit may be used as a reference oracle and architecture guide. Its equations, validation families, and architectural lessons can inform AeroCodex contracts and tests. Do not copy the Java class hierarchy class-for-class, and do not promote Orekit-derived behavior without AeroCodex-native source IDs, tests, tolerance records, and documentation.
