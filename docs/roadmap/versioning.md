# AeroCodex Versioning and Roadmap Phase Lock

<!-- aerocodex-current-identity:start -->
Release version: `0.1.0-alpha.1`
Release tier: `research_software_alpha` (`Research Software Alpha`)
Workspace packages: `14`
Registry formulas: `152`
Blocked formulas: `140`
Publicly executable formulas: `12`
<!-- aerocodex-current-identity:end -->

Status: current release-identity policy for v0.1.0-alpha.1.

This document separates AeroCodex's human roadmap language from Cargo-compatible semantic versioning. The distinction is intentional and must be preserved during Phase 0.001 work.

## Version terms

| Term | Meaning | Current value |
| --- | --- | --- |
| Human roadmap phase | Planning and scope marker used in docs, issues, prompts, validation cards, and release notes. | `Phase 0.001` |
| Semantic version | SemVer-compatible package and release version used by all workspace Cargo manifests. | `0.1.0-alpha.1` |
| Prerelease component | The SemVer suffix that orders this build before `0.1.0`; it is not a distribution route or maturity tier. | `alpha.1` |
| Release channel | Intended distribution route, distinct from SemVer and maturity. It does not assert that a release or artifact has been published. | `github_releases` |
| Machine release tier | Machine-readable maturity label, separate from version, prerelease, channel, validation, and execution policy. | `research_software_alpha` (`Research Software Alpha`) |
| Publication state | Whether declared release outputs actually exist. Current artifact declarations are future expectations only. | `declared_only`; nothing packaged or published |
| Public API stability | Compatibility promise for downstream users. | Not promised in Phase 0.001 |

Phase 0.001 is not a Cargo version. Do not write `0.001` in any `Cargo.toml` package version.

## Cargo version lock

For the current release:

- The root `[workspace.package]` version is `0.1.0-alpha.1` and is the compile-time semantic-version authority.
- Workspace members should inherit that value with `version.workspace = true`.
- A crate may not independently publish a different package version without an explicit later roadmap decision.
- The `xtask` package also inherits the workspace version so repository tooling remains version-aligned.
- Tags, release notes, and generated bundles may mention `Phase 0.001`, but Cargo metadata must remain SemVer-compatible.

The next Cargo version must be chosen deliberately after the project has a tested, reviewed release process. Phase labels such as `0.01`, `0.1`, and `0.5` are roadmap milestones, not automatic Cargo package versions.

## Historical Beta 1 concept channel

`beta1-concept` was a bounded historical software milestone label for testing the user-facing CLI, deterministic output, negative paths, friend-test workflow, and release gates. It is retained only as a documentation label and compatibility alias; it is not the current runtime identity, a Cargo version, roadmap phase promotion, validation-status promotion, API-stability promise, or operational-readiness claim.

During that historical milestone:

- the historical Cargo version was `0.0.1`;
- validation remains `research_required`;
- the initial self-check pilot directly exercises ten governed M00 canonical-unit kernels;
- the CLI had twelve M00 dispatch-linked records but zero formulas passed the public execution gate while status remained `research_required`;
- release artifacts must identify one Git commit and pass the CLI self-check;
- later bulk equation ingestion remains automated-and-exception-reviewed work, not a claim that the 1,000+ backlog is complete.

## Roadmap ladder

| Roadmap level | Intent | Readiness boundary |
| --- | --- | --- |
| Phase 0.001 | Planning, first equations, source registry, testing scaffold. | No public API stability or operational readiness. |
| Phase 0.01 | Coherent multi-category equation set. | Still pre-alpha; validation coverage remains incomplete. |
| Phase 0.1 | Early public alpha. | APIs may still change; source traceability and tests improve. |
| Phase 0.5 | Broad validation beta. | Validation should be materially broader, but certification is still not implied. |
| Phase 1.0 | Stable verified core API. | Stable core API target, subject to documented validation and release gates. |
| Post-1.0 | Advanced thermo, astrodynamics, aeroelasticity, controls, optimization. | Future expansion after stable core. |
| Beyond 1.0 | High-fidelity modules, uncertainty, agentic optimization, generated reports. | Future capabilities only; not part of the Phase 0.001 promise. |

## Readiness and certification language

Phase 0.001 does not imply public API stability, broad validation, certification, flight readiness, mission readiness, aircraft approval, spacecraft approval, or operational suitability.

AeroCodex is not certified, flight-ready, mission-ready, or approved for aircraft or spacecraft operations.

AeroCodex is an engineering mathematics library for research, education, verification-oriented development, and preliminary design. Safety-critical, regulated, or mission use requires project-specific assurance, validation, qualification, and certification.

For avoidance of doubt, AeroCodex is not certified, not flight-ready, not mission-ready, and not approved for aircraft or spacecraft operations during Phase 0.001.

## Release-gate checklist before any later readiness claim

A later milestone may only strengthen readiness language after the repository has evidence for it. At minimum, future release reviews should confirm:

- every workspace package version is intentionally selected and SemVer-compatible;
- public APIs have documented assumptions, validity ranges, and failure modes;
- validation cards match implemented equations and source registry entries;
- source statuses are not upgraded beyond available evidence;
- formatting, clippy, tests, xtask checks, dependency-policy checks, and documentation builds pass in a Rust-enabled environment;
- the dual `MIT OR Apache-2.0` license and safety/certification caveat remain visible.

## Phase 0.001 audit result

Historically, Microtask 2 reviewed the Cargo manifests and roadmap documents while the workspace version was `0.0.1`. The current workspace version is `0.1.0-alpha.1`, and all member packages inherit it. The earlier audit details remain recorded in `docs/phase_0_001/version_lock_audit.md`.
