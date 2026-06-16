# Formula-vault M00 per-candidate manifest/reference-link depth

Stage 4 Chunk 7D adds a metadata-only per-candidate manifest for the existing M00 angle/unit conversion candidate. The new manifest is:

```text
formula-vault/manifests/m00_angle_unit_conversions_manifest.yaml
```

This manifest deepens traceability for the Chunk 7B candidate record without changing its validation status. It does not implement formulas, translate source expressions, execute Scilab, import fixtures, add generated Rust, create a public application programming interface, or upgrade any validation status.

AeroCodex remains research and preliminary-design software. This manifest is not certification evidence, flight-readiness evidence, mission-readiness evidence, operational approval, medical-use approval, or regulated-use approval.

## Scope

The manifest records, for each selected formula identifier:

- candidate function alias;
- source function alias;
- release-gate row alias;
- equivalence-job alias;
- source-file locator inside the quarantined external artifact;
- pending source-expression review status;
- variable/unit/domain review status;
- tolerance-review status;
- blocked implementation status.

The record is linked back to:

- `formula-vault/candidates/m00_angle_unit_conversions.yaml`;
- `docs/assurance/formula_vault_m00_angle_unit_candidate.md`;
- validation card `validation.formula_vault.m00_angle_unit_conversions`;
- source seed `source.formula_vault.m00_angle_unit_conversions.research_required`;
- `docs/assurance/formula_vault_candidate_gate.md`;
- `docs/assurance/formula_vault_candidate_verifier.md`;
- `docs/source_intake/m07_formula_vault_intake.md`.

## Boundary

The M07 source artifact remains registered externally as `stage4.m07_rust_port_v14.2026_06_15`. Chunk 7D does not copy archive contents, source expressions, executable fixtures, generated outputs, local absolute paths, or evidence logs into the repository.

The manifest keeps all selected formulas blocked until a future explicitly authorized chunk supplies source-review evidence, implementation scope, tests, equivalence/reference evidence, and the required conservative caveats.

## Verification expectation

The existing command remains the candidate metadata gate:

```text
cargo run -p xtask -- verify formula-vault
```

Chunk 7D keeps the manifest as governance metadata and refreshes the formula-vault/data-registry aggregate fingerprints after adding it.


## Chunk 7F contract cross-link

The per-candidate manifest now links forward to `formula-vault/contracts/m00_angle_unit_conversions_contract.yaml`, which records the metadata-only source-expression and test-vector contract for the same three formula identifiers. The manifest and contract remain non-implementation records with `research_required` status.
