# Stage 5 BioSim/Orekit v3 Serial Plan

This document records the final deep BioSim and Orekit v3 handoff arrival and the future serial review plan. It is documentation-only. It does not deploy runtime/code, apply any patch, approve any source-locator normalization, import external source, or change governed inventories.

## Summary

The v3 bundles are now the canonical deep-session intake materials for BioSim and Orekit lanes. They are not live deployable merely because they are packaged. Future chunks must process one bounded subpatch at a time from exact current `main`, with live preflight, semantic review, governed count-delta calculation, local gates, push, and exact deployed CI.

- Orekit v3 overall classification: `needs_review`.
- BioSim v3 overall classification: `needs_review`.
- Runtime/code subpatches remain `needs_review` or `blocked` until their predecessor and gate requirements close.
- Legacy Session E/F, old v1/v2, B1a, O1a, and aggregate wrappers are audit/history references, not direct deployment targets.

## Hash table

Exact relative paths are recorded in acronym-guard-safe headings before the summary table.

### ACX-NOM-EXACT BioSim v3 bundle path: `BIO v/AeroCodex_stage5_session_BIO_v3_deep_clean_room_scenario_engine_handoff.zip`
### ACX-NOM-EXACT BioSim v3 B2a member: `BIO_v3_patch_series/B2a_domain_validation.patch`
### ACX-NOM-EXACT BioSim v3 B2b member: `BIO_v3_patch_series/B2b_process_replay_adapter.patch`
### ACX-NOM-EXACT BioSim v3 B2c member: `BIO_v3_patch_series/B2c_ledger_report_example.patch`
### ACX-NOM-EXACT Orekit v3 bundle path: `ORE v/AeroCodex_stage5_session_ORE_astrodynamics_foundation_v3_handoff.zip`
### ACX-NOM-EXACT Orekit v3 O2a member: `subpatches/ORE_v3_O2a_time_frame_state.patch`
### ACX-NOM-EXACT Orekit v3 O2b member: `subpatches/ORE_v3_O2b_elements_kepler.patch`
### ACX-NOM-EXACT Orekit v3 O2c member: `subpatches/ORE_v3_O2c_oracle_records.patch`
### ACX-NOM-EXACT Orekit v3 O2d member: `subpatches/ORE_v3_O2d_tle_contract_source_policy.patch`
### ACX-NOM-EXACT legacy BioSim B1a patch: `BIO v/BIO_B1a_patch.patch`
### ACX-NOM-EXACT legacy Orekit O1a patch: `ORE v/ORE_O1a_patch.patch`
### ACX-NOM-EXACT legacy aggregate wrapper: `BIO and ORE v.zip`
### ACX-NOM-EXACT newer aggregate wrapper: `ORE and Bio new v.zip`
### ACX-NOM-EXACT BioSim v2 audit: `BIO_v2_gap_audit.md`
### ACX-NOM-EXACT Orekit v2 audit: `ORE_v2_gap_audit.md`

| Lane | Artifact | SHA256 | Bytes | Classification |
|---|---|---|---:|---|
| BioSim v3 bundle | exact path recorded above | `ccc1f4e576f1242256dea429e7316a6897d672227e527517d0675cd6107503a8` | 173650 | needs_review |
| BioSim v3 B2a | exact member recorded above | `6f94069229de8ea76c6366528c0eeb608e3c59940b483d99b53d438f8fae1dea` | 37413 | needs_review |
| BioSim v3 B2b | exact member recorded above | `10561209f926016ae1cc8d2bf8ab559945d3ed2002c2f64285051fe5b276de3f` | 84569 | blocked |
| BioSim v3 B2c | exact member recorded above | `902770052146b78c1aef517217c1af31019984274e52486fb3e5e73b45fff0f7` | 78964 | blocked |
| Orekit v3 bundle | exact path recorded above | `f1c2f4b224b6b9701b99753b9ad33aca590a38234596bc19d749e8b25cd34b21` | 340382 | needs_review |
| Orekit v3 O2a | exact member recorded above | `b0d1d783124cfd39b2dba03e268f86dca08f0e57a3cea16ffa78bb5c0379dc0f` | 55321 | needs_review |
| Orekit v3 O2b | exact member recorded above | `40eac48edd55f5ca0f0d70bce0b73bd5480ce99cafc00c0efe15d525d9c44273` | 100645 | blocked |
| Orekit v3 O2c | exact member recorded above | `658b0380a3c2573ada548a827668349c535c231fadad8e771f424cf9d24d4aaa` | 55530 | blocked |
| Orekit v3 O2d | exact member recorded above | `da937d5870cc092dd75683db2a04b8ba226c00ded9c13f26c7ef85a8d2ca0621` | 56638 | blocked |

## Orekit v3 serial order

1. O2a — time/frame/Cartesian-state foundation.
   - Advisory deltas from handoff: executable +0, helper +47, cards +0, source seeds +0, card-only +0.
   - Known blockers: live-main applicability, Rust/rustfmt/clippy/tests/docs/xtask/CI unverified, nomenclature/acronym guard, live inventory and data-registry hashes.
   - Classification: `needs_review`.
2. O2b — classical elements, Kepler helpers, deterministic smoke example.
   - Advisory deltas from handoff: executable +13, helper +6.
   - Known blockers: depends on O2a; numerical-policy review; live equation-count delta must be computed from final formatted source.
   - Classification: `blocked`.
3. O2c — oracle records and tolerance comparison helpers.
   - Advisory deltas from handoff: executable +0, helper +15.
   - Known blockers: depends on O2b; metadata-only oracle evidence; no external fixtures copied.
   - Classification: `blocked`.
4. O2d — two-line element contract/source policy plus one validation/source pair.
   - Advisory deltas from handoff: executable +0, helper +3, validation cards +1, source registry seeds +1, validation-card-only records +1.
   - Known blockers: depends on O2c; two-line element contract only, no parser or propagator/frame implementation; source locators remain research requests.
   - Classification: `blocked`.

## BioSim v3 serial order

1. Corrected B2a — domain and structural validation.
   - Advisory deltas from handoff: executable +0, helper +13.
   - Known blockers: correct three doubled-escape validation-test literals, rustfmt, compile, tests, final inventory-line regeneration, registry recomputation, full gates, CI.
   - Classification: `needs_review`.
2. B2b-1 — process types, validated constructors, shape validation, deterministic intent planner.
   - Advisory delta is part of B2b total, not final until re-cut.
   - Known blockers: depends on B2a.
   - Classification: `blocked`.
3. B2b-2 — bounded replay, compact digest, atomic commit/event model, no flat adapter.
   - Advisory delta is part of B2b total, not final until re-cut.
   - Known blockers: depends on B2b-1; must fix hard limits, checked allocation, underflow, requested/committed/clamp event semantics.
   - Classification: `blocked`.
4. B2b-3 — optional flat-resource adapter proof with exact mappings only.
   - Advisory delta is part of B2b total, not final until re-cut.
   - Known blockers: depends on B2b-2; optional and non-authoritative.
   - Classification: `blocked`.
5. B2c — ledger/report/example/governance.
   - Advisory deltas from handoff: executable +0, helper +3, validation cards +1, source registry seeds +1, validation-card-only records +1.
   - Known blockers: depends on final B2b contract; requires replay/event/final-state integrity validation, ledger self-consistency, report-input safety, negative tests, actual example execution evidence, and live registry recomputation.
   - Classification: `blocked`.

Treat all advisory deltas as planning numbers only. Never hardcode counts or absolute totals.

## Relationship to older materials

| Older material | Relationship to v3 | Classification |
|---|---|---|
| Session E original BioSim-plus clean-room handoff | Older direct lane replaced by BioSim v3 serial intake. Retain as audit/history only. | superseded |
| Session F original Orekit oracle handoff | Older direct lane replaced by Orekit v3 serial intake. Retain as audit/history only. | superseded |
| Older BioSim v1/v2 materials and the exact BioSim v2 audit recorded above | Useful audit inputs, but BioSim v3 final self-review and patch series define future blockers. | superseded |
| Older Orekit v1/v2 materials and the exact Orekit v2 audit recorded above | Useful audit inputs, but Orekit v3 risk and nomenclature plans define future blockers. | superseded |
| legacy BioSim B1a patch and companion notes | B1a is an older domain/validation draft; corrected B2a is the future first BioSim v3 lane. | superseded |
| legacy Orekit O1a patch and companion notes | O1a is an older time/frame/state draft; O2a is the future first Orekit v3 lane. | superseded |
| Aggregate wrappers such as `stage 5.zip`, `files-aerocodex.zip`, and the exact aggregate names recorded above | Not deployment patches; do not deploy aggregate/source containers. Historical wrappers already superseded by v3 remain inventory-only. | blocked |

## Source-boundary rules

- No Orekit Java source, class hierarchy, API cloning, class files, or translated implementation structure may be copied into AeroCodex.
- No GPL BioSim or BioSim-RS code may be copied into the dual MIT/Apache AeroCodex core.
- No M07, Scilab source, generated source outputs, or raw source archives may be imported.
- No external fixtures may be added without provenance, hash, source review, and explicit prompt authorization.

## Status rules

- BioSim/Orekit v3 material remains `research_required` only.
- No flight, mission, habitat, medical, operational, certification, regulated-use, parity, or readiness claims are authorized.
- Friend-test language must stay caveated as research/preliminary-design evidence only.

## Gate rules

Every future BioSim/Orekit v3 deployment slice must include:

1. exact current-main preflight, including remote/main sync and exact-main CI;
2. focused patch applicability and semantic/source-boundary review;
3. focused tests for the touched crate/surface;
4. all governed verifiers;
5. full Rust, doc, and nomenclature gates;
6. data-registry hash refresh only when a future deployment actually changes registered aggregates;
7. forbidden-material scan;
8. exact deployed CI before closeout.

## Known blockers

- Orekit nomenclature/acronym reconciliation before any O2 deployment.
- Orekit numerical-policy review for O2b.
- BioSim B2a doubled-escape test corrections.
- BioSim B2b re-cut requirement.
- BioSim B2c example execution and report-integrity evidence.
