# Beta 1 concept CLI quickstart

<!-- aerocodex-current-identity:start -->
Release version: `0.1.0-alpha.1`
Release tier: `research_software_alpha` (`Research Software Alpha`)
Workspace packages: `14`
Registry formulas: `152`
Blocked formulas: `152`
Publicly executable formulas: `0`
<!-- aerocodex-current-identity:end -->

This page preserves the historical Beta 1 command surface as compatibility documentation. The current `aerocodex` binary is a bounded, research-only inventory and status surface with twelve M00 dispatch-linked records: ten canonical-unit records and two angle-conversion records. The current registry keeps every record at `research_required` with `execution_policy=blocked`, so none is publicly executable. Runtime dispatch links support internal self-checking and future governed promotion work; they are not execution authorization. Beta 1 is not the current runtime identity.

It is not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use.

## Build and inspect

```bash
cargo run -p aero-codex-cli -- version
cargo run -p aero-codex-cli -- version --json
cargo run -p aero-codex-cli -- formula list
cargo run -p aero-codex-cli -- formula list --json
```

The older `formulas` alias remains available during the migration window and reports `deprecated_alias=true` in JSON output. Prefer `aerocodex formula list` for new scripts.

## Describe a formula

```bash
cargo run -p aero-codex-cli -- formula describe \
  m00.canonical.distance_to_canonical --json
```

The legacy `describe formula_vault.m00.canonical.distance_to_canonical --json` form is still accepted and reports the canonical formula ID plus `alias_used`/`deprecated_alias` migration fields.

## Observe the fail-closed execution gate

```bash
cargo run -p aero-codex-cli -- formula run \
  m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
```

The command exits with code `4` and reports `execution_blocked_by_status` before runtime dispatch. The expected status fields include:

```json
{"ok":false,"command":"formula run","formula_id":"m00.canonical.distance_to_canonical","status":"research_required","execution_policy":"blocked","error":{"code":"execution_blocked_by_status"}}
```

The real output also includes registry traceability and the safety notice. The legacy `run formula_vault.m00.canonical.distance_to_canonical ... --json` form routes through the same status gate and cannot bypass it.

## Run the bounded self-check

```bash
cargo run -p aero-codex-cli -- self-check --json
```

A clean run reports `"passed":14` and `"failed":0`. Self-check directly exercises the ten canonical-unit kernels plus invalid-scale, nonfinite-input, overflow, and unknown-formula rejection. It is a software diagnostic, not a public-execution or validation-status claim.

## Stable exit codes

| Code | Meaning |
|---:|---|
| 0 | Command succeeded. |
| 2 | Usage, assignment, number parsing, missing-input, or unexpected-input error. |
| 3 | Unknown formula ID. |
| 4 | Existing AeroCodex equation rejected the input, produced a checked numerical failure, or the formula is present in the registry but not executable through the Beta 1 CLI surface. |
| 5 | Built-in self-check found one or more failures. |

## Machine-readable error example

```bash
cargo run -p aero-codex-cli -- formula run \
  m00.canonical.distance_to_canonical \
  distance=1 distance_unit=0 --json
```

The command exits with code `4` and writes a JSON error containing the stable code `execution_blocked_by_status`; the status gate runs before equation-domain evaluation.

## Release-gate commands

```bash
cargo test -p aero-codex-cli --all-targets
cargo run -p aero-codex-cli -- self-check --json
cargo run -p xtask -- verify beta1
```

The complete repository gate is `cargo run -p xtask -- verify --all` plus the other Rust CI commands documented in the friend-test quickstart.
## Public release-candidate gate

From a clean checkout, run:

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo run -p aero-codex-cli -- self-check --json
```

Formula validation remains `research_required`. This historical Beta 1 compatibility check is not a certified or operational release; the current release identity is `0.1.0-alpha.1` and `research_software_alpha`. See [`release_testing.md`](release_testing.md).
