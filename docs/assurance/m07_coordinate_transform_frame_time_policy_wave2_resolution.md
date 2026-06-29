# External M07 coordinate-transform / frame-graph / time-scale policy Wave 2 resolution

A27 assigns terminal metadata dispositions to the remaining governed coordinate-transform / frame-graph / time-scale policy rows after the A26 Wave 1 slice. This is a metadata-only backlog resolution wave. It does not import, execute, scrape, or translate raw Rust-port, M07, or Scilab source.

## Selection

- Wave: A27 external M07 coordinate-transform / frame-graph / time-scale policy Wave 2.
- Selection policy: governed backlog-family continuation after A26 Wave 1.
- Selected rows: `PORT_STATUS_RELEASE_GATE.csv:row_0405` through `PORT_STATUS_RELEASE_GATE.csv:row_1259`, 45 source rows.
- Source groups: 29 coordinate-transform contract rows, 13 frame-graph/time-policy rows, and 3 time-scale/sidereal rows.

## Disposition

All 45 selected rows remain contract- or policy-blocked. No selected row is promoted to a runtime formula, no formula-vault candidate record is added, and no public API is changed.

| Category | Count |
|---|---:|
| Deduplicated aliases | 0 |
| Internal/helper exclusions | 0 |
| Contract/policy blocks | 45 |
| Medium-risk contract-review rows | 29 |
| Frame/time-policy blocked rows | 16 |

## Accounting

A27 increases cumulative external M07 terminal dispositions from 578 to 623 and reduces the remaining external M07 backlog from 745 to 700. The executable research equation count remains 152 and metadata-only formula-vault candidate records remain 27.

## Verification

The repository verifier is:

```bash
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify formula-vault --self-test
```

Both modes emit JSON with `result=PASS` on success, matching the promoted external runner v2 validation contract.
