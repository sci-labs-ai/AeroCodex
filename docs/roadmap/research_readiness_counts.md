# Research readiness count source of truth

RR-004 establishes this file as the human-readable source of truth for the current research-readiness count language at commit `0c6f231725aa152a486fd5494d12d1f077927d0c`.

Use this file when updating public alpha dashboards, README summaries, release notes, or agent handoffs that mention readiness counts. Do not infer runtime readiness from old roadmap snapshots, historical M07 wave notes, or formula-vault resolution row totals alone.

## Baseline count table

| count_name | current_value | source_or_command | meaning | does_not_mean |
|---|---:|---|---|---|
| governed equation-batch rows | 152 | `CARGO_TARGET_DIR=/tmp/aerocodex_rr004_count_target cargo run -p xtask -- verify --all` reported `verified equation-batch manifests: manifests=8; rows=152; validation_status=research_required`; cross-check: 8 governed `equation-batches/*.tsv` manifests have 152 total data rows. | Executable Rust/runtime equation rows represented by governed equation-batch manifests and tracked with conservative `research_required` status. | Does not mean the rows are certified, operational, flight-ready, mission-ready, or promoted to `implementation_verified`. |
| CLI-accessible legacy M00 canonical formulas | 10 | `CARGO_TARGET_DIR=/tmp/aerocodex_rr004_count_target cargo run -p xtask -- verify --all` reported `verified Beta 1 concept: ... supported_formulas=10; validation_status=research_required`; README Beta 1 concept CLI documents the same ten governed M00 canonical-unit formulas. | The current Beta 1 concept CLI exposes ten legacy M00 canonical formulas for bounded software testing. | Does not mean all 27 M00 formula-vault candidates are CLI-accessible, and does not promote any formula validation status. |
| M00 formula-vault candidates | 27 | `validation/equation_inventory.tsv` sums `metadata_only_formula_vault_candidate=27`; `formula-vault/resolutions/m00_runtime_links.tsv` has 27 data rows with disposition `linked_to_existing_runtime`; xtask verification reported `metadata_only_candidates=27`. | Formula-vault candidate records for M00 metadata/provenance and runtime linkage accounting. | Does not mean 27 new formulas were implemented, newly exposed through the CLI, or promoted beyond `research_required`. |
| visible M07 terminal candidate rows | 1,323 | `validation/equation_inventory.tsv` sums `external_m07_processed_row=1323`; read-only cross-check: 35 `formula-vault/resolutions/m07_*.tsv` files contain 1,323 total data rows; xtask verification reported `external_m07_processed_rows=1323`. | M07 source-derived rows that have visible terminal dispositions in formula-vault resolution manifests and are accounted for by the governed inventory. | The 1,323 M07 rows are not 1,323 usable equations. They are not runtime implementations, CLI-executable formulas, validation promotions, M07/Scilab parity, or public API readiness. |
| M07 execution backlog rows | 0 M07 execution backlog rows | `validation/equation_inventory.tsv` sums `external_m07_backlog_row=0`; xtask verification reported `external_m07_backlog_rows=0`. | No M07 rows remain in the governed external M07 accounting backlog without a terminal disposition row. | Does not mean M07 is executable, unquarantined, validated, source-promoted, or ready for runtime dispatch. |

## Separation of meanings

- **inventory visibility** means a row is visible in a governed inventory, formula-vault manifest, or dashboard. Inventory visibility is accounting and traceability, not runtime authorization.
- **runtime implementation** means Rust code exists in runtime crates and is represented by governed equation-batch rows. Runtime implementation still carries conservative validation status unless separately promoted.
- **CLI accessibility** means a formula is reachable through the current command-line surface. Today that is limited to 10 legacy M00 canonical formulas in the Beta 1 concept CLI.
- **validation status** is independent of inventory and CLI visibility. The RR-004 baseline keeps the relevant counts at `research_required`; RR-004 does not change formula validation status.
- **execution readiness** requires future status-gate, registry, CLI, and promotion work. A row counted here is not ready for normal execution unless a later approved task establishes the required gate state.
- **M07 quarantine** means M07 material remains visible as blocked candidate/source-accounting rows until later family-by-family promotion work explicitly changes a row's status and execution policy.

## Current evidence bundle

The primary RR-004 count-evidence command was:

```bash
CARGO_TARGET_DIR=/tmp/aerocodex_rr004_count_target cargo run -p xtask -- verify --all | tee /tmp/aerocodex_rr004_xtask_verify_counts.txt
```

Relevant output lines:

```text
verified equation inventory: executable_research_equations=152; metadata_only_candidates=27; external_m07_processed_rows=1323; external_m07_backlog_rows=0; validation_cards=46; source_registry_seeds=44; validation_card_only_records=46; helper_algorithms=262
verified equation-batch manifests: manifests=8; rows=152; validation_status=research_required
verified Beta 1 concept: channel=beta1-concept; cargo_version=0.0.1; supported_formulas=10; validation_status=research_required; release_packaging=not_public_repo_tracked
```

Read-only file cross-checks used for this RR-004 baseline:

- `equation-batches/*.tsv`: 8 manifests, 152 total data rows.
- `validation/equation_inventory.tsv`: category row-count sums include `executable_research_equation=152`, `metadata_only_formula_vault_candidate=27`, `external_m07_processed_row=1323`, and `external_m07_backlog_row=0`.
- `formula-vault/resolutions/m00_runtime_links.tsv`: 27 data rows, all `linked_to_existing_runtime`.
- `formula-vault/resolutions/m07_*.tsv`: 35 read-only resolution files, 1,323 total data rows.

## Maintenance note

TODO(RR-026/RR-037 or later registry/status-report task): replace this manually maintained dashboard document with a deterministic status-report/count command that emits the same `count_name`, `current_value`, `source_or_command`, `meaning`, and `does_not_mean` fields from governed repository sources.
