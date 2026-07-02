# CI and local verification gates

RR-003 defines the minimum first-milestone CI/local gates for AeroCodex as professional research/preliminary-design software. These gates support the public wording posture that AeroCodex is not certified operational aerospace software and is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

Run the baseline gates from the repository root:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo doc --no-deps
cargo run -p xtask -- verify --all
```

## Gate intent

- `cargo fmt --check` verifies Rust formatting without rewriting files.
- `cargo clippy --all-targets --all-features -- -D warnings` treats Clippy warnings as failures for all configured targets and features.
- `cargo test --all` runs the workspace test suite through Cargo.
- `cargo doc --no-deps` builds local documentation without third-party dependency docs.
- `cargo run -p xtask -- verify --all` runs the repository governance and verification checks.

## Equation-batch verify-all probe gate

RR-011 adds a local/CI-friendly command contract for checking every existing equation-batch manifest without changing validation status or committed generated artifacts:

```bash
cargo run -p xtask -- equation-batch verify \
  --all-manifests \
  --output-dir /tmp/acx-equation-batch-probes \
  --json \
  --check
```

Expected runtime depends on Cargo cache state because the command generates one temporary probe crate per `equation-batches/*.tsv` manifest and runs `cargo test` inside each generated probe. Output is deterministic in manifest-path order and returns nonzero if any manifest parse, generation, compile, cargo, or generated-test check fails. The probe output directory must be outside the repository and is safe to refresh only when it is empty or marked as AeroCodex-generated probe output.

This is not a formula status-promotion gate by itself. It does not edit equation-batch TSVs, validation cards, validation status files, generated registries, product CLI code, runtime formula code, M07 materials, or GitHub Actions workflow files.

## Future placeholders

- Future registry generation/check placeholder: when deterministic formula-registry generation lands, CI should verify the generated registry is reproducible and checked in only from governed inputs.
- Future formula status/gating check placeholder: when formula execution and status gates expand, CI should verify that normal execution remains blocked unless the formula status and mode permit it.

## Current GitHub Actions note

The repository already has a GitHub Actions Rust workflow. It uses Rust stable and may run workspace-expanded equivalents or additional checks around the minimum gates above. RR-003 does not build a new workflow and does not change the existing workflow file.
