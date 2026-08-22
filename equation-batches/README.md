# Equation batch compiler

The equation batch compiler turns a reviewed, tab-separated manifest into a temporary Rust probe crate. The Rust compiler verifies every declared runtime symbol, and one authored contract expression is executed for every row.

The tool does not scan or parse Rust source text. Runtime identity comes from the governed equation inventory and is confirmed by compilation.

## Commands

From the repository root:

```text
cargo run -p xtask -- equation-batch plan --manifest equation-batches/m00-canonical-units.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/m00-canonical-units.tsv --output-dir <canonical-directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/m00-canonical-units.tsv --output-dir <canonical-directory-outside-the-repository>

cargo run -p xtask -- equation-batch plan --manifest equation-batches/m00-angle-vector.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/m00-angle-vector.tsv --output-dir <angle-vector-directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/m00-angle-vector.tsv --output-dir <angle-vector-directory-outside-the-repository>

cargo run -p xtask -- equation-batch plan --manifest equation-batches/a4-atmosphere-thermo-gasdynamics.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/a4-atmosphere-thermo-gasdynamics.tsv --output-dir <a4-directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/a4-atmosphere-thermo-gasdynamics.tsv --output-dir <a4-directory-outside-the-repository>

cargo run -p xtask -- equation-batch plan --manifest equation-batches/a5-aerodynamics-flight-structures.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/a5-aerodynamics-flight-structures.tsv --output-dir <a5-directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/a5-aerodynamics-flight-structures.tsv --output-dir <a5-directory-outside-the-repository>

cargo run -p xtask -- equation-batch plan --manifest equation-batches/a6-propulsion-heat-transfer.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/a6-propulsion-heat-transfer.tsv --output-dir <a6-directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/a6-propulsion-heat-transfer.tsv --output-dir <a6-directory-outside-the-repository>

cargo run -p xtask -- equation-batch plan --manifest equation-batches/a7-astrodynamics-orekit-foundation.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/a7-astrodynamics-orekit-foundation.tsv --output-dir <a7-directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/a7-astrodynamics-orekit-foundation.tsv --output-dir <a7-directory-outside-the-repository>


cargo run -p xtask -- equation-batch plan --manifest equation-batches/a8-life-support-biosim-foundation.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/a8-life-support-biosim-foundation.tsv --output-dir <a8-directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/a8-life-support-biosim-foundation.tsv --output-dir <a8-directory-outside-the-repository>

cargo run -p xtask -- equation-batch plan --manifest equation-batches/a9-life-support-thinfilm-pde-rom.tsv
cargo run -p xtask -- equation-batch generate --manifest equation-batches/a9-life-support-thinfilm-pde-rom.tsv --output-dir <a9-directory-outside-the-repository>
cargo run -p xtask -- equation-batch verify --manifest equation-batches/a9-life-support-thinfilm-pde-rom.tsv --output-dir <a9-directory-outside-the-repository>
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
- Module-qualified runtime symbols such as `elements::compute_raan` are supported while inventory identity remains tied to the terminal function name inside the selected Cargo package.
- Additional workspace crates referenced by a test expression are added as direct probe dependencies; this supports typed inputs such as `aero_codex_core::Angle`.
- Contract, validation-card, source-seed, and inventory links must resolve.
- Batch exposure is row-specific: the twelve release-manifest formulas are `implementation_verified`, while the other 140 rows remain `research_required`. A linked validation card or source seed may be top-level `research_required` or `equation_traceable`; accepting equation-traceable evidence alone does not promote a batch row.
- The test expression is a single, bounded Rust boolean expression that references the exact runtime path.
- Generated output must be outside the Git repository and may not overwrite an existing directory.
- Missing symbols, invalid expressions, stale governance links, compilation failures, and failed contract probes stop the batch.

## Governed reference batches

- `m00-canonical-units.tsv` covers 10 canonical-unit equations.
- `m00-angle-vector.tsv` covers 17 existing angle, wrapping, and vector-algebra runtime equations.
- `a4-atmosphere-thermo-gasdynamics.tsv` covers 28 existing atmosphere, thermodynamics, and gas-dynamics runtime equations.
- `a5-aerodynamics-flight-structures.tsv` covers 15 existing aerodynamics, flight-dynamics, and structures runtime equations.
- `a6-propulsion-heat-transfer.tsv` covers 9 existing propulsion and heat-transfer runtime equations.
- `a7-astrodynamics-orekit-foundation.tsv` covers 23 existing two-body, transfer, elliptic-element, and non-solver Kepler runtime equations.
- `a8-life-support-biosim-foundation.tsv` covers 40 existing bioregenerative, BLSS-backbone, MELiSSA, nitrifying-biofilm, and attached-algal foundation runtime equations.
- `a9-life-support-thinfilm-pde-rom.tsv` covers the remaining 10 attached-algal PDE residual, reduced-order service, and habitat-coupling runtime equations.

The eight manifests provide compiler-verified coverage for all 152 existing runtime equation paths in the governed executable inventory. Runtime identity is package-scoped, and typed test inputs can reference reviewed workspace dependencies without parsing Rust source. The A8 and A9 batches keep BioSim resource-ledger, transaction, scenario, replay, reporting, and CLI-smoke helpers outside formula scope because they are support algorithms rather than executable research equations. A9 covers bounded helpers for PDE residuals, reduced-order service projections, and habitat coupling; it does not claim to solve or externally validate the underlying PDE, ROM basis, calibration, or habitat controller. Equation-traceable governance evidence is consumed conservatively; only the exact twelve-formula release slice is `implementation_verified`, and the other 140 rows remain `research_required`. These manifests do not change runtime kernels, expose formulas outside that bounded release slice, claim external reference parity, or imply that the external equation backlog is complete or operationally ready.
