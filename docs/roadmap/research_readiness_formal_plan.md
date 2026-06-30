# Research Readiness Formal Plan v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md` from the v0.7.2 research-readiness planning package.

AeroCodex is professional research/preliminary-design software. It is not flight-certified, mission-certified, habitat-safety-certified, life-support-certified, NASA-approved, or regulatory-approved software.

## Execution boundary

The v0.7.2 plan uses **Execution Wave** terminology for recommended merge order only. An Execution Wave is not a PR group. Each LOCK or RR task is one reviewable PR or one clean commit series unless a maintainer explicitly batches adjacent non-overlapping docs-only work.

This boundary keeps large formula-infrastructure work reviewable by separating planning contracts, parser/verifier work, registry generation, CLI/status gates, formula execution, promotion packets, M07 quarantine, and release documentation.

## Task execution rule

Agents should implement one assigned task at a time. A task handoff should name the task ID, files changed, commands run, evidence gathered, and any stop conditions. Do not proceed from one task to the next just because both tasks appear in the same Execution Wave.

Docs-only batching is allowed only when a maintainer approves the batch and the files are non-overlapping or explicitly partitioned. Batching never changes the acceptance criteria for each task.

## Fail-closed batching limits

Do not batch:

- status promotion with runtime execution;
- status promotion with registry or manifest plumbing;
- parser/model changes with verifier, generator, or CLI dispatch changes unless the dependency has merged first;
- two tasks that touch the same module unless a maintainer serializes them;
- M07 quarantine/promotion work with M00 execution work;
- public-release wording changes with runtime behavior changes.

## Contract sequence summary

The v0.7.2 lockdown sequence starts with Execution Wave 00 contract hardening before normal formula execution work. JSON/error envelopes and status gates must merge before input parsing and execution dispatch. M00 Slice A remains the first normal execution scope. M07 remains quarantined as visible blocked candidates until a later family-by-family promotion task explicitly changes that status.

## Dependency/conflict matrix

`docs/roadmap/research_readiness_dependency_conflict_matrix.md` records the queue-safe dependency/conflict matrix for serialized implementation zones. Agents must check it before starting parser, verifier, registry, CLI, status-gate, generated-artifact, Cargo workspace, validation-status, runtime execution, or M07 quarantine work. A conflicting task must not start until its dependency has merged or a maintainer explicitly partitions the files.

## Related roadmap docs

- `docs/roadmap/research_readiness_execution_waves.md` defines Execution Wave terminology, ordering, and batching examples.
- `docs/roadmap/research_readiness_queue_order.md` lists the v0.7.2 queue in merge-order form while preserving the one-task-one-PR rule.
- `docs/roadmap/research_readiness_dependency_conflict_matrix.md` defines serialized zones, critical ordering constraints, and safe-parallel-work boundaries.
