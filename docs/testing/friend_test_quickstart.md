# AeroCodex friend-test quickstart

This friend-test runs the public Rust-only repository gate from a local checkout. It exercises formatting, build, Clippy, tests, governed metadata checks through `xtask`, dependency policy, documentation, and the Beta 1 CLI smoke path.

Passing this package does **not** prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval.

## Prerequisites

Install the Rust toolchain with `cargo`, `rustc`, `rustfmt`, and `clippy` available on your command search path. The scripts also require `git`. The Bash script requires `sha256sum`; the PowerShell script can use either `sha256sum` or its built-in `Get-FileHash` fallback.

```bash
cargo --version
rustc --version
git --version
```

## Run the package

On macOS/Linux:

```bash
scripts/friend_test_local.sh
```

On Windows PowerShell:

```powershell
.\scripts\friend_test_local.ps1
```

## CI-equivalent sequence

The scripts run this sequence in order:

```bash
git status --short
git diff --check
sha256sum -c checksums/SHA256SUMS
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo run -p aero-codex-cli -- version --json
cargo run -p aero-codex-cli -- run formula_vault.m00.canonical.distance_to_canonical distance=-42 distance_unit=7 --json
cargo run -p aero-codex-cli -- self-check --json
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
```

## What to report

Include the OS, Rust versions, the exact failing command, the first error line, and whether a root `Cargo.lock` appeared after the run. Do not report a green friend-test as certification, flight readiness, habitat safety, medical suitability, or regulated-use approval.
