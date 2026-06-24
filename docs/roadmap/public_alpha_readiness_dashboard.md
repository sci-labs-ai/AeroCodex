# Public alpha readiness dashboard

This dashboard is a tester-facing summary for the public friend-test package. It is not a release approval, certification artifact, safety case, mission-readiness case, habitat-safety case, medical-use case, or regulated-use approval.

AeroCodex is research/preliminary-design software. Passing local checks does not prove physical validity, safety, certification, or mission readiness.

## Dashboard scope

The public alpha friend-test package is intended to answer four narrow questions:

1. Can a tester clone the repository and run the configured local Rust and governance checks?
2. Does the equation inventory explain what is executable research code, metadata-only candidate material, validation-card-only metadata, and helper code?
3. Do testers see the project’s research-only and blocked status before using any formula?
4. Can failures be reported in a reproducible way without importing external source material or generated artifacts?

## Current main governed counts

The current main counts below are verifier-derived and include Session G, later Stage 5 work through BioSim B2c and the final Stage 5 closeout/status consolidation, plus the bounded Post-Stage-5 `m00_wrap2pi` runtime deployment. Session G itself historically added `+1` validation card, `+1` source-registry seed, `+1` validation-card-only record, and `+0` to the other four governed counters; the absolute values here are the current-main values, not the old Session G branch-local snapshot. The wrap2pi runtime deployment adds `+1` executable research equation and zero change to the other governed counters.

| Inventory class | Current main count | Meaning |
|---|---:|---|
| Executable research equations | 152 | Public Rust research/preliminary-design equation kernels inventoried by `validation/equation_inventory.tsv`. |
| Metadata-only formula-vault candidates | 27 | Formula-vault candidate metadata records; not implementations by themselves. |
| External M07 rows with terminal dispositions | 281 | A11-A18 record 121 aliases, 45 excluded internal helpers, and 115 contract-blocked rows. |
| External M07 backlog rows | 1,042 | Registered external M07 represented rows that still lack a terminal disposition. |
| Validation cards | 46 | Conservative validation/governance records. They are not certification evidence. |
| Source-registry seeds | 44 | Source/governance traceability seeds. |
| Validation-card-only records | 46 | Metadata records, not formula implementations. |
| Helper algorithms | 262 | Support routines not counted as executable research equations. |

## Public alpha lanes

| Lane | User-visible artifact | Current label | Blocked from promotion because |
|---|---|---|---|
| Local clone checks | `docs/testing/friend_test_quickstart.md`, `scripts/friend_test_local.sh`, `scripts/friend_test_local.ps1` | research_required | Local checks are software gates only and do not prove physical validity or safety. |
| Expected-output guide | `docs/testing/friend_test_expected_output.md` | research_required | Output varies by toolchain and is not validation evidence. |
| Safety caveats | `docs/testing/research_safety_caveats_for_testers.md` | research_required | Caveats must stay visible for public testers. |
| Equation inventory | `validation/equation_inventory.tsv` and `cargo run -p xtask -- verify equation-inventory` | research_required | Inventory rows classify items; they do not validate formulas. |
| Formula-vault and external-resolution metadata | `formula-vault/`, `python scripts/verify_formula_vault_resolutions.py --repo .`, and the configured `python scripts/verify_external_m07_*.py --repo .` gates | research_required | Candidate and resolution records reuse or block existing runtime concepts; they do not imply certification, source parity, or operational readiness. |
| Dependency policy | `cargo run -p xtask -- dependency-policy` | research_required | Dependency hygiene is not physical validation or safety approval. |
| Documentation build | `cargo doc --workspace --all-features --no-deps` | research_required | Documentation generation does not create validation evidence. |
| Beta 1 concept CLI | `aerocodex` commands and `cargo run -p aero-codex-cli -- self-check --json` | research_required | The vertical slice exposes ten governed canonical-unit formulas for software testing only; it is not the full 1,000+ equation program or an operational release. |

## Known blocked public-facing items

- `m00_wrap2pi` has a bounded executable/public Rust runtime for `formula_vault.m00.angle.wrap2pi`, remains research_required/non-certified, and makes no M07/Scilab parity claim; alternate aliases remain blocked.
- `app_resolve_coplanar` remains blocked for least-squares, rank, singularity, and tolerance policy.
- Orekit v3 O2a/O2b/O2c/O2d exist as bounded research/preliminary-only foundations and metadata helpers; O2d is contract/source-policy only and does not provide two-line-element parsing, checksums, field decoding, SGP4, TEME transforms, propagation, operational tracking, or parity.
- BioSim docs/contracts, flat resource/tick primitives, corrected B2a scenario-domain structural validation, B2b-1 process/intent-planner helpers, B2b-2 bounded replay/digest/event helpers, and B2c replay-integrity/ledger/report/example helpers remain research-only and incomplete as a full scenario engine or controller; no flat-resource adapter exists in B2c, B2b-3 is skipped/not required for the deployed B2c consumer path, and no biological-fidelity, habitat-safety, medical, operational, parity, certification, or regulated-use claim is made.
- Session F provides Orekit reference-oracle planning metadata only and does not provide certified Orekit parity.
- M07 material remains quarantined source material and is not bulk-imported.
- A11-A18 resolve eight bounded metadata waves: 121 aliases reuse existing governed runtimes, 45 internal/composite helpers are excluded from formula scope, and 115 rows remain contract-blocked. A14-A15 complete all 49 classical two-body algebra rows; A16-A18 process the first 120 orbital-geometry/conic rows and leave 257 rows in that classifier group. A18 retains 33 medium-risk and 7 high-risk labels, and the remaining 1,042-row external backlog is incomplete.

## Minimum friend-test acceptance criteria

A public friend-test report is useful if it includes:

- platform and shell;
- Rust and Cargo versions;
- commit hash if available;
- whether the Bash script, PowerShell script, or manual command list was used;
- the first failing command and nearby terminal output, or a note that all checks completed;
- whether a root `Cargo.lock` appeared after the run.

A clean report is still not a safety or certification claim.
