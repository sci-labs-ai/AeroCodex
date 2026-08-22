# Contributing to AeroCodex

AeroCodex accepts software corrections, documentation improvements, validation evidence, and carefully bounded formula promotions. Every contribution must preserve the Research Software Alpha safety boundary.

## Code and documentation changes

1. Open a focused issue or pull request describing the user-visible or assurance outcome.
2. Keep unrelated changes separate.
3. Add or update tests for behavior changes.
4. Run the governed checks in `docs/development/ci_gates.md` with the committed `Cargo.lock` and `--locked`.
5. Regenerate governed outputs and `checksums/SHA256SUMS` when their sources change.

Rust changes must remain safe Rust and dependency-minimal. New third-party dependencies require an explicit license, vulnerability, maintenance, and reproducibility review before they enter `Cargo.lock`.

## Formula contribution path

A formula candidate does not become executable merely because code exists. A promotion must include:

- a stable formula ID and contract;
- source-registry and validation-card links;
- domain, units, assumptions, failure behavior, and tolerance policy;
- runtime implementation and independently stated analytical vectors;
- deterministic edge, invalid-input, round-trip, and cross-platform tests where applicable;
- a completed promotion packet;
- an audited registry-status and execution-policy change;
- exact CLI list, describe, run, self-check, generated-artifact, and checksum updates.

Use `docs/assurance/promotion_packet_template.md` and `docs/templates/formula_sidecar_template.yaml`. Status promotion requires review; a pull request must not imply scientific validation or certification beyond its evidence.

## Publication boundary

Only `aero-codex-core`, `aero-codex-constants`, `aero-codex-astrodynamics`, and `aero-codex-cli` are approved Cargo package surfaces for this alpha. Other workspace crates and `xtask` declare `publish = false`. GitHub Release binaries remain the primary alpha distribution channel.

By participating, contributors agree to `CODE_OF_CONDUCT.md`.
