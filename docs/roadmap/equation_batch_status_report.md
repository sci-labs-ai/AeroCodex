# Equation-batch status report

RR-008 adds a deterministic status-report artifact for the current public equation-batch manifests.

Command:

```bash
cargo run -p xtask -- equation-batch report --all-manifests --out generated/equation_batch_status_report.json
```

Check mode:

```bash
cargo run -p xtask -- equation-batch report --all-manifests --out generated/equation_batch_status_report.json --check
```

`--check` recomputes the expected JSON and exits nonzero when the requested output file is missing or stale. The command intentionally requires both `--all-manifests` and `--out`; manifest-specific status reports are not part of RR-008.

## Artifact

Generated path:

- `generated/equation_batch_status_report.json`

Schema version:

- `aerocodex.equation_batch.status_report.v1`

The artifact includes:

- top-level `ok`, `command`, `schema_version`, and `generated_by` fields;
- manifest and row counts;
- validation-status, family, batch, test-strategy, static-symbol, and static-path count maps;
- blocked-execution, metadata-completeness, missing-metadata-path, and static-warning counts;
- per-manifest rows sorted by `formula_id`;
- static path/package/crate/symbol status details;
- `non_claims` and `safety_notice` guardrail language.

## Determinism rules

The report is rendered as stable pretty JSON. It deliberately includes no wall-clock timestamp, username, host-local absolute path, or temporary directory path. Manifest entries are sorted by repository-relative path, row entries are sorted by `formula_id`, count maps are ordered lexically, and missing-path/static-warning lists are sorted consistently.

## Scope and non-claims

This report is inventory and readiness evidence only. It is not execution readiness, does not make formulas executable, and does not promote validation status. It does not compile probes, generate runtime registries, evaluate manifest test expressions, or execute formula code.

AeroCodex remains research and preliminary-design software. The report does not claim that the 152 equation-batch rows are validated, publication-ready, certified, flight-ready, mission-ready, operational, or executable through the normal CLI. AeroCodex is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

M07 rows are not included unless they are actually present in `equation-batches/*.tsv`; M07 quarantine/status work remains separate from RR-008.
