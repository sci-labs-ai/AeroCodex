# Research readiness count source of truth

RR-004 established this file as the human-readable source of truth for research-readiness count language. The `v0.1.0-alpha.1` release-integrity batch reconciles its current CLI/execution counts against base commit `ffcc2b218220cf705d4673f3b44b45c929f3a65d` and the release manifest in `docs/release/v0.1.0-alpha.1.toml`.

Use this file when updating public alpha dashboards, README summaries, release notes, or agent handoffs that mention readiness counts. Do not infer runtime readiness from old roadmap snapshots, historical M07 wave notes, or formula-vault resolution row totals alone.

## Baseline count table

| count_name | current_value | source_or_command | meaning | does_not_mean |
|---|---:|---|---|---|
| governed equation-batch rows | 152 | `CARGO_TARGET_DIR=/tmp/aerocodex_rr004_count_target cargo run -p xtask -- verify --all` reported `verified equation-batch manifests: manifests=8; rows=152; validation_status=research_required`; cross-check: 8 governed `equation-batches/*.tsv` manifests have 152 total data rows. | Executable Rust/runtime equation rows represented by governed equation-batch manifests and tracked with conservative `research_required` status. | Does not mean the rows are certified, operational, flight-ready, mission-ready, or promoted to `implementation_verified`. |
| CLI dispatch-linked M00 formulas | 12 | `aero-codex-cli` has ten canonical-unit and two angle-conversion `FormulaSpec` records; CLI tests report `supported_formula_count=12`; `docs/release/v0.1.0-alpha.1.toml` records the same twelve canonical registry IDs and symbols. | Twelve M00 registry records have bounded CLI dispatch links available behind the status gate. | A dispatch link is not public execution authorization and does not promote validation status. |
| Publicly executable formulas | 0 | The generated Formula Registry has 152 `research_required` / `blocked` rows; `cargo run -p aero-codex-cli -- formula status-report --json` reports zero normal executable formulas; the release manifest records `public_executable_formula_count=0`. | No formula currently passes the public `formula run` status gate. | Does not remove runtime implementations or internal self-check coverage, and does not authorize formula promotion. |
| M00 formula-vault candidates | 27 | `validation/equation_inventory.tsv` sums `metadata_only_formula_vault_candidate=27`; `formula-vault/resolutions/m00_runtime_links.tsv` has 27 data rows with disposition `linked_to_existing_runtime`; xtask verification reported `metadata_only_candidates=27`. | Formula-vault candidate records for M00 metadata/provenance and runtime linkage accounting. | Does not mean 27 new formulas were implemented, newly exposed through the CLI, or promoted beyond `research_required`. |
| visible M07 terminal candidate rows | 1,323 | `validation/equation_inventory.tsv` sums `external_m07_processed_row=1323`; read-only cross-check: 35 `formula-vault/resolutions/m07_*.tsv` files contain 1,323 total data rows; xtask verification reported `external_m07_processed_rows=1323`. | M07 source-derived rows that have visible terminal dispositions in formula-vault resolution manifests and are accounted for by the governed inventory. | The 1,323 M07 rows are not 1,323 usable equations. They are not runtime implementations, CLI-executable formulas, validation promotions, M07/Scilab parity, or public API readiness. |
| M07 execution backlog rows | 0 M07 execution backlog rows | `validation/equation_inventory.tsv` sums `external_m07_backlog_row=0`; xtask verification reported `external_m07_backlog_rows=0`. | No M07 rows remain in the governed external M07 accounting backlog without a terminal disposition row. | Does not mean M07 is executable, unquarantined, validated, source-promoted, or ready for runtime dispatch. |

## Separation of meanings

- **inventory visibility** means a row is visible in a governed inventory, formula-vault manifest, or dashboard. Inventory visibility is accounting and traceability, not runtime authorization.
- **runtime implementation** means Rust code exists in runtime crates and is represented by governed equation-batch rows. Runtime implementation still carries conservative validation status unless separately promoted.
- **CLI dispatch linkage** means a registry formula resolves to an existing bounded CLI dispatch spec. Today that is 12 M00 records, all still blocked by status.
- **public executability** means a formula passes the public status gate. Today that count is zero.
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
verified Beta 1 concept: channel=beta1-concept; cargo_version=0.0.1; self_check_kernel_count=10; cli_dispatch_formula_count=12; public_executable_formula_count=0; validation_status=research_required; release_packaging=not_public_repo_tracked
```

Read-only file cross-checks used for this RR-004 baseline:

- `equation-batches/*.tsv`: 8 manifests, 152 total data rows.
- `validation/equation_inventory.tsv`: category row-count sums include `executable_research_equation=152`, `metadata_only_formula_vault_candidate=27`, `external_m07_processed_row=1323`, and `external_m07_backlog_row=0`.
- `formula-vault/resolutions/m00_runtime_links.tsv`: 27 data rows, all `linked_to_existing_runtime`.
- `formula-vault/resolutions/m07_*.tsv`: 35 read-only resolution files, 1,323 total data rows.

## Maintenance note

TODO(RR-026/RR-037 or later registry/status-report task): replace this manually maintained dashboard document with a deterministic status-report/count command that emits the same `count_name`, `current_value`, `source_or_command`, `meaning`, and `does_not_mean` fields from governed repository sources.
