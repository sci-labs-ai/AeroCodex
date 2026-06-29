# Beta 1 release-candidate testing

Status: `research_required`

This public repository keeps the Beta 1 concept release check Rust-only. The former deployment packaging helpers are not tracked here. This procedure validates a clean checkout and a locally built `aerocodex` binary; it is a software release-engineering gate, not an aerospace assurance or certification gate.

The current candidate surface remains exactly ten governed M00 canonical-unit formulas. The 1,000+ equation backlog is outside this release-candidate scope.

## Prerequisites

- Git
- Rust and Cargo compatible with the workspace `rust-version`
- `rustfmt` and Clippy for the normal repository gate

A root `Cargo.lock` remains intentionally uncommitted while every Cargo dependency is workspace-local and path-only.

## Public Rust-only release-candidate check

Run from a clean checkout:

```bash
git status --short
git diff --check
sha256sum -c checksums/SHA256SUMS
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo run -p aero-codex-cli -- version --json
cargo run -p aero-codex-cli -- self-check --json
```

The gate is public-source validation only. It does not create a Git tag, GitHub release, upload, signing bundle, or published artifact. Any private packaging or distribution automation should live outside the public repository unless the maintainers deliberately re-adopt it.

## Candidate acceptance

A candidate is testable when:

- repository CI passes;
- the Rust-only public gate above passes;
- `aerocodex self-check --json` reports zero failures;
- the manifest and CLI report `release_channel=beta1-concept` and `package_version=0.0.1`;
- validation remains `research_required`;
- no operational-readiness, certification, full-inventory, external-parity, or safety claim is added.

Passing this gate authorizes private or internal Beta 1 concept testing only. Publication, signing, tagging, or broader distribution requires a separate release decision.
