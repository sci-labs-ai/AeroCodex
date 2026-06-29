# External M07 solver / least-squares / root-selection policy Wave 1 resolution

A28 assigns terminal metadata dispositions to the first bounded source-ordered slice of the governed solver / least-squares / root-selection policy backlog after the completed A26-A27 coordinate-transform / frame-graph / time-scale policy pool. This is a metadata-only backlog resolution wave. It does not import, execute, scrape, or translate raw Rust-port, M07, or Scilab source.

## Selection

- Wave: A28 external M07 solver / least-squares / root-selection policy Wave 1.
- Selection policy: governed backlog-family continuation after the completed 9B frame/time policy pool.
- Selected rows: `PORT_STATUS_RELEASE_GATE.csv:row_0025` through `PORT_STATUS_RELEASE_GATE.csv:row_0415`, 40 source rows.
- Source groups: 36 Kepler/Lambert/Gauss or numerical-propagation solver-policy rows, 3 solver-rank/tolerance observation-policy rows, and 1 pre-promotion solver-rank/tolerance policy row.

## Disposition

All 40 selected rows remain contract- or policy-blocked. No selected row is promoted to a runtime formula, no formula-vault candidate record is added, and no public API is changed.

| Category | Count |
|---|---:|
| Deduplicated aliases | 0 |
| Internal/helper exclusions | 0 |
| Contract/policy blocks | 40 |
| Solver-policy blocked rows | 40 |
| Iterative-solver rows | 36 |
| Least-squares / solver rows | 4 |

## Accounting

A28 increases cumulative external M07 terminal dispositions from 623 to 663 and reduces the remaining external M07 backlog from 700 to 660. The executable research equation count remains 152 and metadata-only formula-vault candidate records remain 27.

The A28 selected rows are the first 40 source-ordered rows from the 123-row governed solver-policy candidate pool. A28 leaves 83 rows in that candidate pool for later bounded solver-policy waves.

## Verification

The repository verifier is:

```bash
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify formula-vault --self-test
```

Both modes emit JSON with `result=PASS` on success, matching the promoted external runner v2 validation contract.
