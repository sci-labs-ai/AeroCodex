# M07 formula-vault intake

This document records the Stage 4 intake shape for the M07 Scilab-to-Rust astrodynamics release-candidate bundle. It is a quarantined source-intake plan only. It does not import M07 source code, does not add public Rust application programming interfaces, and does not overwrite `crates/aero-codex-astrodynamics`.

AeroCodex remains research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Registered source artifact

| Field | Value |
| --- | --- |
| Data-governance artifact ID | `stage4.m07_rust_port_v14.2026_06_15` |
| Logical source path | `external://stage4/aerocodex_rust_port_v14_m07_final_bundle.zip` |
| Registered SHA256 | `15b1ca3a39267187167c43ea1228f28fd4736c4456f65d072dc42a32a7b19190` |
| Reported represented function rows | 1,350 |
| Reported Scilab equivalence jobs | 188 |
| Current status | Release-candidate / not certified |
| Allowed use | Quarantined formula-vault candidate and reference material only |

The registered counts and hash are source-material facts. They are not evidence of operational approval, flight readiness, or public API readiness.

## Intake boundary

M07 intake must stay inside these boundaries until a later verified chunk narrows the scope:

- no bulk import of the external bundle;
- no copying M07 implementation files into public crates;
- no replacement of the existing astrodynamics crate;
- no public API promotion from source-material availability alone;
- no validation-status upgrade without matching evidence;
- no use of the word certified except in caveats or in a separate scoped assurance package.

## Intake workflow

1. Confirm the source artifact is registered in `data-governance/DATA_REGISTRY.yaml`.
2. Build an inventory from the M07 represented function rows without importing implementation code.
3. Assign a stable `formula_id` for each candidate formula or formula family.
4. Draft an equation contract using `docs/assurance/formula_vault_staging.md`.
5. Record source equation, table, page, function-row, or file-local references for human review.
6. Record variables, units, coordinate/time assumptions, valid domain, and exclusions.
7. Record numerical tolerance policy and reference-oracle/equivalence-test plan.
8. Add per-slice source-registry seed and validation card records before implementation work is proposed.
9. Queue only bounded implementation candidates after contract review.
10. Run Rust, Scilab equivalence, and SGP4 gates that apply to the candidate scope.
11. Promote only after the promotion gate is satisfied and a chunk explicitly approves that bounded promotion.

## Formula-vault quarantine states

M07 material should move through these states in small, reviewable chunks:

| State | Meaning | Promotion implication |
| --- | --- | --- |
| `registered_source_material` | The archive is tracked by data governance. | No implementation promotion. |
| `parsed_inventory_only` | Function/formula inventory exists without source-code import. | No implementation promotion. |
| `equation_contract_drafted` | Required contract fields are drafted for review. | No implementation promotion by itself. |
| `implementation_candidate` | A bounded Rust candidate is proposed behind gates. | Candidate remains quarantined until checks pass. |
| `equivalence_tested` | Equivalence/reference tests have run with recorded tolerances and results. | May support review, not certification. |
| `reference_validated` | The selected reference-oracle plan passed for the declared scope. | May support bounded promotion if all other gates pass. |
| `rejected_or_superseded` | Candidate is blocked, replaced, or retained only for traceability. | No promotion. |

These are quarantine lifecycle labels. They do not replace the canonical validation/status vocabulary and should not be written as `data-governance` validation statuses unless a later verifier explicitly supports them.

## First metadata slice

Stage 4 Chunk 7B adds the first formula-vault metadata slice by copying the candidate-gate shape into `formula-vault/candidates/m00_angle_unit_conversions.yaml` for a small, low-risk subset only. The slice covers three M00 angle/unit helper rows with clear M07 release-gate row locators and Scilab equivalence job locator `equivalence job 002`. It does not attempt to ingest all 1,350 represented rows at once.

The first metadata slice records:

- a bounded formula-vault candidate metadata file approved by the repo;
- unique `formula_id` values;
- source artifact references back to `stage4.m07_rust_port_v14.2026_06_15`;
- source status-row/function locators;
- variables and units;
- coordinate/time assumptions;
- valid domains and invalid non-finite inputs;
- deferred tolerance policy;
- planned reference-oracle or Scilab equivalence checks;
- a promotion-gate field that remains blocked until evidence exists.

## Deferred items

The following are intentionally deferred beyond Chunk 3:

- selecting the machine-readable contract format;
- parsing the M07 archive;
- importing formula implementations;
- creating public AeroCodex APIs;
- changing the astrodynamics crate;
- executing the 188 Scilab equivalence jobs;
- no SGP4 certification claim and no broader operational-readiness claim.
