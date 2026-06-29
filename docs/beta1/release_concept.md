# AeroCodex Beta 1 concept

Status: `research_required`

The AeroCodex Beta 1 concept is a **software-quality vertical slice**, not an aerospace operational-readiness claim. Cargo version remains `0.0.1`; `beta1-concept` is a release-channel label used to test packaging, user interaction, diagnostics, deterministic output, and release gates before any later version decision.

AeroCodex remains research/preliminary-design software. It is not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use.

## Initial supported scope

The first Beta 1 executable surface exposes exactly the ten canonical-unit formulas whose Rust runtime links, contracts, failure policies, numerical policies, family validation card, and family source seed were completed by M00-C2:

- four positive-scale canonical-unit formulas;
- six signed distance, time, and speed conversions;
- stable formula IDs and runtime-symbol reporting;
- deterministic text and JSON output;
- fail-closed input-shape, domain, nonfinite, overflow, and unknown-formula handling;
- a built-in bounded self-check.

No equation implementation is duplicated in the command-line interface. The CLI dispatches to the existing checked Rust kernels.

## Why start with ten formulas

The ten canonical-unit formulas are the release-system pilot, not the full equation program. The repository contains a 1,000+ external equation backlog. The Beta 1 architecture proves the reusable path needed for later automated ingestion:

1. a stable formula identifier;
2. an exact runtime symbol;
3. a declared input/output schema;
4. a checked Rust implementation;
5. deterministic machine-readable execution;
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
- an exact signed conversion runs through the CLI and existing Rust kernel;
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

After the ten-formula CLI and release gate are deployed, the next production-engineering task is a generated formula registry. It should consume governed contract/inventory rows, emit deterministic dispatcher metadata and tests, and produce an exception report for blocked rows. That is the mechanism intended to scale from this pilot to the larger equation inventory without repeating manual four-to-ten-formula review ceremonies.
## Public release-candidate gate

The public repository now keeps the Beta 1 release-candidate gate Rust-only. Former deployment packaging helpers are not tracked here. The public gate proves the workspace-local dependency policy, runs the governance checks through `xtask`, and verifies the bounded CLI smoke contract.

The public gate does not tag, publish, sign, package, upload, or certify a release. Cargo version remains `0.0.1`, validation remains `research_required`, and the release channel remains `beta1-concept`. See [`release_testing.md`](release_testing.md).
