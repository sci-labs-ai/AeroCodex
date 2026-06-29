# AeroCodex friend-test expected output

A successful run prints the friend-test header, the repository root, Rust toolchain versions, then thirteen numbered steps.

Representative skeleton:

```text
[friend-test] AeroCodex local friend-test package
[friend-test] repository root: <path-to-checkout>
[friend-test] rustc: <version>
[friend-test] cargo: <version>
[friend-test] git commit: <short-hash>
[friend-test] step 1/13: git status --short
[friend-test] step 2/13: git diff --check
[friend-test] step 3/13: sha256sum -c checksums/SHA256SUMS
[friend-test] step 4/13: cargo fmt --all -- --check
[friend-test] step 5/13: cargo check --workspace --all-targets --all-features
[friend-test] step 6/13: cargo clippy --workspace --all-targets --all-features -- -D warnings
[friend-test] step 7/13: cargo test --workspace --all-targets --all-features
[friend-test] step 8/13: cargo run -p aero-codex-cli -- version --json
[friend-test] step 9/13: cargo run -p aero-codex-cli -- run canonical distance smoke
[friend-test] step 10/13: cargo run -p aero-codex-cli -- self-check --json
[friend-test] step 11/13: cargo run -p xtask -- verify --all
[friend-test] step 12/13: cargo run -p xtask -- dependency-policy
[friend-test] step 13/13: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
[friend-test] completed all requested local checks
```

The Beta 1 CLI self-check should report zero failures. The governance command should verify validation cards, source registry, data registry, status vocabulary, formula-vault records, equation inventory, equation-batch manifests, and the Beta 1 CLI concept without promoting any safety or certification status.
