# Equation inventory and readiness dashboard

Stage 4 Chunk 7E adds a machine-readable equation inventory at `validation/equation_inventory.tsv` and a dependency-free verifier:

```text
cargo run -p xtask -- verify equation-inventory
```

The verifier is also wired into:

```text
cargo run -p xtask -- verify --all
```

## Scope

This inventory remains governance-only. A10 adds a machine-readable resolution overlay that links the 27 existing formula-vault metadata records to already governed runtimes without adding formula implementations.

It does not implement formulas, import M07 code, generate formula code, create a new runtime, create public application programming interfaces, execute Scilab jobs, bundle M07 archive contents, or promote any flight, mission, operational, certification, or regulated-use claim.

## Inventory classes

The inventory distinguishes:

- `executable_research_equation` — currently public Rust research/preliminary-design equation kernels in repository crates. These are executable but remain blocked from readiness promotion.
- `metadata_only_formula_vault_candidate` — metadata intake/provenance records. A10 resolves all 27 represented formula IDs to existing governed runtimes; the records remain metadata-only artifacts, while the mathematics lives in Rust crates.
- `external_m07_processed_row` — aggregate external rows assigned a terminal alias or blocked disposition by a bounded reviewed wave.
- `external_m07_backlog_row` — aggregate external M07 rows that still lack a terminal disposition.
- `validation_card_only_record` — validation-card metadata rows. These records are not formula implementations.
- `helper_algorithm` — public support/helper routines, validation helpers, type constructors, provenance helpers, BioSim governance primitives, and other support algorithms not counted as executable research equations.

## Historical Stage 4 Chunk 7E counts

The verifier is expected to report:

- executable research equations: 112
- metadata-only formula-vault candidates: 3
- external M07 backlog rows: 1347
- validation cards: 36
- source-registry seeds: 34
- validation-card-only records: 36
- helper algorithms: 89

The remaining external M07 backlog is derived from the registered 1,350 represented rows minus 27 formula-vault candidate records and minus rows with terminal external-resolution dispositions. A11-A14 assign 152 terminal dispositions, leaving 1,171 rows unprocessed.


## Current post-A14 resolution state

The current governed inventory contains 152 executable research equations, 27 metadata-only intake records, 152 external M07 rows with terminal dispositions, 1,171 remaining external M07 backlog rows, 46 validation-card-only records, and 262 helper algorithms. The A10 runtime-resolution counters remain:

- runtime-linked formula-vault records: 27;
- unresolved formula-vault candidate formula IDs: 0;
- angle/unit links: 3;
- vector-algebra links: 14;
- canonical-unit links: 10.

`formula-vault/resolutions/m00_runtime_links.tsv` must exactly match the formula IDs in the candidate records and the runtime, contract, validation-card, and source-seed fields in the two governed M00 equation-batch manifests. A11 processes 38 unit-conversion rows. A12-A13 process all 74 vector-helper rows: 56 aliases, 13 excluded internal utilities, and 5 contract blocks. A14 processes the first 40 rows of the 49-row classical two-body algebra group: 16 aliases and 24 contract blocks, while preserving the classifier risk tier `medium_risk_requires_contract_review`. No executable or metadata-candidate count changes.

## Readiness rule

Every inventory row is explicitly blocked. The block reason is row-local and machine-readable. No row may claim certification, flight readiness, mission readiness, operational approval, or regulated-use approval. The dashboard answers readiness by showing which class an item belongs to and why it is blocked before any future implementation chunk can be considered.
