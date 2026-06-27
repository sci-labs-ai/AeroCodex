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

The remaining external M07 backlog is derived from the registered 1,350 represented rows minus 27 formula-vault candidate records and minus rows with terminal external-resolution dispositions. A11-A33 assign 855 terminal dispositions, leaving 468 rows unprocessed.


## Current post-A27 resolution state

The current governed inventory contains 152 executable research equations, 27 metadata-only intake records, 855 external M07 rows with terminal dispositions, 468 remaining external M07 backlog rows, 46 validation-card-only records, and 262 helper algorithms. The A10 runtime-resolution counters remain:

- runtime-linked formula-vault records: 27;
- unresolved formula-vault candidate formula IDs: 0;
- angle/unit links: 3;
- vector-algebra links: 14;
- canonical-unit links: 10.

`formula-vault/resolutions/m00_runtime_links.tsv` must exactly match the formula IDs in the candidate records and the runtime, contract, validation-card, and source-seed fields in the two governed M00 equation-batch manifests. A11 processes 38 unit-conversion rows. A12-A13 process all 74 vector-helper rows: 56 aliases, 13 excluded internal utilities, and 5 contract blocks. A14-A15 process all 49 classical two-body algebra rows: 22 aliases and 27 contract blocks. A16-A25 complete all 377 rows of `9A_classical_elements_and_9E_mission_design_contracts`: 12 aliases, 90 internal/composite-helper exclusions, and 275 contract or policy blocks, leaving 0 group rows. A26-A27 complete the governed 9B coordinate-transform/frame-graph/time-scale policy backlog with 85 contract or policy blocks, leaving 0 rows in that candidate pool. A26-A27 preserve 58 medium-risk and 27 frame/time-policy blocked classifier labels without downgrade. No executable or metadata-candidate count changes.

## Readiness rule

Every inventory row is explicitly blocked. The block reason is row-local and machine-readable. No row may claim certification, flight readiness, mission readiness, operational approval, or regulated-use approval. The dashboard answers readiness by showing which class an item belongs to and why it is blocked before any future implementation chunk can be considered.

### A34 attitude frame policy Wave 1

External M07 processed rows increase to 895; remaining backlog is 428. A34 adds only research-required metadata dispositions.


### A35 attitude frame policy Wave 2

External M07 processed rows increase to 914; remaining backlog is 409. A35 adds only research-required metadata dispositions and closes the attitude representation candidate pool.

### A36 attitude dynamics/control policy Wave 1

External M07 processed rows increase to 952; remaining backlog is 371. A36 adds only research-required metadata dispositions for the attitude dynamics/control policy candidate pool.

### A37 external M07 J2 perturbation / numerical propagation policy Wave 1

A37 adds 40 research-required terminal dispositions for `10B_J2_perturbation_and_numerical_policy`, bringing external M07 processed/backlog counters to 992/331.

### A38 external M07 J2 perturbation / numerical propagation policy Wave 2

A38 adds 40 research-required terminal dispositions for `10B_J2_perturbation_and_numerical_policy`, bringing external M07 processed/backlog counters to 1032/291.

### A39 external M07 J2 perturbation / numerical propagation policy Wave 3

A39 adds 48 research-required terminal dispositions for `10B_J2_perturbation_and_numerical_policy`, closing that governed candidate pool and bringing external M07 processed/backlog counters to 1080/243.


### A40 external M07 SGP4 / TEME frame-time policy Wave 1

A40 adds 45 research-required terminal dispositions for `10C_sgp4_teme_oracle_and_frame_time_gate` and `10C_sgp4_hold_no_public_helper_import`, bringing external M07 processed/backlog counters to 1125/198.
