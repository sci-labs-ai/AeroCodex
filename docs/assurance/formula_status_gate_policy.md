# Formula Status Gate Policy v0.7.2

Source of truth: AeroCodex Research Readiness Master Plan v0.7.2, task LOCK-004.

This policy locks formula inventory and execution exposure before formula execution is implemented. It is fail-closed by default and preserves the research/preliminary-design posture of AeroCodex.

## Two-level exposure policy

Inventory exposure and execution permission are separate gates:

1. Inventory commands may list, count, search, describe, and report formulas that are not runnable.
2. Normal execution requires implementation_verified or a higher explicitly allowed research/publication-supporting status.

The exact LOCK-004 threshold phrase is: normal execution requires implementation_verified.

| Status | Execution policy | Inventory visibility | Normal execution | Preliminary/dev execution |
|---|---|---|---|---|
| `research_required` | `blocked` | yes | no | no by default |
| `equation_traceable` | `preliminary_flag_required` | yes | no | yes only with explicit `--preliminary` |
| `implementation_verified` | `normal_research` | yes | yes | yes |
| `reference_validated` | `publication_supporting` | yes | yes | yes; may be described only as publication-supporting research support |

## Research-required gate

`research_required` formulas are inventory-visible only. They may appear in `aerocodex formula list`, `aerocodex formula describe`, and `aerocodex formula status-report`, including JSON output. They must not run through normal execution and must not run through public alpha preliminary mode by default.

A normal run attempt for a `research_required` formula must fail with `execution_blocked_by_status` unless a later explicit contract task creates a narrower dev-only path.

## Equation-traceable preliminary gate

`equation_traceable` means a formula has traceability evidence but is below the normal execution threshold. It must not run in normal mode. If a future implementation task supports preliminary/dev execution, that execution must require explicit `--preliminary`, must mark JSON output with `execution_policy: "preliminary_flag_required"`, and must include a warning that preliminary execution is not validation.

A normal run attempt for an `equation_traceable` formula must fail with `preliminary_flag_required`.

## Implementation-verified and reference-validated gates

`implementation_verified` formulas may run in normal research mode when all parser, schema, registry, runtime-symbol, domain-constraint, and test-vector gates pass.

`reference_validated` formulas may run in publication-supporting research mode when the same implementation gates pass and the validation evidence supports that status. This policy does not promote any formula to `reference_validated`.

## M07 quarantine

M07 candidates may be visible in inventory, search, counts, and status reports. M07 candidates are blocked even when they have local metadata or candidate equations. They must not execute through the public alpha CLI until promoted family by family through traceability, schema, implementation, and validation gates.

A run attempt for an M07 candidate must fail with `m07_candidate_blocked`. No LOCK-004 contract may allow M07 candidates to execute through `aerocodex formula run`.

## Legacy CLI alias rule

Deprecated Beta 1 commands and `formula_vault.*` aliases must route through the same registry lookup, alias resolution, and status-gate path as canonical `aerocodex formula ...` commands. They may warn, redirect, or fail with a migration message, but they must not bypass status gates.

## Fail-closed rule

When status, quarantine state, schema validity, alias resolution, runtime symbol resolution, domain constraints, registry freshness, or test-vector state is missing or ambiguous, execution fails closed.

Fail-closed errors should use the canonical public error-code catalog:

- `execution_blocked_by_status` for below-threshold normal execution;
- `preliminary_flag_required` for `equation_traceable` normal-mode attempts;
- `m07_candidate_blocked` for M07 quarantine attempts;
- `registry_stale`, `schema_validation_failed`, `runtime_symbol_missing`, or `test_vector_failed` for corresponding infrastructure gates.

## Non-promotion rule

This policy is a contract. It does not change any validation status, formula manifest, equation batch, M07 quarantine state, runtime implementation, or generated registry artifact.
