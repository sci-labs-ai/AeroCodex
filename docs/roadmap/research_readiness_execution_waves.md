# Research Readiness Execution Waves v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md` from the v0.7.2 research-readiness planning package.

## Terminology lock

An **Execution Wave** is merge-order guidance for queue-safe planning. It is not a pull-request bundle and not a requirement that all tasks in the wave merge together.

Each LOCK or RR task is one reviewable PR, or one clean commit series, unless a maintainer explicitly batches adjacent non-overlapping docs-only work. Even when a maintainer allows docs-only batching, every task keeps its own acceptance criteria, rollback notes, and handoff evidence.

## Batching rules

Batching is fail-closed. If a task boundary is ambiguous, keep it as one task/one PR or ask the maintainer to partition files before work starts.

Allowed batching is limited to adjacent docs-only tasks that:

- touch different files, or have an explicit maintainer-approved file partition;
- do not change Rust code, generated artifacts, equation-batch TSV files, validation statuses, M07 resolution files, formula runtime behavior, registry generation, or status promotion;
- preserve independently checkable acceptance criteria for each task.

Forbidden batching includes:

- status promotion combined with runtime execution, registry plumbing, manifest plumbing, or generated artifact work;
- two tasks that touch the same Rust module, xtask routing area, registry generator, schema, manifest parser, formula executor, or validation status file unless the dependency has merged first;
- M07 quarantine/promotion work combined with M00 execution work;
- any work that would make a lower-status formula executable outside the explicit status-gate contract.

## Examples

| Situation | Batch? | Reason |
|---|---:|---|
| LOCK-001 plus a maintainer-partitioned docs-only LOCK-002 edit in separate files | Allowed with maintainer approval | Adjacent docs-only work; acceptance criteria can remain independent. |
| RR-024 JSON/error envelopes plus RR-025 status gates | Do not batch by default | Both define execution contracts; reviewers need serialized review. |
| RR-025 status gates plus RR-023 formula execution dispatch | Forbidden | Status gates must merge before normal execution dispatch. |
| RR-015 registry generation plus RR-016 Rust registry module | Forbidden unless explicitly serialized | Generated registry and type-safe module boundaries overlap. |
| RR-030 metadata promotion plus RR-031 normal execution | Forbidden | Status promotion and runtime execution must not be reviewed as one change. |
| Two agents editing the same xtask command routing file | Forbidden until partitioned | Same-module edits are conflict-prone and must be serialized. |

## Required Execution Wave order

The v0.7.2 master plan defines the controlling merge order:

- **Execution Wave 00 - Contract hardening:** LOCK-001 to LOCK-005, RR-001 to RR-004, RR-056.
- **Execution Wave 01 - Equation-batch parser and static reports:** RR-005, RR-055, RR-006, RR-007, RR-008.
- **Execution Wave 02 - Probe, verifier, and CI:** RR-009, RR-010, RR-011, RR-012.
- **Execution Wave 03 - Registry schemas, alias policy, generators, and agent check script:** RR-013, RR-014, RR-018, RR-015, RR-016, RR-017, RR-054.
- **Execution Wave 04 - CLI shell, JSON, gates, and status report:** RR-019, RR-020, RR-021, RR-024, RR-025, RR-026.
- **Execution Wave 05 - Execution after gates:** RR-022, RR-023.
- **Execution Wave 06 - M00 Slice A promotion and first normal execution:** RR-027 to RR-032.
- **Execution Wave 07 - Runtime inventory:** RR-033 to RR-036.
- **Execution Wave 08 - M07 quarantine:** RR-037 to RR-040.
- **Execution Wave 09 - A4 first-family promotion:** RR-041 to RR-043.
- **Execution Wave 10 - Public alpha/release operations:** RR-044 to RR-048.
- **Execution Wave 11 - Rust library API:** RR-049 to RR-053.

## Ordering invariant

JSON/error contracts and fail-closed status gates must merge before input parsing, runtime dispatch, or normal formula execution. Alias policy must merge before registry ID normalization. M07 remains quarantined unless a later explicit task promotes a family through the governed process.
