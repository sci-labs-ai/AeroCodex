# AeroCodex friend-test quickstart

This friend-test runs the public Rust-only gate from either a Git checkout or the downloaded source archive paired with a downloaded binary. It exercises the committed lockfile, formatting, build, Clippy, tests, governed metadata checks through `xtask`, release-manifest/checksum/generated-artifact integrity, dependency policy, documentation, and the Research Software Alpha CLI inventory/status path.

Passing this package does **not** prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval.

## Prerequisites

Install the Rust toolchain with `cargo`, `rustc`, `rustfmt`, and `clippy` available on your command search path. Git is required for checkout mode but is not required inside the source archive. Checksum verification is implemented by the cross-platform Rust `xtask` command and does not require GNU coreutils.

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

For the governed downloaded-artifact test, extract the source archive and the matching binary archive. From the extracted source directory, set the absolute binary path and the 40-character commit reported for the release, then run the same public script:

```bash
AEROCODEX_FRIEND_BINARY=/absolute/path/to/aerocodex \
AEROCODEX_EXPECTED_COMMIT=<40-character-release-commit> \
scripts/friend_test_local.sh
```

```powershell
$env:AEROCODEX_FRIEND_BINARY = "C:\absolute\path\to\aerocodex.exe"
$env:AEROCODEX_EXPECTED_COMMIT = "<40-character-release-commit>"
.\scripts\friend_test_local.ps1
```

In archive mode, steps 1 and 2 verify the source-archive prerequisites instead of consulting absent Git metadata. The three public CLI steps execute the downloaded binary. The aggregate source check uses the explicitly narrower `xtask verify-source-archive` command, and the release-identity step verifies that the binary's embedded commit and manifest hash match the extracted source. Checkout mode retains the full Git ancestry verifier.

## CI-equivalent sequence

In checkout mode the scripts run this sequence in order. Archive mode preserves the same 16-step gate with the substitutions described above.

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

Include the OS, Rust versions, test mode (`checkout` or `downloaded artifacts`), exact commit, archive names, the exact failing command, and the first error line. The committed root `Cargo.lock` must remain byte-identical. Do not report a green friend-test as certification, flight readiness, habitat safety, medical suitability, or regulated-use approval.
