# Beta 1 concept CLI quickstart

The `aerocodex` binary is a bounded, research-only execution surface for ten M00 canonical-unit formulas. It is intended for software testing, integration experiments, and release-process validation.

It is not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use.

## Build and inspect

```bash
cargo run -p aero-codex-cli -- version
cargo run -p aero-codex-cli -- version --json
cargo run -p aero-codex-cli -- formulas
```

## Describe a formula

```bash
cargo run -p aero-codex-cli -- describe \
  formula_vault.m00.canonical.distance_to_canonical --json
```

## Run a signed conversion

```bash
cargo run -p aero-codex-cli -- run \
  formula_vault.m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
```

The expected output contains:

```json
{"ok":true,"command":"run","formula_id":"formula_vault.m00.canonical.distance_to_canonical","runtime_symbol":"m00_distance_to_canonical","output_variable":"canonical_distance","value":-6,"validation_status":"research_required"}
```

The real output also includes the safety notice.

## Run the bounded self-check

```bash
cargo run -p aero-codex-cli -- self-check --json
```

A clean run reports `"passed":14` and `"failed":0`. The checks cover all ten formulas plus invalid-scale, nonfinite-input, overflow, and unknown-formula rejection.

## Stable exit codes

| Code | Meaning |
|---:|---|
| 0 | Command succeeded. |
| 2 | Usage, assignment, number parsing, missing-input, or unexpected-input error. |
| 3 | Unknown formula ID. |
| 4 | Existing AeroCodex equation rejected the input or produced a checked numerical failure. |
| 5 | Built-in self-check found one or more failures. |

## Machine-readable error example

```bash
cargo run -p aero-codex-cli -- run \
  formula_vault.m00.canonical.distance_to_canonical \
  distance=1 distance_unit=0 --json
```

The command exits with code `4` and writes a JSON error containing the stable code `non_positive_input`.

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

This remains a `research_required` Beta 1 concept check, not a certified or operational release. See [`release_testing.md`](release_testing.md).
