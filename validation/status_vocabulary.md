# AeroCodex Status Vocabulary

Stage 4 Chunk 2 normalizes the status vocabulary used by validation cards, source-registry seeds, and the data/source governance registry. This file is governance scaffolding only: it does not promote any model, source bundle, equation, archive, validation card, or data-governance artifact to certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use status.

The machine-readable registry is `validation/status_vocabulary.yaml`. The dependency-free verifier is:

```text
cargo run -p xtask -- verify status-vocabulary
```

`cargo run -p xtask -- verify --all` also runs the status-vocabulary check.

## Verification status ladder

These values are used by `validation/cards/*.yaml`, nested `source.status` fields inside those cards, `validation/source_registry/*.yaml`, the Codex Card schema, and the Rust `VerificationStatus` type.

1. `research_required`: source and validation details still need review.
2. `equation_traceable`: exact source edition and equation/table/page identifiers have been reviewed.
3. `implementation_verified`: implementation has been checked against traceable equations and unit tests.
4. `reference_validated`: implementation has been compared against reference values or tables with documented tolerance.
5. `experiment_validated`: implementation has been compared against experimental data with documented applicability.

Status upgrades require evidence in the same review. Registry inclusion, source-material registration, or source-bundle availability is not an upgrade.

## Data-governance validation statuses

These values are used by `data-governance/DATA_REGISTRY.yaml` in `validation_status`. They describe source-material governance state, not mathematical certification:

- `repository_tracked_manifest`
- `repository_tracked_metadata`
- `repository_tracked_citation_support`
- `repository_tracked_research_data`
- `verified_by_xtask_cards`
- `verified_by_xtask_source_registry`
- `governed_generated_inventory`
- `repository_tracked_source_material`
- `repository_tracked_status_vocabulary`
- `repository_tracked_formula_vault_metadata`
- `planning_reference_compared_in_chunk0`
- `release_candidate_not_certified`
- `source_material_registered_not_promoted`
- `planning_material_registered_not_promoted`

## Hash statuses

These values are used by `data-governance/DATA_REGISTRY.yaml` in `hash_status`:

- `sha256_verified_current_repo_file`
- `aggregate_sha256_current_tracked_tree`
- `sha256_verified_from_stage4_local_source`
- `pending_with_reason`

A pending hash must include the reason in the same registry entry and does not permit implementation or public API promotion.

## Forbidden readiness claims

The status vocabulary must not contain allowed values such as `certified`, `flight_ready`, `mission_ready`, `operationally_approved`, `approved_for_operations`, `habitat_safe`, `medical_use`, or `regulated_use_approved`. Those words may appear in caveats, but they are not allowed status values in AeroCodex Stage 4.
