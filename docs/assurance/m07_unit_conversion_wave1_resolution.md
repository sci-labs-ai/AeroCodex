# External M07 unit-conversion Wave 1 resolution

A11 processes one bounded low-risk classifier family without importing source code or creating duplicate equation kernels.

## Result

- classifier rows processed: 38;
- duplicate aliases linked to existing governed M00 runtimes: 37;
- contract-blocked rows: 1;
- new Rust equation kernels: 0;
- executable research equations: 152;
- formula-vault metadata candidate records: 27;
- external M07 rows with terminal dispositions: 38;
- remaining external M07 backlog rows: 1,285;
- validation status: `research_required`.

The machine-readable disposition file is `formula-vault/resolutions/m07_unit_conversion_wave1.tsv`. It is an exact projection of the classifier rows whose recommended chunk group is `8D_deduplicated_unit_conversion_helpers`.

## Alias policy

The resolved rows reuse existing M00 contracts and compiler-verified runtimes for:

- degree-to-radian and radian-to-degree conversion;
- canonical distance, time, speed, and gravitational-parameter conversion.

The resolution is a deduplication record, not an external parity claim. Source aliases and file locators are retained as metadata only. Raw M07 or Scilab source is not bundled, parsed, translated, or executed.

## Blocked row

`earth_rotation_rate_canonical` remains blocked because AeroCodex does not yet have an explicit angular-rate canonical-unit contract and matching governed runtime. A11 does not infer one from a function name.

## Verification

```text
cargo run -p xtask -- verify formula-vault
```

The verifier checks the 38-row classifier union, exact target reuse, the 37/1 disposition split, inventory accounting, and the absence of new validation cards, source seeds, kernels, parity claims, certification claims, or operational-readiness claims.

## Later aggregate accounting

A12-A13 add two separate vector-helper disposition manifests covering 74 rows. A11 itself remains a 38-row wave; aggregate terminally processed rows are now 112 and the remaining external backlog is 1,211.
