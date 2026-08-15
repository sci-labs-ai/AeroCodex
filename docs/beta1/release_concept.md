# AeroCodex Beta 1 concept

<!-- aerocodex-current-identity:start -->
Release version: `0.1.0-alpha.1`
Release tier: `research_software_alpha` (`Research Software Alpha`)
Workspace packages: `14`
Registry formulas: `152`
Blocked formulas: `152`
Publicly executable formulas: `0`
<!-- aerocodex-current-identity:end -->

Status: historical milestone documentation; current formula validation remains `research_required`.

The AeroCodex Beta 1 concept was a **software-quality vertical slice**, not an aerospace operational-readiness claim. Its Cargo version was `0.0.1`; `beta1-concept` is now retained only as a historical milestone label and compatibility alias. The current runtime identity is AeroCodex `0.1.0-alpha.1`, tier `research_software_alpha` (`Research Software Alpha`).

AeroCodex remains research/preliminary-design software. It is not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use.

## Historical pilot scope and current gate

The initial Beta 1 pilot directly exercised exactly ten canonical-unit kernels whose Rust runtime links, contracts, failure policies, numerical policies, family validation card, and family source seed were completed by M00-C2. The current CLI retains those ten dispatch links plus two angle-conversion dispatch links, but every corresponding registry row remains `research_required` and `blocked`; public `formula run` therefore fails closed before dispatch. This paragraph describes Beta 1 historically and the dispatch names as compatibility surfaces, not as the current release identity.

- four positive-scale canonical-unit formulas;
- six signed distance, time, and speed conversions;
- stable formula IDs and runtime-symbol reporting;
- deterministic inventory/status text and JSON output;
- fail-closed input-shape, domain, nonfinite, overflow, and unknown-formula handling;
- a built-in bounded self-check.

No equation implementation is duplicated in the command-line interface. The CLI dispatches to the existing checked Rust kernels.

## Why start with ten formulas

The ten canonical-unit formulas were the historical release-system self-check pilot, not the full equation program or a current public-execution claim. The repository contains a 1,000+ external equation backlog. The historical Beta 1 architecture proved the reusable path needed for later automated ingestion:

1. a stable formula identifier;
2. an exact runtime symbol;
3. a declared input/output schema;
4. a checked Rust implementation;
5. deterministic machine-readable inventory/status output and fail-closed execution gating;
6. validation and source-governance linkage;
7. automated Rust smoke, negative, and repository-gate checks;
8. fail-closed handling for unsupported or ambiguous rows.

Future batches should generate registry entries from governed contract and inventory data. Clean rows should flow through automated checks; ambiguous mappings, missing tests, solver-policy questions, or unsupported domains should remain quarantined for human review.

## Beta 1 acceptance gates

A Beta 1 candidate is acceptable only when all of these are true:

- the full existing workspace CI is green;
- `cargo run -p xtask -- verify --all` and `cargo run -p xtask -- verify beta1` pass;
- the `aerocodex` binary builds on the supported Rust toolchain;
- `aerocodex self-check --json` reports zero failures;
- integration tests verify stable success and error exit codes;
- public formula execution fails closed while registry status remains `research_required`;
- internal self-check directly exercises the ten canonical-unit kernels without changing their public execution policy;
- invalid scales, nonfinite quantities, overflow, and unknown formula IDs fail closed;
- text and JSON outputs retain `research_required` and the safety notice;
- no formula count, validation status, source status, parity status, or certification claim is silently upgraded;
- generated release artifacts are traceable to one Git commit and pass their own smoke check.

## Explicit non-scope

Beta 1 does not claim:

- all 1,000+ equations are implemented;
- public API stability across every AeroCodex crate;
- broad physical reference validation;
- M07, Scilab, Orekit, BioSim, or external-tool parity;
- arbitrary-magnitude floating-point exactness;
- flight, mission, navigation, habitat, medical, regulated-use, or operational readiness;
- certification.

## Next scaling step

The generated Formula Registry now consumes governed contract/inventory rows and emits deterministic dispatcher metadata. Future scaling work must preserve its status gates and promote formulas only through separately authorized, evidence-backed review; this release-integrity batch does not begin that work.
## Public release-candidate gate

The historical Beta 1 release-candidate gate was Rust-only. Former deployment packaging helpers are not tracked here. The current public gate proves the workspace-local dependency policy, runs the governance checks through `xtask`, and verifies the bounded CLI smoke contract.

The public gate does not tag, publish, sign, package, upload, or certify a release. The current Cargo version is `0.1.0-alpha.1`; validation remains `research_required`; `beta1-concept` remains only a historical label and compatibility alias. See [`release_testing.md`](release_testing.md).
