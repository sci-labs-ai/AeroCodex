# Public alpha readiness dashboard

This dashboard is a tester-facing summary for the public friend-test package. It is not a release approval, certification artifact, safety case, mission-readiness case, habitat-safety case, medical-use case, or regulated-use approval.

AeroCodex is research/preliminary-design software. Passing local checks does not prove physical validity, safety, certification, or mission readiness.

## Dashboard scope

The public alpha friend-test package is intended to answer four narrow questions:

1. Can a tester clone the repository and run the configured local Rust and governance checks?
2. Does the equation inventory explain what is executable research code, metadata-only candidate material, validation-card-only metadata, and helper code?
3. Do testers see the project's research-only and blocked status before using any formula?
4. Can failures be reported in a reproducible way without importing external source material or generated artifacts?

## Current main governed counts

Authoritative readiness count language lives in `docs/roadmap/research_readiness_counts.md`. That file is the source of truth for current values, evidence, meanings, and non-claims; this dashboard only mirrors the RR-004 public-alpha summary.

| Count summary | Current value | Dashboard meaning |
|---|---:|---|
| Governed equation-batch rows | 152 | Executable Rust/runtime equation rows from governed equation-batch manifests, still conservative `research_required` research/preliminary-design software. |
| CLI-accessible legacy M00 canonical formulas | 10 | The current Beta 1 concept CLI exposes ten legacy M00 canonical formulas for bounded software testing. |
| M00 formula-vault candidates | 27 | Formula-vault metadata/provenance candidate records; not 27 newly implemented or newly exposed formulas. |
| Visible M07 terminal candidate rows | 1,323 | Visible M07 formula-vault resolution rows with terminal dispositions. The 1,323 M07 rows are not 1,323 usable equations. |
| M07 execution backlog rows | 0 M07 execution backlog rows | No governed M07 source rows remain without a terminal disposition row; this does not unblock M07 execution. |

Do not use historical A11-A45 dashboard snapshots, old Stage 5 queue snapshots, or raw M07 resolution row totals as execution-readiness claims. M07 material remains quarantined source/candidate material unless a later task explicitly promotes a family and changes execution policy.

## Public alpha lanes

| Lane | User-visible artifact | Current label | Blocked from promotion because |
|---|---|---|---|
| Local clone checks | `docs/testing/friend_test_quickstart.md`, `scripts/friend_test_local.sh`, `scripts/friend_test_local.ps1` | research_required | Local checks are software gates only and do not prove physical validity or safety. |
| Expected-output guide | `docs/testing/friend_test_expected_output.md` | research_required | Output varies by toolchain and is not validation evidence. |
| Safety caveats | `docs/testing/research_safety_caveats_for_testers.md` | research_required | Caveats must stay visible for public testers. |
| Equation inventory | `validation/equation_inventory.tsv` and `cargo run -p xtask -- verify equation-inventory` | research_required | Inventory rows classify items; they do not validate formulas. |
| Formula-vault and external-resolution metadata | `formula-vault/` and `cargo run -p xtask -- verify formula-vault` | research_required | Candidate and resolution records reuse, block, or account for existing concepts; they do not imply certification, source parity, or operational readiness. |
| Dependency policy | `cargo run -p xtask -- dependency-policy` | research_required | Dependency hygiene is not physical validation or safety approval. |
| Documentation build | `cargo doc --workspace --all-features --no-deps` | research_required | Documentation generation does not create validation evidence. |
| Beta 1 concept CLI | `aerocodex` commands and `cargo run -p aero-codex-cli -- self-check --json` | research_required | The vertical slice exposes ten governed M00 canonical formulas for software testing only; it is not the full 1,000+ equation program or an operational release. |

## Known blocked public-facing items

- `m00_wrap2pi` has a bounded executable/public Rust runtime for `formula_vault.m00.angle.wrap2pi`, remains research_required/non-certified, and makes no M07/Scilab parity claim; alternate aliases remain blocked.
- `app_resolve_coplanar` remains blocked for least-squares, rank, singularity, and tolerance policy.
- Orekit v3 O2a/O2b/O2c/O2d exist as bounded research/preliminary-only foundations and metadata helpers; O2d is contract/source-policy only and does not provide two-line-element parsing, checksums, field decoding, SGP4, TEME transforms, propagation, operational tracking, or parity.
- BioSim docs/contracts, flat resource/tick primitives, corrected B2a scenario-domain structural validation, B2b-1 process/intent-planner helpers, B2b-2 bounded replay/digest/event helpers, and B2c replay-integrity/ledger/report/example helpers remain research-only and incomplete as a full scenario engine or controller; no flat-resource adapter exists in B2c, B2b-3 is skipped/not required for the deployed B2c consumer path, and no biological-fidelity, habitat-safety, medical, operational, parity, certification, or regulated-use claim is made.
- Session F provides Orekit reference-oracle planning metadata only and does not provide certified Orekit parity.
- M07 material remains quarantined source material and is not bulk-imported.

## Minimum friend-test acceptance criteria

A public friend-test report is useful if it includes:

- platform and shell;
- Rust and Cargo versions;
- commit hash if available;
- whether the Bash script, PowerShell script, or manual command list was used;
- the first failing command and nearby terminal output, or a note that all checks completed;
- whether a root `Cargo.lock` appeared after the run.

A clean report is still not a safety or certification claim.
