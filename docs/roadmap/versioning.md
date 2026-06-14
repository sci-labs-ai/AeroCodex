# AeroCodex Versioning and Roadmap Phase Lock

Status: locked for Phase 0.001 session review.

This document separates AeroCodex's human roadmap language from Cargo-compatible semantic versioning. The distinction is intentional and must be preserved during Phase 0.001 work.

## Version terms

| Term | Meaning | Current value |
| --- | --- | --- |
| Human roadmap phase | Planning and scope marker used in docs, issues, prompts, validation cards, and release notes. | `Phase 0.001` |
| Cargo package version | SemVer-compatible package version used by all workspace Cargo manifests. | `0.0.1` |
| Public API stability | Compatibility promise for downstream users. | Not promised in Phase 0.001 |

Phase 0.001 is not a Cargo version. Do not write `0.001` in any `Cargo.toml` package version.

## Cargo version lock

During Phase 0.001:

- The root `[workspace.package]` version must remain `0.0.1`.
- Workspace members should inherit that value with `version.workspace = true`.
- A crate may not independently publish a different package version without an explicit later roadmap decision.
- The `xtask` package also inherits the workspace version so repository tooling remains version-aligned.
- Tags, release notes, and generated bundles may mention `Phase 0.001`, but Cargo metadata must remain SemVer-compatible.

The next Cargo version must be chosen deliberately after the project has a tested, reviewed release process. Phase labels such as `0.01`, `0.1`, and `0.5` are roadmap milestones, not automatic Cargo package versions.

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

Microtask 2 reviewed the Cargo manifests and roadmap documents. The active workspace version remains `0.0.1`, and all member packages inherit the workspace version. The audit details are recorded in `docs/phase_0_001/version_lock_audit.md`.
