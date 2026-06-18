# Stage 5 BioSim/Orekit v3 Serial Plan

This document records the current BioSim and Orekit v3 serial plan after deployed O2a, deployed O2b, deployed O2c, adapted Session E, and adapted Session F slices. It is documentation/status control only. It does not authorize O2d runtime/code, approve source-locator normalization, import external source, or change governed inventories outside a bounded deployment chunk.

## Summary

- Orekit v3 O2a time/frame/Cartesian-state foundation is deployed/completed at `2f1e64ea7638b2f54071eca488c26252256235ca`.
- Orekit v3 O2b classical-elements/Kepler numerical-policy review and bounded deployment is deployed/completed as research/preliminary-only code and remains historically classified `needs_review`.
- Orekit O2c oracle-record/tolerance-helper review is deployed/completed as research/preliminary-only local comparison infrastructure; Orekit O2d is the next recommended candidate and remains `needs_review` pending separate authorization.
- Adapted Session F reference-oracle planning metadata is deployed at `68dc10fc9215df2be9bc64e0f2a94121250c361a`, but it is not a replacement for O2c or O2d; O2c is separately deployed and O2d remains unfinished.
- Adapted Session E BioSim-plus docs/contracts are deployed at `9dcc303336d12e401c4a866b3bc2410c937014dd`, but they are not a replacement for BioSim B2a, B2b, or B2c.
- Corrected BioSim B2a remains `needs_review`; BioSim B2b and BioSim B2c remain `blocked` by predecessors.
- Legacy Session E/F, old v1/v2, B1a, O1a, and aggregate wrappers are audit/history references, not direct deployment targets.

## Orekit v3 serial order

| Order | Slice | Current status | Classification | Evidence / dependency | Remaining scope |
|---:|---|---|---|---|---|
| 1 | O2a time/frame/Cartesian-state foundation | deployed/completed | needs_review | `2f1e64ea7638b2f54071eca488c26252256235ca` | None for O2a; current counts already include it. |
| 2 | O2b classical elements, Kepler helpers, deterministic smoke example | deployed/completed | needs_review | O2a completed. | No remaining O2b deployment scope; research/preliminary-only and not operational Orekit parity. |
| 3 | O2c oracle records and tolerance comparison helpers | deployed/completed | needs_review | Depends on deployed O2b. | Local deterministic record construction and tolerance comparison only; no external oracle execution, evidence verification, external fixture, TLE, SGP4, TEME, propagation, or parity claim. |
| 4 | O2d two-line element contract/source policy | next bounded candidate | needs_review | Depends on deployed O2c. | Contract/source policy only; no parser, SGP4, TEME transform, propagator, or frame implementation. |

## BioSim v3 serial order

| Order | Slice | Current status | Classification | Evidence / dependency | Remaining scope |
|---:|---|---|---|---|---|
| 1 | Corrected B2a domain and structural validation | outstanding future candidate | needs_review | Separate BioSim prompt required. | Correct known test-literal issue, run source-boundary review, rustfmt/compile/tests, inventory/regeneration, full gates, and CI. |
| 2 | B2b-1 process/planner | blocked_until_predecessor | blocked | Depends on accepted corrected B2a. | Re-cut B2b into smaller review units before any deployment. |
| 3 | B2b-2 bounded replay/event model | blocked_until_predecessor | blocked | Depends on B2b-1. | Resolve hard limits, checked allocation, underflow, and event semantics. |
| 4 | B2b-3 optional flat-resource adapter proof | blocked_until_predecessor | blocked | Depends on B2b-2. | Optional and non-authoritative exact-mapping proof only. |
| 5 | B2c ledger/report/example/governance | blocked_until_predecessor | blocked | Depends on final accepted B2b contract. | Requires example execution evidence and report-integrity review. |

## Relationship to deployed adapted slices

| Material | Current relationship | Classification |
|---|---|---|
| Adapted Session E BioSim-plus clean-room docs/contracts | Deployed as docs/contracts only; deep BioSim v3 runtime lanes remain open. | superseded |
| Adapted Session F Orekit reference-oracle planning metadata | Deployed as planning metadata only; O2b and O2c were deployed separately and Orekit O2d remains open. | superseded |
| Session E original BioSim-plus handoff | Older direct lane replaced by BioSim v3 serial intake or adapted bounded docs/contracts where explicitly deployed. | superseded |
| Session F original Orekit handoff | Older direct lane replaced by Orekit v3 serial intake or adapted bounded reference-oracle metadata where explicitly deployed. | superseded |
| Older BioSim v1/v2 materials, B1a, and companion notes | Audit inputs only; corrected B2a is the future first BioSim lane. | superseded |
| Older Orekit v1/v2 materials, O1a, and companion notes | Audit inputs only; deployed O2a/O2b/O2c and remaining O2d define the Orekit lane. | superseded |
| Aggregate wrappers such as `stage 5.zip`, `files-aerocodex.zip`, `BioSim and Orekit v.zip`, and `Orekit and Bio new v.zip` | Not deployment patches; do not deploy aggregate/source containers. | blocked |

## Source-boundary rules

- No Orekit Java source, class hierarchy, API cloning, class files, or translated implementation structure may be copied into AeroCodex.
- No GPL BioSim or BioSim-RS code may be copied into the dual MIT/Apache AeroCodex core.
- No M07 or Scilab source, generated source outputs, or raw source archives may be imported.
- No external fixtures may be added without provenance, hash, source review, and explicit prompt authorization.
- All advisory deltas remain planning numbers only; never hardcode counts or absolute totals.

## Known blockers

- Orekit O2d TLE contract/source-policy review and deployment after O2c closure.
- BioSim B2a corrections and source-boundary review.
- BioSim B2b re-cut requirement.
- BioSim B2c example execution and report-integrity evidence.
