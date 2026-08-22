# Beta 1 concept CLI quickstart

<!-- aerocodex-current-identity:start -->
Release version: `0.1.0-alpha.1`
Release tier: `research_software_alpha` (`Research Software Alpha`)
Workspace packages: `14`
Registry formulas: `152`
Blocked formulas: `140`
Publicly executable formulas: `12`
<!-- aerocodex-current-identity:end -->

This page preserves the historical Beta 1 command surface as compatibility documentation. The current `aerocodex` binary is a bounded, research-only inventory and execution surface with twelve `implementation_verified` / `normal_research` M00 formulas: ten canonical-unit records and two angle conversions. The other 140 registry rows remain `research_required` / `blocked`. Beta 1 is not the current runtime identity.

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

## Run a promoted formula

```bash
cargo run -p aero-codex-cli -- formula run \
  m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
```

The command succeeds with a versioned JSON envelope. The expected state fields include:

```json
{"ok":true,"command":"formula run","json_contract_version":"aerocodex.cli.json.v1","formula_id":"m00.canonical.distance_to_canonical","value":-6,"status":"implementation_verified","execution_policy":"normal_research","error":null}
```

The real output also includes registry traceability and the safety notice. The legacy `run formula_vault.m00.canonical.distance_to_canonical ... --json` form routes through the same resolver, parser, status gate, evaluator, and envelope builder.

## Run the bounded self-check

```bash
cargo run -p aero-codex-cli -- self-check --json
```

A clean run reports `"passed":14` and `"failed":0`. Self-check exercises the public formula-run path for the ten canonical-unit kernels plus invalid-scale, nonfinite-input, overflow, and unknown-formula rejection. It is a software diagnostic, not scientific reference validation or certification.

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

The command exits with code `4` and writes a versioned JSON error containing the stable equation code `non_positive_input`; the promoted formula has passed the status and dispatch gates before equation-domain evaluation.

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

The release slice is `implementation_verified`; the 140-formula complement remains `research_required`. This historical Beta 1 compatibility surface is not a certified or operational release; the current release identity is `0.1.0-alpha.1` and `research_software_alpha`. See [`release_testing.md`](release_testing.md).
