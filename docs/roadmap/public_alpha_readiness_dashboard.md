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
| External M07 rows with terminal dispositions | 855 | A11-A33 record 129 aliases, 103 excluded internal/composite helpers, and 623 contract- or policy-blocked rows. |
| External M07 backlog rows | 198 | Registered external M07 represented rows that still lack a terminal disposition. |
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
| Formula-vault and external-resolution metadata | `formula-vault/` and `cargo run -p xtask -- verify formula-vault` | research_required | Candidate and resolution records reuse or block existing runtime concepts; they do not imply certification, source parity, or operational readiness. |
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
- A11-A33 resolve twenty-three bounded metadata waves: 129 aliases reuse existing governed runtimes, 103 internal/composite helpers are excluded from formula scope, and 623 rows remain contract- or policy-blocked. A14-A15 complete all 49 classical two-body algebra rows; A16-A25 complete all 377 orbital-geometry/conic rows and leave 0 rows in that classifier group. A26-A27 complete the coordinate-transform/frame-graph/time-scale policy backlog, retain 58 medium-risk and 27 frame/time-policy blocked labels; A31-A33 complete the relative-motion and finite-burn scalar policy backlog and leave a 468-row external backlog incomplete.

## Minimum friend-test acceptance criteria

A public friend-test report is useful if it includes:

- platform and shell;
- Rust and Cargo versions;
- commit hash if available;
- whether the Bash script, PowerShell script, or manual command list was used;
- the first failing command and nearby terminal output, or a note that all checks completed;
- whether a root `Cargo.lock` appeared after the run.

A clean report is still not a safety or certification claim.

### A34 external M07 attitude / inertia / quaternion policy Wave 1

A34 is metadata-only and blocks 40 attitude / inertia / quaternion source rows pending representation, frame, source-registry, and validation-oracle policy. External M07 counters after A34: 895 processed / 428 backlog.


### A35 external M07 attitude / inertia / quaternion policy Wave 2

A35 is metadata-only and blocks the remaining 19 attitude / inertia / quaternion source rows pending representation, frame, source-registry, and validation-oracle policy. External M07 counters after A35: 914 processed / 409 backlog.

### A36 external M07 attitude dynamics/control policy Wave 1

A36 is metadata-only and blocks 38 attitude dynamics/control source rows pending torque, inertia, integration, frame, source-registry, and validation-oracle policy. External M07 counters after A36: 952 processed / 371 backlog.

### A37 external M07 J2 perturbation / numerical propagation policy Wave 1

A37 is metadata-only and blocks 40 J2 perturbation / numerical-propagation source rows pending force-model, frame/time, source-registry, numerical integration, and validation-oracle policy. External M07 counters after A37: 992 processed / 331 backlog.

### A38 external M07 J2 perturbation / numerical propagation policy Wave 2

A38 is metadata-only and blocks 40 J2 perturbation / numerical-propagation source rows pending force-model, frame/time, source-registry, numerical integration, and validation-oracle policy. External M07 counters after A38: 1032 processed / 291 backlog.

### A39 external M07 J2 perturbation / numerical propagation policy Wave 3

A39 is metadata-only and blocks 48 J2 perturbation / numerical-propagation source rows pending force-model, frame/time, source-registry, numerical integration, and validation-oracle policy. External M07 counters after A39: 1080 processed / 243 backlog.


### A40 external M07 SGP4 / TEME frame-time policy Wave 1

A40 is metadata-only and records 45 SGP4/TEME frame-time policy terminal dispositions or helper exclusions. External M07 counters after A40: 1125 processed / 198 backlog.
### A41 external M07 CR3BP / external-data / input-output policy Wave 1

Status: research-required metadata only. A41 adds no public API and no readiness promotion; it records 45 terminal dispositions and leaves 153 external M07 backlog rows.
### A42 classifier-refresh / manual source-review policy Wave 1

Status: research-required metadata only. A42 records 45 terminal dispositions and leaves 108 external M07 backlog rows. No public API readiness promotion is made.

### A43 scalar/unit helper policy Wave 1

Status: research-required metadata only. A43 records 45 terminal dispositions and leaves 63 external M07 backlog rows. No public API readiness promotion is made.

### A44 residual scalar/unit/helper policy Wave 1

Status: research-required metadata only. A44 records 45 terminal dispositions and leaves 18 external M07 backlog rows. No public API readiness promotion is made.

### A45 final residual backlog closure Wave 1

Status: research-required metadata only. A45 records 18 terminal dispositions and leaves zero external M07 backlog rows. No public API readiness promotion is made.
