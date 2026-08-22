# AeroCodex friend-test quickstart

This friend-test runs the public Rust-only repository gate from a local checkout. It exercises the committed lockfile, formatting, build, Clippy, tests, governed metadata checks through `xtask`, release-manifest/checksum/generated-artifact integrity, dependency policy, documentation, and the Research Software Alpha CLI inventory/status path.

Passing this package does **not** prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval.

## Prerequisites

Install the Rust toolchain with `cargo`, `rustc`, `rustfmt`, and `clippy` available on your command search path. The scripts also require `git`. Checksum verification is implemented by the cross-platform Rust `xtask` command and does not require GNU coreutils.

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
cargo run --locked -p xtask -- verify-checksums
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-targets --all-features
cargo run --locked -p aero-codex-cli -- version --json
cargo run --locked -p aero-codex-cli -- formula status-report --json
cargo run --locked -p aero-codex-cli -- self-check --json
cargo run --locked -p xtask -- verify --all
cargo run --locked -p xtask -- verify-release-manifest
cargo run --locked -p xtask -- verify-release-identity
cargo run --locked -p xtask -- verify-generated
cargo run --locked -p xtask -- dependency-policy
RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps
```

## What to report

Include the OS, Rust versions, exact commit, the exact failing command, and the first error line. The committed root `Cargo.lock` must remain byte-identical. Do not report a green friend-test as certification, flight readiness, habitat safety, medical suitability, or regulated-use approval.
