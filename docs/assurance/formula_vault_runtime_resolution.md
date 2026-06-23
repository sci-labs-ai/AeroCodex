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
- A11-A12 assign terminal dispositions to 78 external rows; 1,245 external M07 rows remain unprocessed and incomplete. A12 reuses existing M00 vector runtimes for 30 aliases, excludes 8 internal shape helpers, and keeps 2 rows contract-blocked.
