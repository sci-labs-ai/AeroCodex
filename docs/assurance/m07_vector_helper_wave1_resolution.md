# External M07 vector-helper Wave 1 resolution

A12 assigns terminal metadata dispositions to the first 40 rows, in source-row order, from the classifier group `8D_helper_deduplication_then_low_risk_vector_contracts`.

## Results

- 30 rows are deduplicated aliases to existing governed M00 vector runtimes: dot, cross, norm, unit-vector, and vector-angle;
- 8 column-shape or identity-constructor rows are excluded as internal utilities rather than formula nodes;
- `ch2_det3_cols` remains blocked until determinant column order and scalar-triple equivalence are explicitly contracted;
- `ch6::latlon_to_unit` remains blocked until latitude/longitude units, axis order, handedness, and frame semantics are explicitly contracted;
- no Rust kernel, public application programming interface, validation card, source seed, or formula node is added;
- validation remains `research_required`.

The machine-readable dispositions are in `formula-vault/resolutions/m07_vector_helper_wave1.tsv`. Verify them with:

```text
cargo run -p xtask -- verify formula-vault
```

A12 increases terminally processed external rows from 38 to 78 and reduces the unprocessed external backlog from 1,285 to 1,245. It does not inspect or import raw M07 or Scilab source text and makes no parity, certification, or operational-readiness claim.

## Later aggregate accounting

A13 adds a separate 34-row Wave 2 manifest for the remainder of the same classifier group. A12 itself remains a 40-row wave; A12-A13 together cover all 74 vector-helper rows. Aggregate terminally processed external rows are now 112 and the remaining backlog is 1,211.
