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

The current main counts below are verifier-derived and include Session G plus later Stage 5 work. Session G itself historically added `+1` validation card, `+1` source-registry seed, `+1` validation-card-only record, and `+0` to the other four governed counters; the absolute values here are the current-main values, not the old Session G branch-local snapshot.

| Inventory class | Current main count | Meaning |
|---|---:|---|
| Executable research equations | 151 | Public Rust research/preliminary-design equation kernels inventoried by `validation/equation_inventory.tsv`. |
| Metadata-only formula-vault candidates | 27 | Formula-vault candidate metadata records; not implementations by themselves. |
| External M07 backlog rows | 1,323 | Registered external M07 represented rows not yet selected as formula-vault candidates. C2 classification does not remove rows from this backlog. |
| Validation cards | 44 | Conservative validation/governance records. They are not certification evidence. |
| Source-registry seeds | 42 | Source/governance traceability seeds. |
| Validation-card-only records | 44 | Metadata records, not formula implementations. |
| Helper algorithms | 159 | Support routines not counted as executable research equations. |

## Public alpha lanes

| Lane | User-visible artifact | Current label | Blocked from promotion because |
|---|---|---|---|
| Local clone checks | `docs/testing/friend_test_quickstart.md`, `scripts/friend_test_local.sh`, `scripts/friend_test_local.ps1` | research_required | Local checks are software gates only and do not prove physical validity or safety. |
| Expected-output guide | `docs/testing/friend_test_expected_output.md` | research_required | Output varies by toolchain and is not validation evidence. |
| Safety caveats | `docs/testing/research_safety_caveats_for_testers.md` | research_required | Caveats must stay visible for public testers. |
| Equation inventory | `validation/equation_inventory.tsv` and `cargo run -p xtask -- verify equation-inventory` | research_required | Inventory rows classify items; they do not validate formulas. |
| Formula-vault metadata | `formula-vault/` and `cargo run -p xtask -- verify formula-vault` | research_required | Candidate records remain metadata-only until a later authorized implementation chunk. |
| Dependency policy | `cargo run -p xtask -- dependency-policy` | research_required | Dependency hygiene is not physical validation or safety approval. |
| Documentation build | `cargo doc --workspace --all-features --no-deps` | research_required | Documentation generation does not create validation evidence. |

## Known blocked public-facing items

- `wrap2pi` contract/test metadata exists, but executable/public runtime implementation remains blocked pending a separate endpoint-behavior decision.
- `app_resolve_coplanar` remains blocked for least-squares, rank, singularity, and tolerance policy.
- Orekit v3 O2a and O2b exist as research/preliminary-only foundations; O2c exists as local deterministic oracle-record/tolerance-comparison infrastructure only; O2d remains incomplete.
- BioSim docs/contracts and clean-room primitives remain research-only and incomplete as a full engine.
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
