# Research Readiness Dependency/Conflict Matrix v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md` and `AeroCodex_Research_Readiness_Detailed_Task_Queue_v0_7_2.json` from the v0.7.2 research-readiness planning package.

This matrix implements LOCK-002. It is a planning and review contract for queue-safe parallel work; it does not change code, formula status, generated registry artifacts, equation manifests, runtime execution, or M07 quarantine behavior.

## Conflict-start rule

Agents must not start a conflicting task until the dependency has merged, or until a maintainer explicitly partitions the files and records the partition in the task handoff. If a task touches a serialized zone, treat the task as serialized even when another row looks parallel-safe.

## Serialized zones

The following zones are serialized unless a maintainer explicitly partitions files:

- xtask command routing
- equation-batch parser/model
- equation-batch verifier/probe generator
- registry generator
- generated registry artifacts
- CLI formula run dispatch
- CLI JSON/error envelopes
- CLI status gates
- workspace Cargo.toml
- validation status files
- M07 resolution/quarantine files

## Critical ordering constraints

- RR-024 and RR-025 must merge before RR-022, RR-023, and RR-031.
- RR-018 must merge before RR-015.
- RR-055 must define fixtures/golden-output paths before parser, verifier, or CLI output tasks rely on them.
- RR-037 through RR-040 keep M07 visible but blocked unless a later explicit family-promotion task changes status.

## Matrix

| task | must_merge_before | conflicts_with | safe_parallel_work | serialized_reason |
|---|---|---|---|---|
| RR-005 | RR-055, RR-006 through RR-012 | equation-batch parser/model; xtask command routing | RR-001 through RR-004 after file partitioning; docs-only tasks that do not touch xtask equation-batch files | Serialized foundation for manifest model and parser types. |
| RR-055 | Parser, verifier, registry, or CLI output tasks that rely on shared fixtures/golden outputs | parser tests; verifier tests; CLI output snapshots; fixture path ownership | Docs-only tasks that do not create fixture or golden-output path contracts | Serialized fixture/golden-output path contract before downstream tests rely on those paths. |
| RR-006 | RR-007, RR-008, RR-013; RR-009 after RR-007 | xtask command routing; equation-batch parser/model | Docs-only tasks that do not edit xtask routing or equation-batch model files | Serialized plan command work because later static validation and schema tasks consume the plan output. |
| RR-007 | RR-008, RR-009 | xtask command routing; equation-batch parser/model | Docs-only tasks; registry schema work only after RR-006 output is stable | Serialized static validation layer before probe generation and status reporting. |
| RR-008 | RR-012, RR-015 | generated equation-batch status report; registry generator if both write generated paths | CI docs after output path is stable | Serialized generated-artifact contract before registry generation consumes status report semantics. |
| RR-009 | RR-010 | equation-batch verifier/probe generator; xtask command routing | Docs-only tasks that do not edit xtask equation-batch files | Serialized probe generator before verify command. |
| RR-010 | RR-011, RR-029 | equation-batch verifier/probe generator; xtask command routing | Docs-only tasks and non-overlapping CI docs | Serialized verify command before verify-all and later promotion evidence uses it. |
| RR-011 | RR-012, RR-054 | xtask command routing; equation-batch verifier/probe generator; CI check semantics | CI workflow docs after command shape is stable | Serialized verify-all/check-mode command before CI and agent PR check scripts depend on it. |
| RR-012 | none declared; must follow RR-008 and RR-011 | GitHub Actions workflow files; CI gate semantics | Docs-only tasks and non-overlapping implementation tasks after required commands merge | Serialized CI gate wiring because workflow changes should not race command semantics. |
| RR-013 | RR-014, RR-018, RR-015, RR-027, RR-033 | registry schema design; status-gate policy schema references | Docs-only tasks that do not alter registry schema fields | Serialized registry schema foundation before sidecars, generator, promotion template, and runtime inventory schema. |
| RR-014 | RR-018, RR-015, RR-028, RR-042 | formula sidecar schema and sidecar path contract | Docs-only tasks that do not edit formula-schemas paths | Serialized sidecar contract before generator and sidecar population tasks rely on it. |
| RR-018 | RR-015, RR-019 | formula ID alias policy; registry contract ID normalization | RR-017 only if docs do not overlap; unrelated docs-only tasks | Serialized alias/migration policy must merge before RR-015 normalizes IDs and before CLI namespace exposes aliases. |
| RR-015 | RR-016, RR-017, RR-019, RR-034, RR-038, RR-049 | registry generator; generated registry artifacts; xtask command routing | Docs-only tasks that do not touch registry contracts or generated paths | Serialized registry generator before generated Rust module, stale checks, CLI namespace, runtime inventory, M07 inventory, and Rust API crate. |
| RR-016 | RR-017, RR-020, RR-049 | generated registry artifacts; generated Rust module path | Docs-only tasks and non-overlapping schema docs | Serialized generated Rust registry module before registry checks, CLI list backing, and registry crate boundary. |
| RR-017 | RR-054, RR-049 | registry check commands; CI workflows; xtask command routing | CI docs after command semantics are stable | Serialized stale/schema check commands before agent PR checks and Rust API crate setup depend on them. |
| RR-019 | RR-020 | CLI formula command namespace; CLI tests | Docs-only tasks that avoid CLI quickstart overlap | Serialized CLI namespace before formula list/describe/run tasks. |
| RR-020 | RR-021, RR-026 | CLI registry lookup; generated registry consumption; CLI tests | Docs-only tasks and non-overlapping registry docs after RR-016 | Serialized formula list backing before describe and status-report CLI work. |
| RR-021 | RR-024 | CLI describe output; traceability metadata display | Docs-only tasks that do not edit CLI quickstart or JSON contract | Serialized describe output before stable JSON/error envelopes. |
| RR-024 | RR-025, RR-022, RR-023, RR-031 | CLI JSON/error envelopes; CLI tests; JSON contract docs | Docs-only tasks that do not alter CLI JSON/error contract | Serialized JSON/error envelope contract must merge before status gates, input parser, run dispatch, and normal execution. |
| RR-025 | RR-026, RR-027, RR-022, RR-023, RR-031, RR-040, RR-051 | CLI status gates; validation gate policy references; CLI tests | Registry docs that do not alter status-gate semantics | Serialized status gates must merge before input parsing, run dispatch, M00 normal execution, M07 blocked tests, and Rust executor gates. |
| RR-026 | RR-036 | CLI status-report command; CLI tests | Docs-only tasks after RR-020/RR-025 | Serialized with core CLI files; avoid overlap with run dispatch tasks. |
| RR-022 | RR-023 | CLI input parser; CLI formula run arguments; CLI tests | Docs-only tasks that do not edit CLI quickstart examples | Serialized input parser after RR-024/RR-025 and before execution dispatch. |
| RR-023 | RR-031, RR-051 | CLI formula run dispatch; runtime invocation boundary; CLI tests | Docs-only tasks that do not alter formula execution examples | Serialized run dispatch before normal M00 execution and Rust executor skeleton. |
| RR-027 | RR-028, RR-029 | promotion packet template; validation card README/checklist | Docs-only tasks that do not touch promotion template/checklist files | Serialized promotion template before M00 sidecars and promotion evidence. |
| RR-028 | RR-029 | M00 sidecar files and formula-schemas paths | Runtime inventory tasks if formula-schemas files do not overlap | Serialized M00 sidecars before promotion evidence references them. |
| RR-029 | RR-030 | promotion packets; implementation verification evidence; validation test vectors | Docs-only tasks that do not touch promotion packets/checklists | Serialized evidence before validation status changes. |
| RR-030 | RR-031, RR-051 | equation-batch TSVs; validation status files | No parallel implementation work touching equation-batches or validation cards | Serialized validation status promotion; must not overlap runtime, registry, or manifest plumbing. |
| RR-031 | RR-032, RR-044 | CLI formula run dispatch; generated registry use; normal execution docs | Researcher docs after execution is verified | Serialized normal execution after RR-023, RR-025, and RR-030. |
| RR-033 | RR-034 | runtime inventory schema; generated runtime inventory path | M00 docs if generated/runtime inventory paths do not overlap | Serialized runtime inventory schema before scanner generation. |
| RR-034 | RR-035, RR-036 | runtime inventory scanner; generated runtime inventory artifact; xtask command routing | Docs-only tasks after generated path ownership is clear | Serialized runtime scanner before promotion-candidate report and CLI inventory visibility. |
| RR-035 | RR-041 | promotion-candidate report; generated promotion candidate artifact | M07 policy docs if generated report paths do not overlap | Serialized promotion-candidate report before A4 readiness report. |
| RR-036 | RR-044 | CLI runtime inventory/status summary; CLI tests | Release docs after CLI visibility semantics are stable | Serialized CLI visibility task because it edits core CLI files. |
| RR-037 | RR-038, RR-039, RR-040, RR-045 | M07 quarantine policy; M07 candidate inventory/count docs; M07 resolution/quarantine files | M00 work and runtime inventory when docs paths do not overlap | Serialized M07 quarantine baseline keeps M07 visible but blocked; no status promotion. |
| RR-038 | RR-040 | registry generator; generated registry artifacts; M07 blocked inventory | RR-039 only if docs paths do not overlap and no generated registry paths are touched | Serialized M07 blocked candidates in registry; M07 remains non-executable inventory. |
| RR-039 | RR-041 | M07 promotion SOP; promotion packet template docs; formal plan references | RR-038 if files are partitioned and M07 status remains blocked | Serialized M07 promotion procedure; does not authorize family promotion or status changes. |
| RR-040 | RR-044 | CLI status-gate tests; registry checks; M07 blocked execution assertions | Release docs after blocked execution tests merge | Serialized CLI/registry tests confirm M07 remains visible but blocked. |
| RR-049 | RR-050, RR-051, RR-052, RR-053 | workspace Cargo.toml; registry crate boundary; generated Rust registry path | Docs-only tasks after registry generation paths are stable | Serialized workspace Cargo.toml/crate boundary edit. |
| RR-050 | RR-051, RR-052, RR-053 | registry crate API model/status/error modules | Docs-only tasks and examples after API names are stable | Serialized FormulaRegistry API before executor skeleton and examples. |
| RR-051 | RR-052 | FormulaExecutor skeleton; status-gated execution boundary; registry crate errors | Docs-only examples after executor API is stable | Serialized executor skeleton with CLI execution tasks and M00 promotion status gates. |
| RR-052 | none declared | Rust examples that call registry/executor APIs | Docs-only tasks that do not edit the same examples or quickstart files | Safe after RR-050/RR-051; avoid overlapping example paths. |
| RR-053 | none declared | Rust API stability docs; registry crate README | RR-052 if README/examples files are partitioned | Docs-only after RR-049/RR-050; avoid Cargo version edits unless a release task owns them. |
