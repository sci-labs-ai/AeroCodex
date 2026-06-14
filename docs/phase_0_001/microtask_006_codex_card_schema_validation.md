# Microtask 6 — Codex Card Schema and Validation Scaffold

Status: complete in this interactive session.

## Scope

Microtask 6 reviews and tightens the Phase 0.001 validation-card scaffold. It does not validate any aerospace equation against a source, reference table, test dataset, experiment, flight result, or certification artifact.

## What changed

- Tightened `validation/schema/codex_card.schema.json` with a Draft 2020-12 schema, explicit `additionalProperties: false`, required fields, nonempty list sections, status enums, category enums, dotted-ID patterns, and a structured `source` object.
- Added `validation/README.md` to explain the validation directory layout, status ladder, local scaffold commands, and card-authoring rules.
- Expanded `xtask` so the deployment agent can run:

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- dependency-policy
```

- Strengthened dependency-free `xtask` checks for:
  - Codex Card schema markers;
  - required top-level card fields;
  - nonempty list sections;
  - known status strings;
  - known card and source categories;
  - card source IDs matching source-registry IDs;
  - required source-registry fields;
  - duplicate source-registry IDs;
  - selected forbidden readiness markers;
  - native/wrapper dependency-policy tokens.
- Added unit-test scaffolding inside `xtask/src/main.rs` for core text-scanning helpers. These tests still require a Rust toolchain to execute.
- Updated the README, docs index, API summary, deployment prompt, working inventory, source-research backlog, file manifest, and file inventory.

## Current validation-card status

All current validation cards remain at:

```text
status: research_required
```

All current source-registry entries remain at:

```text
status: research_required
```

No card or source-registry entry was upgraded to `equation_traceable`, `implementation_verified`, `reference_validated`, or `experiment_validated` during Microtask 6.

## Definition of done result

The Codex Card scaffold is stricter, source-registry linkage is checked by `xtask`, and the documentation now explains how deployment agents should run and interpret the validation scaffold. The scaffold remains dependency-free and conservative.

## Checks performed locally

The local environment does not provide `cargo`, `rustc`, `rustfmt`, or `clippy-driver`, so Rust execution checks were not run here. Instead, the following static checks were completed:

- Parsed every `Cargo.toml` with Python `tomllib`.
- Confirmed `xtask` still has no dependencies.
- Confirmed the Codex Card schema is valid JSON with Python `json` tooling.
- Confirmed required schema markers, status strings, and category strings are present.
- Confirmed every validation card has the required top-level fields.
- Confirmed every validation card has nonempty `assumptions`, `inputs`, `outputs`, `tests`, and `failure_modes` lists.
- Confirmed every validation card source ID matches a source-registry ID.
- Confirmed every source-registry file has required fields and nonempty list sections.
- Confirmed every validation card and source-registry entry remains `research_required`.
- Re-ran the static forbidden native dependency token scan across Cargo manifests.
- Performed rough brace/parenthesis balance checks on changed Rust source.

## Required deployment-agent checks

A Rust-enabled deployment environment must still run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```

## Source-verification gaps

- Exact source editions, equation numbers, table numbers, page ranges, data provenance, uncertainty metadata, and validation tolerances still require later source review.
- The schema and `xtask` checks are validation-governance scaffolding only.
- No certification, flight-readiness, mission-readiness, habitat-readiness, operational-approval, or safety-critical suitability claim is made.
