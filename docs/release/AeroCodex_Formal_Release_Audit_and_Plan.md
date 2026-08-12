# AeroCodex Formal Release Audit and Workable-Release Plan

**Reviewed artifact:** `AeroCodex-main.zip`  
**Review date:** 2026-08-11  
**Recommended first formal release:** `v0.1.0-alpha.1`  
**Recommended release type:** Public research-software alpha, distributed through GitHub Releases first

## Executive assessment

AeroCodex is not an empty prototype. It is a substantial, carefully governed Rust research-code repository with approximately 40,700 lines of Rust, 14 Cargo workspace packages, about 540 `#[test]` functions, 152 governed formula-registry rows, extensive provenance material, conservative safety language, and a serious clean-room/source-boundary posture.

The project is nevertheless **not ready for a formal workable release in its uploaded state**. The principal problem is not lack of code. It is that the release-facing surfaces disagree with each other:

1. The repository checksum gate fails for eight governed files.
2. The generated formula registry marks all 152 formulas `research_required` with `execution_policy=blocked`.
3. The CLI has 12 dispatch specifications, but normal `formula run` execution is blocked by the registry status gate.
4. The public friend-test script expects a canonical formula run to succeed, so it cannot pass even after the checksum file is repaired unless the execution policy is also fixed.
5. The built-in self-check evaluates formulas through a private evaluator and bypasses the public status-gated execution route, so a green self-check does not prove that users can run a formula.
6. Documentation still describes ten executable formulas and a 14-check self-check, while the current CLI reports twelve dispatch specifications and intentionally blocks normal execution.
7. There is no formal tag-driven release workflow, binary packaging matrix, signed/attested checksum bundle, SBOM, release notes process, or published release artifact.
8. Cargo package metadata and path-dependency declarations are not ready for a crates.io publication sequence.

The right next move is **not** to expand the equation inventory. Freeze new families temporarily and convert the existing foundation into one honest, reproducible, usable alpha vertical slice.

## What is already strong

### Engineering and architecture

- Pure Rust workspace with `unsafe_code = "forbid"` at the workspace level and crate-level `#![forbid(unsafe_code)]` declarations.
- Clear crate separation across core, constants, atmosphere, thermo, gas dynamics, aerodynamics, propulsion, heat transfer, structures, flight dynamics, astrodynamics, life support, CLI, and repository tooling.
- Stable structured error codes in `aero-codex-core`.
- Checked numerical/domain failures in many public kernels.
- Source traceability IDs, validation records, formula contracts, inventories, schemas, and deterministic generated registries.
- Explicit source boundaries for M07, Orekit, BioSim, and thin-film materials.

### Testing and governance

- Approximately 540 Rust test functions and no ignored tests found by static inventory.
- CI includes formatting, checking, Clippy with warnings denied, tests, documentation generation, formula-registry consistency, equation-batch checks, dependency policy, and CLI self-check.
- Extensive negative-path tests for CLI parsing and status gates.
- Conservative public safety wording is unusually well developed.
- Formula status and execution policy are separated conceptually, which is the correct direction even though the current release slice is not yet reconciled.

### Documentation and research posture

- Strong internal documentation for provenance, assurance, source intake, validation, formula promotion, and roadmap history.
- The repository correctly avoids equating software build success with physical validity, mission readiness, certification, or operational approval.
- The M07 accounting work is visibly quarantined rather than silently represented as implemented capability.

## Verified release blockers

### P0-1: Governed checksum bundle is stale

Running:

```bash
sha256sum -c checksums/SHA256SUMS
```

returns failure. There are 419 entries: 411 pass and eight fail.

| Mismatched file |
|---|
| `.github/workflows/ci.yml` |
| `README.md` |
| `crates/aero-codex-cli/src/main.rs` |
| `crates/aero-codex-cli/tests/cli.rs` |
| `docs/beta1/cli_quickstart.md` |
| `docs/index.md` |
| `docs/roadmap/public_alpha_readiness_dashboard.md` |
| `xtask/src/main.rs` |

This is a release blocker because `scripts/friend_test_local.sh` and the documented release procedure require the checksum command to pass. The blocking CI workflow does not currently run the full checksum command, which explains how stale checksums can coexist with a green Rust workflow.

### P0-2: The public CLI is intentionally non-executable

`generated/formula_registry.json` contains:

- 152 formulas total;
- 152 with `status=research_required`;
- 152 with `execution_policy=blocked`;
- 152 with `quarantine_state=below_execution_threshold`.

`crates/aero-codex-cli/src/main.rs` applies the status gate before formula dispatch. The integration tests explicitly require canonical M00 execution to fail with exit code 4 and `execution_blocked_by_status`.

This means the documented command:

```bash
aerocodex formula run m00.canonical.distance_to_canonical ...
```

is not a working user feature in the current state.

### P0-3: The friend-test cannot reach its success message

`scripts/friend_test_local.sh` requires both:

```bash
sha256sum -c checksums/SHA256SUMS
```

and:

```bash
cargo run -p aero-codex-cli -- run \
  formula_vault.m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
```

The first fails in the uploaded archive. The second is expected by current integration tests to fail because the formula remains `research_required` and blocked. The script therefore cannot pass as documented.

### P0-4: The self-check bypasses the public execution gate

`run_self_check()` calls the internal formula evaluator directly. It does not exercise `execute_run()` and does not pass through the generated registry execution policy. As a result:

- self-check can report 14 passes;
- ordinary user execution of the same formula can still be blocked;
- the release gate validates math dispatch internals but not the actual public execution path.

For a formal release, self-check must use the same resolver, input validation, status policy, dispatch, and output envelope as user-facing execution, or it must be renamed to make its narrower meaning explicit.

### P0-5: Release documentation has drifted from implementation

Examples of current drift:

- Documentation repeatedly states that ten formulas are exposed.
- CLI tests require `supported_formula_count=12`.
- CLI help describes ten canonical formulas plus two angle dispatch specifications behind a gate.
- The generated registry reports 152 formulas but zero normally executable formulas.
- The dashboard describes CLI-accessible formulas even though the status gate blocks normal execution.
- Some golden fixture files still say generator commands are `not_implemented` or are placeholders even though corresponding generator/report functionality now exists.

This is exactly the kind of drift a generated release manifest should eliminate.

### P0-6: No formal release automation or artifact chain

The repository has no tag-triggered release workflow that produces installable artifacts. Missing release capabilities include:

- version/tag consistency check;
- cross-platform release builds;
- archive creation;
- artifact smoke tests;
- SHA-256 release checksum file;
- signature or provenance attestation;
- SBOM;
- release notes generation;
- GitHub Release publication;
- post-publication installation test.

The current Beta 1 documents explicitly state that they do not tag, package, upload, sign, or publish a release.

### P0-7: Cargo packages are not publication-ready

All workspace packages currently lack package-level publication metadata such as descriptions, readmes, documentation URLs, keywords, and categories. Internal path dependencies do not include version requirements. For example, domain crates depend on `aero-codex-core` using only a path.

Before crates.io publication:

- internal path dependencies need both `path` and `version`;
- publication order must be defined;
- tooling crates such as `xtask` should be `publish = false`;
- the life-support crate should remain unpublished until its release scope and license/source-boundary audit are explicitly approved;
- crate package contents should be inspected with `cargo package --list` and `cargo package`.

### P0-8: Binary reproducibility conflicts with the no-lockfile policy

The repository intentionally removes the root `Cargo.lock`. That is defensible for a library-only workspace, but the project now has a distributed binary CLI. A formal binary release needs a reproducible dependency graph. The policy should change to commit the root lockfile for release builds, while continuing to test library compatibility against the declared dependency ranges.

### P1-1: CI is strong but not yet a release matrix

Current blocking CI is Ubuntu/stable only. A formal alpha should additionally exercise:

- the declared MSRV;
- Windows, because Windows is a documented supported/non-broken target and is a primary user environment;
- macOS at least as a non-blocking or Tier 2 target;
- `RUSTDOCFLAGS="-D warnings"` in CI;
- the full checksum gate;
- clean-package tests;
- artifact smoke tests.

The all-manifest equation-batch verifier remains diagnostic/non-blocking. That is reasonable for the quarantined backlog, but the exact release slice needs a separate blocking verifier.

### P1-2: Registry and dispatch are duplicate sources of truth

The CLI maintains manual `FormulaSpec` dispatch metadata while also consuming a generated registry. This duplication caused the current count and execution mismatch. A formal release should derive, generate, or validate all of the following from one governed release manifest:

- canonical formula ID;
- aliases;
- inputs and outputs;
- runtime symbol;
- status;
- execution policy;
- CLI availability;
- self-check vectors;
- public count fields;
- documentation tables.

### P1-3: Community, security, and research citation surfaces are incomplete

The repository lacks root-level:

- `CHANGELOG.md`;
- `CONTRIBUTING.md`;
- `CODE_OF_CONDUCT.md`;
- `SECURITY.md`;
- `SUPPORT.md` or equivalent support policy;
- `CITATION.cff`;
- an explicit release checklist.

These are not mathematical-validation artifacts, but they are part of a professional public release.

## Recommended formal release definition

### Release name

**AeroCodex `v0.1.0-alpha.1` — Research Software Alpha**

This should replace the ambiguous `Phase 0.001`/`0.0.1` external release identity. Historical roadmap phase language may remain in archived planning documents, but the user-facing package and release version should follow SemVer.

### Honest product promise

The alpha should promise only:

> A source-traceable, deterministic Rust CLI and library foundation for research, education, verification-oriented development, and preliminary engineering calculations. The alpha includes a small explicitly promoted M00 conversion slice and a larger visible but blocked research inventory. It is not certified or approved for operational or safety-critical use.

### Executable alpha slice

Promote and expose **exactly 12 M00 scalar conversion formulas** already represented by CLI dispatch specifications:

- 10 canonical distance/time/speed/unit conversions;
- degrees to radians;
- radians to degrees.

Why this slice:

- implementation already exists;
- formulas are simple enough for independent analytical validation;
- units and domains are clear;
- cross-platform behavior is straightforward to test;
- it proves the entire governed path without overclaiming the 152-row registry.

All other registry entries remain visible and blocked.

`m00_wrap2pi` should be deferred to `alpha.2` unless its endpoint policy, periodicity properties, and CLI schema are completed in the same release branch.

### Distribution scope

For `alpha.1`:

- publish source and prebuilt `aerocodex` binaries through GitHub Releases;
- do not publish all domain crates to crates.io yet;
- optionally publish no crates at all until `alpha.2`;
- mark `xtask` and non-release crates `publish = false` immediately so accidental publication is impossible.

This creates a formal usable release without forcing a rushed 13-crate publication sequence.

### Supported platforms

Recommended initial platform policy:

- Tier 1: Windows x86_64 and Linux x86_64;
- Tier 2: macOS arm64 and macOS x86_64;
- source builds on Rust-compatible platforms remain best effort unless added to CI.

## Formal release work plan

The plan is ordered so that each milestone leaves the repository more truthful and testable than before it.

## Milestone R0 — Freeze scope and establish one release authority

**Goal:** Stop feature expansion and define exactly what `v0.1.0-alpha.1` means.

### Tasks

- **R0-01:** Create a release charter for `v0.1.0-alpha.1`.
- **R0-02:** Lock the executable formula list to the 12 M00 conversion formulas.
- **R0-03:** Declare every other formula non-release and blocked.
- **R0-04:** Define the status transition evidence required for `implementation_verified`.
- **R0-05:** Add a machine-readable `release/release-manifest.toml` containing version, channel, formula IDs, supported targets, status requirements, and artifact names.

### Exit criteria

- One document and one machine-readable manifest define the release.
- No README, CLI count, test, or workflow hardcodes a competing release count.
- New formula-family work is deferred until the alpha release gate is green.

## Milestone R1 — Repair repository integrity and generated-state trust

**Goal:** Make every documented repository verification command pass from a clean checkout.

### Tasks

- **R1-01:** Determine whether each of the eight checksum mismatches is an intended change.
- **R1-02:** Regenerate `checksums/SHA256SUMS` from the approved committed state.
- **R1-03:** Add `sha256sum -c checksums/SHA256SUMS` to blocking Linux CI.
- **R1-04:** Add an `xtask checksum check` command with a platform-independent implementation for Windows CI.
- **R1-05:** Add a deterministic generated-file freshness gate covering formula registry, Rust registry code, status report, and checksums.
- **R1-06:** Replace obsolete golden placeholders or rename them clearly as historical fixtures.
- **R1-07:** Make `scripts/friend_test_local.sh` and its PowerShell counterpart use the same generated release manifest.

### Exit criteria

- Full checksum verification passes.
- Deliberately changing a governed file without refreshing hashes fails CI.
- Bash and PowerShell friend tests agree on commands and expected release counts.

## Milestone R2 — Make the CLI genuinely usable

**Goal:** Ensure the public command path can execute the promoted release slice and that all reported counts are truthful.

### Tasks

- **R2-01:** Complete promotion packets for the 12 release formulas.
- **R2-02:** Change their registry status to `implementation_verified` and generated execution policy to `normal_research`.
- **R2-03:** Keep the remaining 140 registry rows blocked.
- **R2-04:** Remove manual count claims; compute registry, dispatchable, and executable counts separately.
- **R2-05:** Generate or strictly validate CLI dispatch specifications against the registry.
- **R2-06:** Change self-check to execute through the same public resolver, status gate, input parser, evaluator, and output envelope used by `formula run`.
- **R2-07:** Add a negative test proving self-check fails if a release formula is blocked or missing from dispatch.
- **R2-08:** Make `formula list --executable` return exactly the promoted formulas.
- **R2-09:** Ensure `formula describe` clearly differentiates implemented, executable, validated, and blocked states.
- **R2-10:** Stabilize the JSON success and error schemas and add golden contract tests.

### Exit criteria

- A clean binary successfully runs all 12 release formulas.
- `self-check` proves the same public path works.
- `formula list --executable` reports 12.
- No blocked formula can execute with or without `--preliminary` unless its declared status policy permits it.

## Milestone R3 — Validate the release slice independently

**Goal:** Support the `implementation_verified` claim with evidence beyond implementation-local examples.

### Tasks

- **R3-01:** Add analytical reference vectors for every release formula.
- **R3-02:** Add round-trip properties for distance, time, speed, degrees, and radians.
- **R3-03:** Add identity tests at canonical unit scale 1.
- **R3-04:** Add signed-value tests, zero tests where valid, invalid-scale tests, nonfinite tests, and overflow tests.
- **R3-05:** Define absolute, relative, and ULP-style tolerance policy for conversions.
- **R3-06:** Add deterministic broad-domain property loops without requiring random seeds.
- **R3-07:** Add cross-platform edge vectors to detect architecture/compiler differences.
- **R3-08:** Link each test vector to its formula contract, validation card, source record, and promotion packet.
- **R3-09:** Produce a generated validation summary for only the release slice.

### Exit criteria

- Every promoted formula has independent vectors and property evidence.
- Promotion evidence is machine-linked and reviewable.
- No broader physical-validity claim is made.

## Milestone R4 — Establish professional versioning and packaging

**Goal:** Produce reproducible installable artifacts from one tag.

### Tasks

- **R4-01:** Change workspace package version to `0.1.0-alpha.1`.
- **R4-02:** Add an explicit `rust-toolchain.toml` for release builds and separately test the declared MSRV.
- **R4-03:** Commit the root `Cargo.lock` for reproducible CLI release builds.
- **R4-04:** Add release profile settings intentionally and document them.
- **R4-05:** Add package descriptions, readmes, repository/documentation/homepage fields, keywords, and categories.
- **R4-06:** Add version requirements beside all internal path dependencies.
- **R4-07:** Set `publish = false` for `xtask` and any crate not approved for alpha publication.
- **R4-08:** Add `cargo package` checks for every publishable package.
- **R4-09:** Define release artifact names and archive layouts.
- **R4-10:** Embed version, Git commit, build target, and release manifest hash in `aerocodex version --json`.

### Exit criteria

- Tag, Cargo version, CLI version, release manifest version, and archive names match.
- A clean checkout produces byte-traceable artifacts from the committed lockfile.
- Accidental publication of tooling or quarantined crates is impossible.

## Milestone R5 — Expand CI into a release assurance pipeline

**Goal:** Test the supported platforms and build releasable artifacts automatically.

### Tasks

- **R5-01:** Add blocking stable Linux CI.
- **R5-02:** Add blocking declared-MSRV Linux CI.
- **R5-03:** Add blocking Windows CI for check, test, CLI contracts, and platform-independent checksum verification.
- **R5-04:** Add macOS CI as Tier 2 initially.
- **R5-05:** Enforce `RUSTDOCFLAGS="-D warnings"`.
- **R5-06:** Add release-slice equation verification as a blocking gate.
- **R5-07:** Keep all-backlog probe verification diagnostic until its cost and expected state are controlled.
- **R5-08:** Add dependency vulnerability and license-policy checking appropriate to the dependency set.
- **R5-09:** Add a tag-triggered release workflow that builds, archives, hashes, and smoke-tests every artifact.
- **R5-10:** Generate an SBOM and provenance/attestation record for each release.

### Exit criteria

- Every Tier 1 artifact is built by CI, not manually.
- Downloaded artifacts pass `version`, `formula list --executable`, one formula run, and `self-check` on clean runners.
- Release workflow cannot publish if the source tree, generated state, version, or checksum state is inconsistent.

## Milestone R6 — Create a user-facing project rather than an internal dossier

**Goal:** Keep the excellent assurance detail while giving users a clear front door.

### Tasks

- **R6-01:** Rewrite the README around installation, first successful formula run, supported scope, status meanings, and safety boundary.
- **R6-02:** Move historical phase/microtask material behind an archive index rather than presenting it as current release documentation.
- **R6-03:** Update CLI quickstart, expected outputs, dashboard counts, and friend-test docs from generated release data.
- **R6-04:** Add `CHANGELOG.md` using a consistent release-note structure.
- **R6-05:** Add `CONTRIBUTING.md` with formula promotion and code contribution paths.
- **R6-06:** Add `SECURITY.md` with private vulnerability reporting instructions and supported-version policy.
- **R6-07:** Add `CODE_OF_CONDUCT.md` and support expectations.
- **R6-08:** Add `CITATION.cff` and a research citation guide.
- **R6-09:** Publish API/user documentation from release tags or main.
- **R6-10:** Add a complete worked example showing formula ID, inputs, output, status, source trace, and caveat.

### Exit criteria

- A new user can install and run the alpha without reading roadmap history.
- A researcher can identify how to cite the software and formula evidence.
- A contributor can understand how a formula moves from candidate to executable status.

## Milestone R7 — Release candidate, external test, and formal publication

**Goal:** Prove the release outside the developer checkout and publish it with a clear rollback path.

### Tasks

- **R7-01:** Create `v0.1.0-alpha.1-rc.1` from a clean protected branch.
- **R7-02:** Build all artifacts through the release workflow.
- **R7-03:** Test installation and execution on clean Windows and Linux systems; test macOS Tier 2 artifacts.
- **R7-04:** Run the public friend-test package from downloaded source and binaries.
- **R7-05:** Verify archive checksums, SBOMs, provenance, embedded commit, and manifest hash.
- **R7-06:** Collect at least one external friend-test report and resolve release-blocking findings.
- **R7-07:** Cut the final `v0.1.0-alpha.1` tag from the reviewed RC commit.
- **R7-08:** Publish release notes with supported formulas, known limitations, status definitions, upgrade policy, and non-claims.
- **R7-09:** Perform post-publication download-and-smoke verification.
- **R7-10:** Open the `alpha.2` backlog only after the final release artifacts are verified.

### Exit criteria

- A public release exists with source, binaries, checksums, SBOM/provenance, and release notes.
- A user can successfully execute the 12-formula release slice from a downloaded artifact.
- The project does not imply broader formula validation, operational readiness, or certification.

## Formal alpha finish line

AeroCodex is ready for `v0.1.0-alpha.1` only when every statement below is true.

### Product behavior

- [ ] `aerocodex version --json` reports one consistent version, commit, target, channel, and release-manifest hash.
- [ ] `aerocodex formula list --executable --json` returns exactly 12 release formulas.
- [ ] Every release formula executes through the public command path.
- [ ] Every non-release registry row remains blocked.
- [ ] `aerocodex self-check --json` exercises the public path and reports zero failures.
- [ ] Stable JSON and exit-code contracts are tested.

### Repository integrity

- [ ] All generated artifacts are current.
- [ ] Full governed checksum verification passes.
- [ ] CI fails when a governed file changes without regenerated hashes.
- [ ] Release slice source, contract, card, test, and status links are complete.
- [ ] Root `Cargo.lock` is committed and release builds use it.

### Build and test

- [ ] Formatting, checking, Clippy, tests, docs-with-warnings-denied, xtask, registry, and checksum gates pass.
- [ ] Declared MSRV passes.
- [ ] Windows and Linux Tier 1 CI pass.
- [ ] Release archives pass smoke tests after extraction.
- [ ] Public friend-test scripts pass from clean source.

### Distribution and governance

- [ ] Tag, Cargo version, release manifest, and artifact names match.
- [ ] GitHub Release includes binaries, source, checksums, SBOM/provenance, and release notes.
- [ ] `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, and `CITATION.cff` exist.
- [ ] No unapproved crate can be accidentally published.
- [ ] Safety and certification caveats remain clear without overwhelming the first-use path.

## Recommended order of implementation

The most efficient sequence is:

1. **R0 scope freeze**
2. **R1 checksum and generated-state repair**
3. **R2 status/CLI/self-check reconciliation**
4. **R3 release-slice validation**
5. **R4 versioning and packaging**
6. **R5 CI and release workflow**
7. **R6 documentation/community surfaces**
8. **R7 release candidate and publication**

Do not begin broad M07 promotion, new life-support features, or additional formula-family ingestion until R2 is complete. Otherwise the project will continue accumulating governed inventory faster than it accumulates usable release behavior.

## Suggested first implementation batch

The first code batch should contain only these changes:

1. Add the machine-readable release manifest.
2. Repair and CI-enforce the checksum bundle.
3. Choose the 12 M00 formula statuses and create promotion packet stubs/evidence checklist.
4. Make CLI counts distinguish registry, dispatchable, and executable formulas.
5. Rewrite self-check to call the public execution path.
6. Update the friend-test to one canonical command path.
7. Update the most visible docs from generated release data.

That batch creates a truthful baseline. The following batch can then complete formula promotion evidence and enable execution.

## Overall conclusion

AeroCodex is closer to a **serious research-development platform** than a typical early prototype. Its strongest assets are traceability, conservative status language, modular Rust code, and extensive test/governance machinery. Its weakest area is the final mile between internal governance and an honest user-facing release.

A formal alpha is achievable without rewriting the project. The key is to narrow the promise, repair the trust chain, promote a small validated conversion slice, make self-check exercise the real public path, and automate reproducible cross-platform release artifacts. Once that is done, the existing formula registry and promotion framework can support disciplined family-by-family expansion.
