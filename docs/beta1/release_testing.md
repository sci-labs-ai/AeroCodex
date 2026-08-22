# Beta 1 release-candidate testing

Status: `research_software_alpha`; release slice `implementation_verified`

This page preserves the historical Beta 1 test procedure as compatibility guidance. The public repository keeps the release check Rust-only. The former deployment packaging helpers are not tracked here. This procedure validates a clean checkout and a locally built `aerocodex` binary; it is a software release-engineering gate, not an aerospace assurance or certification gate.

The current candidate has twelve M00 dispatch-linked and publicly executable records (ten canonical-unit plus two angle conversions). The self-check exercises the same public resolver, input parser, status gate, evaluator, and JSON envelopes as `formula run`. The remaining 140 registry rows and the 1,000+ source-accounting backlog are outside this release-candidate scope.

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
cargo run -p xtask -- verify-checksums
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify-release-manifest
cargo run -p xtask -- verify-release-identity
cargo run -p xtask -- verify-generated
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
- the manifest and CLI report semantic version `0.1.0-alpha.1` and release tier `research_software_alpha` (`Research Software Alpha`);
- the release slice remains `implementation_verified` while the 140-formula complement remains `research_required`;
- `formula list --executable --json` returns exactly the twelve manifest-selected formulas;
- no operational-readiness, certification, full-inventory, external-parity, or safety claim is added.

Passing this gate authorizes research-software-alpha testing only. The `beta1-concept` label remains a historical compatibility name, not the current runtime identity. Publication, signing, tagging, or broader distribution requires a separate release decision.
