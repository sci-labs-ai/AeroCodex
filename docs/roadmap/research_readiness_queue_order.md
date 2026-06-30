# Research Readiness Queue Order v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md` from the v0.7.2 research-readiness planning package.

## Queue semantics

This queue is organized by **Execution Wave**. Execution Waves define recommended merge order; they do not define PR groups. Each LOCK or RR task is one reviewable PR or one clean commit series unless a maintainer explicitly batches adjacent non-overlapping docs-only tasks.

Agents must not treat a wave as permission to implement the whole wave. Start only the assigned task ID, preserve its file-scope warnings, and stop at the task's acceptance criteria. Use `docs/roadmap/research_readiness_dependency_conflict_matrix.md` as the dependency/conflict matrix before starting parallel implementation work.

## Merge-order queue

| Execution Wave | Scope | Task order |
|---|---|---|
| 00 | Contract hardening | LOCK-001, LOCK-002, LOCK-003, LOCK-004, LOCK-005, RR-001, RR-002, RR-003, RR-004, RR-056 |
| 01 | Equation-batch parser and static reports | RR-005, RR-055, RR-006, RR-007, RR-008 |
| 02 | Probe, verifier, and CI | RR-009, RR-010, RR-011, RR-012 |
| 03 | Registry schemas, alias policy, generators, and agent check script | RR-013, RR-014, RR-018, RR-015, RR-016, RR-017, RR-054 |
| 04 | CLI shell, JSON, gates, and status report | RR-019, RR-020, RR-021, RR-024, RR-025, RR-026 |
| 05 | Execution after gates | RR-022, RR-023 |
| 06 | M00 Slice A promotion and first normal execution | RR-027, RR-028, RR-029, RR-030, RR-031, RR-032 |
| 07 | Runtime inventory | RR-033, RR-034, RR-035, RR-036 |
| 08 | M07 quarantine | RR-037, RR-038, RR-039, RR-040 |
| 09 | A4 first-family promotion | RR-041, RR-042, RR-043 |
| 10 | Public alpha/release operations | RR-044, RR-045, RR-046, RR-047, RR-048 |
| 11 | Rust library API | RR-049, RR-050, RR-051, RR-052, RR-053 |

## Serialized contract checkpoints

- LOCK-001 must establish task-as-PR and Execution Wave terminology before agents treat wave labels as scheduling guidance.
- LOCK-002 adds `docs/roadmap/research_readiness_dependency_conflict_matrix.md`; use that dependency/conflict matrix to decide whether parallel work is allowed, blocked, or requires maintainer file partitioning.
- RR-024 and RR-025 must merge before RR-022, RR-023, and RR-031.
- RR-018 must merge before RR-015 so alias policy exists before ID normalization.
- RR-055 must define fixtures and golden output paths before parser, verifier, or CLI output work relies on them.
- RR-037 through RR-040 keep M07 visible but blocked unless a later explicit family-promotion task changes status.

## Handoff evidence rule

Every task handoff should report the task ID, changed files, commands run, grep/test results, files intentionally not touched, and whether validation statuses, M07 files, equation manifests, or runtime code changed.
