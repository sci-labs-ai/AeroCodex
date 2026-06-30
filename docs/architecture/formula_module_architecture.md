# Formula Module and File Architecture v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md`, task `LOCK-003`.

This document fixes the module/file architecture for the research-readiness formula infrastructure. It is a docs-only contract. It does not create modules, generate registries, promote formula status, modify M07 quarantine behavior, or change runtime execution.

AeroCodex is professional research/preliminary-design software. It is not certified operational aerospace software.

## Required xtask layout

Formula infrastructure commands must be routed through the existing `xtask` crate. Agents must not add one-off shell scripts or duplicate command routers when these paths own the responsibility.

```text
xtask/src/main.rs
xtask/src/equation_batch/mod.rs
xtask/src/equation_batch/manifest.rs
xtask/src/equation_batch/plan.rs
xtask/src/equation_batch/validate.rs
xtask/src/equation_batch/verify.rs
xtask/src/equation_batch/report.rs
xtask/src/formula_registry/mod.rs
xtask/src/formula_registry/schema.rs
xtask/src/formula_registry/sidecar.rs
xtask/src/formula_registry/generate.rs
xtask/src/formula_registry/check.rs
xtask/src/formula_registry/m00.rs
```

Ownership notes:

- `xtask/src/equation_batch/manifest.rs` owns equation-batch manifest loading and row-level metadata parsing.
- `xtask/src/equation_batch/plan.rs` owns queue/plan summaries derived from equation-batch manifests.
- `xtask/src/equation_batch/validate.rs` owns static manifest validation.
- `xtask/src/equation_batch/verify.rs` owns generated probe/check execution once implementation tasks add it.
- `xtask/src/equation_batch/report.rs` owns deterministic report formatting for equation-batch infrastructure.
- `xtask/src/formula_registry/schema.rs` owns registry schema validation helpers.
- `xtask/src/formula_registry/sidecar.rs` owns formula sidecar parsing and validation.
- `xtask/src/formula_registry/generate.rs` owns generated registry artifacts.
- `xtask/src/formula_registry/check.rs` owns stale/generated-artifact checks.
- `xtask/src/formula_registry/m00.rs` owns M00 Slice A registry selection and checks.

## Required CLI layout

Formula CLI work must stay below `crates/aero-codex-cli/src/formula/` after the top-level command dispatch in `main.rs`. Agents must not dump formula logic into `crates/aero-codex-cli/src/main.rs`.

```text
crates/aero-codex-cli/src/main.rs
crates/aero-codex-cli/src/formula/mod.rs
crates/aero-codex-cli/src/formula/list.rs
crates/aero-codex-cli/src/formula/describe.rs
crates/aero-codex-cli/src/formula/run.rs
crates/aero-codex-cli/src/formula/input.rs
crates/aero-codex-cli/src/formula/output.rs
crates/aero-codex-cli/src/formula/gates.rs
crates/aero-codex-cli/src/formula/errors.rs
```

Ownership notes:

- `list.rs` owns listing/search presentation.
- `describe.rs` owns formula detail presentation.
- `run.rs` owns formula run dispatch, after status gates and input parsing.
- `input.rs` owns CLI input parsing and unit-aware argument mapping.
- `output.rs` owns human and JSON output formatting.
- `gates.rs` owns execution-gate checks before dispatch.
- `errors.rs` owns public CLI error mapping after the error-code contract task defines the catalog.

## Future registry crate layout

The future registry crate must be a single registry/runtime boundary. Agents must not create a second registry format or an alternate crate path.

```text
crates/aero-codex-registry/Cargo.toml
crates/aero-codex-registry/src/lib.rs
crates/aero-codex-registry/src/generated.rs
crates/aero-codex-registry/src/model.rs
crates/aero-codex-registry/src/gates.rs
crates/aero-codex-registry/src/errors.rs
crates/aero-codex-registry/src/executor.rs
crates/aero-codex-registry/examples/list_formulas.rs
crates/aero-codex-registry/examples/describe_formula.rs
crates/aero-codex-registry/examples/run_m00_deg_to_rad.rs
```

Ownership notes:

- `src/generated.rs` is generated from the canonical registry artifacts and must not be hand-edited after generator tasks exist.
- `src/model.rs` owns typed registry records.
- `src/gates.rs` owns reusable status/execution gate checks.
- `src/errors.rs` owns registry crate error types and conversion boundaries.
- `src/executor.rs` owns formula execution dispatch after gate checks.

## Do-not-dump and do-not-fork rules

- Do not dump formula logic into `crates/aero-codex-cli/src/main.rs`.
- Do not invent alternate generated artifact paths.
- Do not create a second registry format.
- Do not create runtime formula execution paths before the registry, gate, and CLI contracts allow them.
- Do not change validation status, M07 quarantine, equation-batch TSVs, or generated registry artifacts in this LOCK-003 task.
