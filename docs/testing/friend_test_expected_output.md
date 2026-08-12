# AeroCodex friend-test expected output

A successful run prints the friend-test header, the repository root, Rust toolchain versions, then fifteen numbered steps.

Representative skeleton:

```text
[friend-test] AeroCodex local friend-test package
[friend-test] repository root: <path-to-checkout>
[friend-test] rustc: <version>
[friend-test] cargo: <version>
[friend-test] git commit: <short-hash>
[friend-test] step 1/15: git status --short
[friend-test] step 2/15: git diff --check
[friend-test] step 3/15: cargo run -p xtask -- verify-checksums
[friend-test] step 4/15: cargo fmt --all -- --check
[friend-test] step 5/15: cargo check --workspace --all-targets --all-features
[friend-test] step 6/15: cargo clippy --workspace --all-targets --all-features -- -D warnings
[friend-test] step 7/15: cargo test --workspace --all-targets --all-features
[friend-test] step 8/15: cargo run -p aero-codex-cli -- version --json
[friend-test] step 9/15: cargo run -p aero-codex-cli -- formula status-report --json
[friend-test] step 10/15: cargo run -p aero-codex-cli -- self-check --json
[friend-test] step 11/15: cargo run -p xtask -- verify --all
[friend-test] step 12/15: cargo run -p xtask -- verify-release-manifest
[friend-test] step 13/15: cargo run -p xtask -- verify-generated
[friend-test] step 14/15: cargo run -p xtask -- dependency-policy
[friend-test] step 15/15: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
[friend-test] completed all requested local checks
```

The Beta 1 CLI self-check should report zero failures. The governance command should verify validation cards, source registry, data registry, status vocabulary, formula-vault records, equation inventory, equation-batch manifests, and the Beta 1 CLI concept without promoting any safety or certification status.
