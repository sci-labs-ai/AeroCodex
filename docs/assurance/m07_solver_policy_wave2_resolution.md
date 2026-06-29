# External M07 solver / numerical propagation policy Wave 2 resolution

A29 assigns terminal metadata dispositions to the second bounded source-ordered slice of the governed solver / least-squares / root-selection policy backlog. This is a metadata-only backlog resolution wave. It does not import, execute, scrape, or translate raw Rust-port, M07, or Scilab source.

## Selection

- Wave: A29 external M07 solver / numerical propagation policy Wave 2.
- Selection policy: governed backlog-family continuation after A28 solver policy Wave 1.
- Selected rows: `PORT_STATUS_RELEASE_GATE.csv:row_0416` through `PORT_STATUS_RELEASE_GATE.csv:row_0884`, 40 source rows.
- Source groups: 26 Kepler, Lambert, Gauss, or numerical-propagation solver-policy rows and 14 generic numerical-method policy rows.

## Disposition

All 40 selected rows remain contract- or policy-blocked. No selected row is promoted to a runtime formula, no formula-vault candidate record is added, and no public api is changed.

| Category | Count |
|---|---:|
| Deduplicated aliases | 0 |
| Internal/helper exclusions | 0 |
| Contract/policy blocks | 40 |
| Solver-policy blocked rows | 40 |
| Iterative-solver rows | 40 |

## Accounting

A29 increases cumulative external M07 terminal dispositions from 663 to 703 and reduces the remaining external M07 backlog from 660 to 620. The executable research equation count remains 152 and metadata-only formula-vault candidate records remain 27.

The A29 selected rows are the first 40 source-ordered rows from the remaining 83-row governed solver-policy candidate pool after A28. A29 leaves 43 rows in that candidate pool for later bounded solver-policy waves.

## Verification

The repository verifier is:

```bash
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify formula-vault --self-test
```

Both modes emit JSON with `result=PASS` on success, matching the promoted external runner v2 validation contract.
