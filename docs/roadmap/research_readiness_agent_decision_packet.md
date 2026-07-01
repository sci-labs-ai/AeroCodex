# AeroCodex Research Readiness Agent Decision Packet

This research readiness decision packet records the RR-001 planning authority for future AeroCodex research-readiness work. The controlling source is `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md` from the v0.7.2 planning package; older v0.3, v0.6, v0.7.0, and v0.7.1 materials are reference material only when v0.7.2 explicitly incorporates them.

## Governing instruction

Treat AeroCodex as professional research software aimed at eventual public alpha and publication-supporting use, not certified operational aerospace software. The immediate goal is not to make all 1,000+ equations runnable. The immediate goal is to build the queue-safe formula infrastructure: parser/verifier, generated registry, formula CLI, status gates, M00 execution, and honest inventory of existing runtime and M07 candidate formulas. Default execution must require implementation_verified; lower-status formulas may be inventoried or run only in explicit preliminary/dev modes. M07 remains quarantined as visible blocked candidates until promoted family by family. Use 2–4 hour PR-sized task cards with commands, tests, acceptance criteria, rollback, and file-overlap warnings.

## Governing posture

AeroCodex is intended to become professional-grade, traceable aerospace research software suitable for academic, laboratory, and agency evaluation. It is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

## Target release state

The target research-readiness release is a public-alpha-oriented, evidence-rich research software baseline. It should make formula provenance, status, validation evidence, and execution gates visible without implying operational approval. The release should be credible for academic, laboratory, and agency evaluation of research/preliminary-design mathematics, not for safety-critical decisions.

## First real user

The first real user is a technical researcher, maintainer, or reviewer who needs to inspect formula provenance, run a small verified slice, and understand why lower-status or quarantined formulas are visible but blocked. The user should be able to reproduce queue decisions from docs, task cards, commands, and artifacts without relying on implicit agent memory.

## Primary usage order

1. Inventory: list formula families, statuses, provenance, and blocked candidate groups honestly.
2. Describe: expose source, schema, sidecar, aliases, inputs, outputs, status, and warnings for a formula.
3. Verify: run parser, registry, schema, status-gate, and fixture checks before enabling execution.
4. Execute: allow normal execution only for formulas meeting the status gate.
5. Promote: use reviewed evidence packets to change status, then regenerate deterministic registries.
6. Publish/support: use conservative research/preliminary-design wording and cite the exact commit, source, status, and evidence.

## Equation family priority

M00 Slice A is the first normal execution slice. Parser/verifier, generated registry, formula CLI, and status gates come before broader execution work. Existing runtime formulas should be inventoried honestly before attempting broad promotion. M07 candidate formulas remain blocked inventory until a later family-by-family promotion task reviews evidence and changes status. A4 and later family work follows only after the infrastructure and M00 slice gates are stable.

## Minimum status for inventory exposure and executable exposure

| Status or group | Inventory exposure | Normal executable exposure | Preliminary/dev executable exposure |
|---|---|---|---|
| `research_required` | Allowed with conservative caveats | Not allowed | Not allowed by default |
| `equation_traceable` | Allowed | Not allowed | Allowed only with an explicit preliminary/dev mode |
| `implementation_verified` | Allowed | Allowed | Allowed |
| `reference_validated` | Allowed | Allowed | Allowed with publication-supporting caveats |
| M07 blocked candidates | Allowed as visible blocked candidates | Not allowed | Not allowed until promoted family by family |

Default execution must require `implementation_verified` or a stronger reviewed status. Lower-status formulas may be inventoried; they may run only in explicit preliminary/dev modes where the controlling contract allows it.

## First milestone

The first milestone is not broad equation execution. It is queue-safe formula infrastructure: parser/verifier, deterministic generated registry, formula CLI, JSON and error contracts, status gates, M00 Slice A execution, and honest inventory of existing runtime and M07 candidate formulas.

## Repo and tooling assumptions

AeroCodex remains a pure Rust workspace with xtask governance checks, deterministic generated artifacts, schema-controlled registry files, and no committed root `Cargo.lock` in this phase. Local verification may transiently generate an untracked root `Cargo.lock`; if it was absent before verification and appears only as an untracked root lockfile, remove it and report the hash/size. Do not add native runtime dependencies, generated binaries, alternate registry formats, or unreviewed workspace changes during research-readiness queue work.

## Work queue style

Each LOCK or RR task is one reviewable PR or one clean commit series unless a maintainer explicitly batches adjacent non-overlapping docs-only work. Task cards should fit a 2–4 hour implementation window and include commands, tests, acceptance criteria, rollback, stop conditions, and file-overlap warnings. Do not start the next task just because it shares an execution wave.

## CLI/API style

The CLI should preserve human-readable output by default and expose stable JSON with explicit success/error envelopes. Formula commands should follow the `aerocodex formula list`, `describe`, `run`, and status-report direction established by v0.7.2. Legacy aliases must route through the same registry and status gates or fail with clear migration messages; they must not bypass policy.

## Registry format

The registry authority is a deterministic generated registry with schema validation and hash sidecars. Canonical registry records should include stable formula IDs, aliases, family, status, source/provenance references, input/output contracts, non-claims, generator/source hashes, and status-gate policy fields. Checked-in generated registry artifacts must be reproducible, sorted deterministically, and verified by generate/check commands before they are trusted.

## M07 quarantine rule

M07 remains quarantined as visible blocked candidates until promoted family by family. M07 records may be exposed in inventory, search, counts, and status reports, but they must not execute, change validation status, or become public runtime/API formulas without an explicit reviewed promotion task and evidence packet.

## Authoritative source hierarchy

1. Current repository `main` at the task base SHA and the user-authorized branch for the active task.
2. The v0.7.2 planning package, especially the master execution plan, task cue card, detailed task queue JSON/CSV, and changelog.
3. Repository roadmap, architecture, assurance, testing, schema, and template docs already merged from LOCK-001 through LOCK-005.
4. `repo_ready/` package content as reference wording only; do not blindly copy it into the repository.
5. Older v0.3, v0.6, v0.7.0, or v0.7.1 files only when v0.7.2 explicitly incorporates or references them.

When sources conflict, the v0.7.2 controlling plan and the current merged repository state govern this queue.

## Safety/public wording

Use conservative research/preliminary-design wording. The approved public posture is:

> AeroCodex is intended to become professional-grade, traceable aerospace research software suitable for academic, laboratory, and agency evaluation. It is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

Do not convert negative safety caveats into positive readiness or approval claims. Do not weaken non-claim wording when adding user-facing docs, CLI output, registry metadata, or release materials.

## Sequencing pressure

The sequence is intentionally conservative: lock task boundaries and contracts first, then parser/verifier, registry generation, CLI/JSON/error contracts, status gates, M00 Slice A execution, promotion evidence, inventory, M07 quarantine, and later family promotion. JSON/error envelopes and fail-closed status gates must precede input parsing and execution dispatch.

## What should not be touched yet

Do not modify Rust runtime or formula execution code, equation-batch TSVs, validation statuses, validation cards, M07 quarantine/status files, generated registry artifacts, equation manifests, `Cargo.toml`, or `Cargo.lock` as part of this decision-packet task. Do not make formulas executable, promote statuses, broaden M00 scope, or start RR-002 or any later RR task from this packet.

## Promotion review model

Sidecars enrich metadata; promotion packets change status. Any status promotion requires a reviewed packet with source traceability, implementation evidence, test vectors, tolerance policy, schema validation, reviewer notes, rollback path, and deterministic registry regeneration. Promotion should be family-by-family or slice-by-slice, never a bulk unlock of unknown or quarantined formulas.

## Agent instruction to use going forward

For future AeroCodex research-readiness work, agents should begin from fresh `main`, read the controlling v0.7.2 task card and queue records, check dependency/conflict docs, implement exactly one assigned task, run the task-specific checks plus forbidden-scope checks, report changed files and evidence, and stop for maintainer approval before committing, pushing, merging, or starting the next task when the prompt requires a local handoff only.
