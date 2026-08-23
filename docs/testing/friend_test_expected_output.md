# AeroCodex friend-test expected output

A successful run prints the friend-test header, the repository root, Rust toolchain versions, then sixteen numbered steps using the committed lockfile.

Downloaded-artifact mode additionally prints `source archive mode`, the downloaded binary path, and confirmation that the binary's embedded commit and release-manifest hash match the source archive. Its numbered step labels remain stable so reports from checkout and archive modes can be compared directly.

Representative skeleton:

```text
[friend-test] AeroCodex local friend-test package
[friend-test] repository root: <path-to-checkout>
[friend-test] rustc: <version>
[friend-test] cargo: <version>
[friend-test] git commit: <short-hash>
[friend-test] step 1/16: git status --short
[friend-test] step 2/16: git diff --check
[friend-test] step 3/16: cargo run --locked -p xtask -- verify-checksums
[friend-test] step 4/16: cargo fmt --all -- --check
[friend-test] step 5/16: cargo check --locked --workspace --all-targets --all-features
[friend-test] step 6/16: cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
[friend-test] step 7/16: cargo test --locked --workspace --all-targets --all-features
[friend-test] step 8/16: cargo run --locked -p aero-codex-cli -- version --json
[friend-test] step 9/16: cargo run --locked -p aero-codex-cli -- formula status-report --json
[friend-test] step 10/16: cargo run --locked -p aero-codex-cli -- self-check --json
[friend-test] step 11/16: cargo run --locked -p xtask -- verify --all
[friend-test] step 12/16: cargo run --locked -p xtask -- verify-release-manifest
[friend-test] step 13/16: cargo run --locked -p xtask -- verify-release-identity
[friend-test] step 14/16: cargo run --locked -p xtask -- verify-generated
[friend-test] step 15/16: cargo run --locked -p xtask -- dependency-policy
[friend-test] step 16/16: RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps
[friend-test] completed all requested local checks
```

The CLI self-check should report `14` passed and `0` failed. The status report should show `152` registry formulas, `12` dispatchable and publicly executable formulas, and `140` blocked formulas. The governance command verifies validation cards, source registry, data registry, status vocabulary, formula-vault records, equation inventory, equation-batch manifests, and the historical Beta 1 compatibility material without promoting any safety or certification status.
