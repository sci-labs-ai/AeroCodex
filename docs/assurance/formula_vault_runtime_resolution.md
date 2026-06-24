# Formula-vault runtime resolution

A10 closes the ambiguity in the 27 existing formula-vault intake records by linking each formula ID to one already governed Rust runtime. It does not add, copy, or regenerate equation kernels.

## Machine-readable record

`formula-vault/resolutions/m00_runtime_links.tsv` contains one row per formula ID with:

- candidate family and candidate metadata path;
- governed equation-batch manifest and batch ID;
- package, crate name, runtime symbol, and runtime path;
- normalized output variable;
- contract, validation-card, and source-seed paths;
- `research_required` status;
- terminal disposition `linked_to_existing_runtime`.

The 27 rows comprise 3 angle/unit, 14 vector-algebra, and 10 canonical-unit formulas. All are already represented in `equation-batches/m00-angle-vector.tsv` or `equation-batches/m00-canonical-units.tsv`.

## Verification

```text
python3 scripts/verify_formula_vault_resolutions.py --repo .
python3 scripts/verify_governance.py --repo .
```

The verifier fails closed on duplicate or missing formula IDs, unresolved candidates, mismatched runtime or governance fields, missing repository paths, changed inventory dispositions, or a changed external backlog aggregate. It does not scrape Rust source text.

## Boundaries

- Candidate YAML files remain metadata/provenance records, not implementation source.
- Existing Rust equation kernels remain the sole executable implementation.
- Validation remains `research_required`.
- No M07 or Scilab source parity, certification, flight readiness, mission readiness, operational approval, or regulated-use approval is claimed.
- A11-A16 assign terminal dispositions to 201 external rows; 1,122 external M07 rows remain unprocessed and incomplete. A12-A13 complete the 74-row vector-helper group with 56 aliases, 13 internal shape-helper exclusions, and 5 contract-blocked rows. A14-A15 complete the 49-row classical two-body algebra group with 22 aliases and 27 contract blocks. A16 processes the first 40 orbital-geometry/conic rows with 2 aliases, 10 helper exclusions, and 28 contract blocks. These waves do not downgrade the classifier risk tier.
