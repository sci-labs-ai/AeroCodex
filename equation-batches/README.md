# Equation batch compiler

The equation batch compiler turns a reviewed, tab-separated manifest into a temporary Rust probe crate. The Rust compiler verifies every declared runtime symbol, and one authored contract expression is executed for every row.

The tool does not scan or parse Rust source text. Runtime identity comes from the governed equation inventory and is confirmed by compilation.

## Commands

From the repository root:

```text
cargo run -p xtask -- equation-batch plan --manifest equation-batches/m00-canonical-units.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/m00-canonical-units.tsv --output-dir <directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/m00-canonical-units.tsv --output-dir <directory-outside-the-repository>
```

`plan` validates the manifest and prints a stable json plan. `generate` writes a deterministic probe crate and artifact hashes outside the repository. `verify` regenerates the expected artifacts, checks their hashes, and runs the probe crate with Cargo in offline mode.

## Manifest contract

The header is fixed:

```text
schema_version	batch_id	formula_id	package	crate_name	runtime_symbol	output_variable	contract_path	validation_card_path	source_seed_path	validation_status	test_strategy	test_expression
```

Rules:

- Schema version is `aerocodex.equation_batch.v1`.
- A batch contains between 1 and 40 rows.
- Formula identifiers and runtime paths are unique inside a batch.
- The Cargo package and Rust crate names must match a library workspace member.
- Contract, validation-card, source-seed, and inventory links must resolve.
- Validation remains `research_required`.
- The test expression is a single, bounded Rust boolean expression that references the exact runtime path.
- Generated output must be outside the Git repository and may not overwrite an existing directory.
- Missing symbols, invalid expressions, stale governance links, compilation failures, and failed contract probes stop the batch.

The canonical-unit manifest is the first reference batch. It proves the pipeline against ten already governed equations; it does not claim that the full equation inventory is complete or operationally ready.
