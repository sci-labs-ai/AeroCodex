# Formula-vault candidate verifier

Stage 4 Chunk 7C adds a dependency-free `xtask` verifier for formula-vault candidate metadata records. It is a governance and metadata check only. It does not implement formulas, translate source expressions, execute Scilab, import fixtures, add generated Rust, create a public application programming interface, or upgrade any validation status.

AeroCodex remains research and preliminary-design software. This verifier is not certification evidence, flight-readiness evidence, mission-readiness evidence, operational approval, medical-use approval, or regulated-use approval.

## Scope

The new command is:

```text
cargo run -p xtask -- verify formula-vault
```

The command is also included in:

```text
cargo run -p xtask -- verify --all
```

The verifier currently scans only `formula-vault/candidates/*.yaml`. It does not parse or import the registered external M07 archive, and it does not inspect source code inside quarantined source bundles.

## Checked metadata

For each candidate record, the verifier requires:

- `schema_version: formula_vault_candidate_slice.v1`;
- a `record_status` that remains `research_required` for this metadata-only stage;
- required top-level sections: `slice`, `sources`, `formula_contract`, `validation_records`, `evidence_plan`, `promotion_gate`, and `non_claims`;
- a `formula_vault.*` slice identifier;
- a blocked public-surface state;
- at least one source artifact identifier;
- matching source-registry seed and validation-card identifiers that exist in `validation/source_registry/` and `validation/cards/`;
- nonempty formula-contract lists for formula identifiers, variables, units, coordinate-frame assumptions, time-scale assumptions, sign conventions, valid domain, singularities, invalid regions, and branch behavior;
- unique `formula_vault.*` formula identifiers within and across candidate records;
- matching `validation_records.required_source_registry_seed` and `validation_records.required_validation_card` values;
- `validation_records.status` within the canonical status ladder;
- `promotion_gate.default_state: blocked`;
- explicit non-claim booleans for no certification evidence, no flight readiness, no mission readiness, no operational approval, no regulated-use approval, no bulk M07 import, and no external parity claim without evidence;
- no local absolute paths, evidence-log paths, or target-output paths in candidate metadata.

## Negative cases

The unit-test fixtures cover:

- a minimal valid candidate record;
- missing required candidate field rejection;
- duplicate formula identifier rejection;
- duplicate slice identifier rejection.

These tests are parser-scaffold tests, not numerical validation and not source equivalence.

## Boundary

Chunk 7C does not add, change, or authorize any formula implementation. It only makes the existing Chunk 7B candidate record more checkable. Any later formula implementation must still be explicitly authorized and must satisfy the per-slice gate in `docs/assurance/formula_vault_candidate_gate.md`.
