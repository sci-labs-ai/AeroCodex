# Friend-test expected output

This page describes the expected shape of the friend-test run. Exact Cargo output varies by Rust version, platform, and cache state, so testers should compare command success/failure rather than matching every line exactly.

AeroCodex is research/preliminary-design software. These outputs are local software-gate signals only. They are not physical-validation evidence, safety evidence, certification evidence, mission-readiness evidence, habitat-safety evidence, medical-use evidence, or regulated-use approval.

## Script output shape

The Bash and PowerShell scripts print a short banner, the repository root, Rust tool versions when available, the selected Python command, the current Git commit when available, and eighteen numbered check steps. A typical clean run has this structure:

```text
[friend-test] AeroCodex local friend-test package
[friend-test] repository root: <path-to-clone>
[friend-test] rustc: <version>
[friend-test] cargo: <version>
[friend-test] python command: <python-or-python3> (<version>)
[friend-test] git commit: <short-sha>
[friend-test] step 1/18: git status --short
[friend-test] step 2/18: git diff --check
[friend-test] step 3/18: sha256sum -c checksums/SHA256SUMS
[friend-test] step 4/18: cargo fmt --all -- --check
[friend-test] step 5/18: cargo check --workspace --all-targets --all-features
[friend-test] step 6/18: cargo clippy --workspace --all-targets --all-features -- -D warnings
[friend-test] step 7/18: cargo test --workspace --all-targets --all-features
[friend-test] step 8/18: cargo run -p aero-codex-cli -- version --json
[friend-test] step 9/18: cargo run -p aero-codex-cli -- run canonical distance smoke
[friend-test] step 10/18: cargo run -p aero-codex-cli -- self-check --json
[friend-test] step 11/18: python scripts/verify_governance.py --repo .
[friend-test] step 12/18: cargo run -p xtask -- dependency-policy
[friend-test] step 13/18: <python> scripts/verify_thinfilm_artifact.py
[friend-test] step 14/18: <python> nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature
[friend-test] step 15/18: <python> nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json
[friend-test] step 16/18: <python> nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl
[friend-test] step 17/18: git diff --exit-code nomenclature/generated/terminology/index.jsonl
[friend-test] step 18/18: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
[friend-test] completed all requested local checks
```

If a step fails, the scripts stop at that step and return a non-zero exit code. The first failing command is usually the most useful item to report.

## Expected command outcomes

| Step | Expected local outcome | What it means |
|---|---|---|
| `git status --short` | exits successfully and should print no tracked changes in a clean clone | The working tree has no unexpected tracked edits before tests. |
| `git diff --check` | exits successfully | No whitespace errors are present in tracked diffs. |
| `sha256sum -c checksums/SHA256SUMS` | exits successfully | Governed checksum-listed files match the manifest. |
| `cargo fmt --all -- --check` | exits successfully | Rust formatting matches the checked-in style. |
| `cargo check --workspace --all-targets --all-features` | exits successfully | The workspace type-checks under all targets/features. |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exits successfully | Lints configured as warnings did not fire for the workspace under the selected toolchain. |
| `cargo test --workspace --all-targets --all-features` | exits successfully | Workspace tests passed under the selected toolchain and platform. |
| `cargo run -p aero-codex-cli -- version --json` | exits successfully | The Beta 1 concept binary reports its bounded release identity and research-only status. |
| `cargo run -p aero-codex-cli -- run ... --json` | exits successfully and reports `canonical_distance=-6` in JSON | One exact signed conversion reaches the existing checked Rust kernel through the user-facing CLI. |
| `cargo run -p aero-codex-cli -- self-check --json` | exits successfully with `passed=14` and `failed=0` | All ten supported formulas and four fail-closed negative cases pass the bounded CLI smoke gate. |
| `python scripts/verify_governance.py --repo .` | exits successfully | Formula-vault runtime resolutions, A11-A33 external dispositions, and the configured xtask governance verifiers completed as a group. |
| `cargo run -p xtask -- dependency-policy` | exits successfully | The workspace did not add dependency tokens blocked by the current policy. |
| `python scripts/verify_thinfilm_artifact.py` | exits successfully, or `python3` fallback succeeds when bare `python` is unavailable | Thin-film governed artifact checks pass. |
| `python nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature` | exits successfully | Nomenclature policy lint passes. |
| `python nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json` | exits successfully | New acronym inventory entries remain controlled. |
| `python nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl` | exits successfully | Terminology export regenerates. |
| `git diff --exit-code nomenclature/generated/terminology/index.jsonl` | exits successfully | Regenerated terminology matches the checked-in file. |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps` | exits successfully | Workspace documentation builds with warnings denied. |

## Current governed inventory snapshot

These current-main counts come from the governed equation-inventory verifier and include Session G, later Stage 5 work, and the bounded post-Stage-5 `m00_wrap2pi` runtime deployment:

| Inventory class | Current main count | Meaning |
|---|---:|---|
| Executable research equations | 152 | Public Rust research/preliminary-design equation kernels inventoried by `validation/equation_inventory.tsv`. |
| Metadata-only formula-vault candidate records | 27 | Intake/provenance records; 27/27 are runtime-linked and 0 remain unresolved. The records are not implementations by themselves. |
| External M07 rows with terminal dispositions | 855 | A11-A33 record 129 aliases, 103 excluded internal/composite helpers, and 623 contract- or policy-blocked rows. |
| External M07 backlog rows | 198 | Registered external M07 represented rows that still lack a terminal disposition. |
| Validation cards | 46 | Conservative validation/governance records. They are not certification evidence. |
| Source-registry seeds | 44 | Source/governance traceability seeds. |
| Validation-card-only records | 46 | Metadata records, not formula implementations. |
| Helper algorithms | 262 | Support routines not counted as executable research equations. |

The historical Session G deltas were `+0` executable research equations, `+0` formula-vault candidates, `+0` external M07 backlog rows, `+1` validation card, `+1` source-registry seed, `+1` validation-card-only record, and `+0` helper algorithms. The absolute values above are no longer the old Session G branch-local counts.

## Expected blocked states

A clean friend-test run still leaves blocked and research-only items blocked. In particular:

- `m00_wrap2pi` has contract/test metadata plus bounded executable/public runtime coverage for all 26 deployed endpoint vectors, while remaining research_required and non-certified; alternate aliases remain blocked;
- `app_resolve_coplanar` remains blocked pending least-squares, rank, and tolerance policy;
- Orekit O2a/O2b/O2c/O2d exist as bounded research/preliminary-only foundations and metadata helpers; O2d is contract/source-policy only and does not parse two-line-element records, implement checksums, decode fields, run SGP4, perform TEME transforms, propagate orbits, track objects operationally, or claim parity;
- BioSim B2a exists only as a clean-room scenario-domain and deterministic structural-validation foundation; B2b-1 exists only as process/validated-constructor/one-tick-intent-planner helpers; B2b-2 exists only as bounded compartment replay, compact noncryptographic digest, and immutable atomic replay-event helpers; B2c exists only as fail-closed replay-integrity validation, deterministic committed-event ledger accounting, clamp accounting, path-safe report formatting, one synthetic example, and governance; no flat-resource adapter, full engine, controller, biological-fidelity, habitat-safety, medical, operational, parity, certification, or regulated-use claim exists; B2b-3 is skipped/not required for the deployed B2c consumer path;
- all 27 current formula-vault candidate formula IDs resolve to existing governed runtimes, while the candidate files remain metadata-only provenance records and validation remains `research_required`;
- the A11 verifier reports its 38-row unit-conversion wave;
- the A12 verifier reports its first 40 vector-helper rows;
- the A13 verifier reports the remaining 34 vector-helper rows with 26 aliases, 5 excluded internal helpers, and 3 contract blocks;
- the A14 verifier reports its first 40 classical two-body algebra rows and the A15 verifier reports the remaining 9 rows; together they cover all 49 rows with 22 aliases and 27 contract blocks;
- the A16 verifier reports the first 40 orbital-geometry/conic rows with 2 aliases, 10 helper exclusions, 28 contract blocks, 337 group rows remaining, an unchanged medium-risk classifier label, and an aggregate remaining backlog of 1,122;
- the A17 verifier reports the next 40 orbital-geometry/conic rows with 3 aliases, 15 helper exclusions, 22 contract blocks, 297 group rows remaining, 38 medium-risk plus 2 high-risk labels retained, and an aggregate remaining backlog of 1,082;
- the A18 verifier reports the third 40 orbital-geometry/conic rows with 1 alias, 7 helper exclusions, 32 contract blocks, 257 group rows remaining, 33 medium-risk plus 7 high-risk labels retained, and an aggregate remaining backlog of 1,042;
- the A19 verifier reports the fourth 40 orbital-geometry/conic rows with 3 aliases, 8 helper exclusions, 29 contract or policy blocks, 217 group rows remaining, 37 medium-risk plus 3 high-risk labels retained, and an aggregate remaining backlog of 1,002;
- validation cards remain conservative records and do not prove implementation or validation by themselves.

## Failure-report template

```text
OS and version:
Shell:
rustc --version:
cargo --version:
Repository commit:
Friend-test path: Bash script / PowerShell script / manual
First failing command:
Exit code:
Terminal excerpt:
Did Cargo generate a root Cargo.lock?: yes/no/unknown
Additional notes:
```

- the A20 verifier reports the fifth 40 orbital-geometry/conic rows with 0 aliases, 10 helper exclusions, 30 contract or policy blocks, 177 group rows remaining, 26 medium-risk plus 14 high-risk labels retained, and an aggregate remaining backlog of 962;

- the A21 verifier reports the sixth 40 orbital-geometry/conic rows with 1 alias, 13 helper exclusions, 26 contract or policy blocks, 137 group rows remaining, 18 medium-risk plus 22 high-risk labels retained, and an aggregate remaining backlog of 922;

- the A22 verifier reports the seventh 40 orbital-geometry/conic rows with 1 alias, 17 helper exclusions, 22 contract or policy blocks, 97 group rows remaining, 38 medium-risk plus 2 high-risk labels retained, and an aggregate remaining backlog of 882;

- the A23 verifier reports the eighth 40 orbital-geometry/conic rows with 1 alias, 10 helper exclusions, 29 contract or policy blocks, 57 group rows remaining, 30 medium-risk plus 10 high-risk labels retained, and an aggregate remaining backlog of 842;

- the A24 verifier reports the ninth 40 orbital-geometry/conic rows with 0 aliases, 0 helper exclusions, 40 contract or policy blocks, 17 group rows remaining, 34 medium-risk plus 6 high-risk labels retained, and an aggregate remaining backlog of 802;

- the A25 verifier reports the final 17 orbital-geometry/conic rows with 0 aliases, 0 helper exclusions, 17 contract or policy blocks, 0 group rows remaining, 14 medium-risk plus 3 high-risk labels retained, and an aggregate remaining backlog of 785;

- the A26 verifier reports the first 40 coordinate-transform / frame-graph / time-scale policy rows with 0 aliases, 0 helper exclusions, 40 contract or policy blocks, 45 candidate-pool rows remaining, 29 medium-risk plus 11 frame/time-policy blocked labels retained, and an aggregate remaining backlog of 745;

- the A27 verifier reports the remaining 45 coordinate-transform / frame-graph / time-scale policy rows with 0 aliases, 0 helper exclusions, 45 contract or policy blocks, 0 candidate-pool rows remaining, 29 medium-risk plus 16 frame/time-policy blocked labels retained, and an aggregate remaining backlog of 700;


A34 attitude frame policy Wave 1 is metadata-only; friend-test runtime output is unchanged.

A35 attitude frame policy Wave 2 is metadata-only; friend-test runtime output is unchanged.

A36 attitude dynamics/control policy Wave 1 is metadata-only; friend-test runtime output is unchanged.

A37 J2 perturbation / numerical propagation policy Wave 1 is metadata-only; friend-test runtime output is unchanged.

A38 J2 perturbation / numerical propagation policy Wave 2 is metadata-only; friend-test runtime output is unchanged.

A39 J2 perturbation / numerical propagation policy Wave 3 is metadata-only; friend-test runtime output is unchanged.

A40 SGP4 / TEME frame-time policy Wave 1 is metadata-only; friend-test runtime output is unchanged.
A41 external M07 metadata-only wave: processed/backlog counters 1170/153; no executable output changes expected.
A42 external M07 metadata-only wave: processed/backlog counters 1215/108; no executable output changes expected.

A43 external M07 metadata-only wave: processed/backlog counters 1260/63; no executable output changes expected.

A44 external M07 metadata-only wave: processed/backlog counters 1305/18; no executable output changes expected.
