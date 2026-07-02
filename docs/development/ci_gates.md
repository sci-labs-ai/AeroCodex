# CI and local verification gates

RR-003 defines the minimum first-milestone CI/local gates for AeroCodex as professional research/preliminary-design software. RR-012 wires the lightweight GitHub Actions coverage for those gates and for the equation-batch infrastructure that exists after RR-011. These gates support the public wording posture that AeroCodex is not certified operational aerospace software and is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

Run the baseline gates from the repository root:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo doc --no-deps
cargo run -p xtask -- verify --all
```

## Blocking GitHub Actions gate

`.github/workflows/ci.yml` runs on `pull_request` and on `push` to `main` using Rust stable on Linux (`ubuntu-latest`). It keeps the repository cargo-first and runs the baseline gates plus lightweight equation-batch inventory/status checks:

```bash
cargo fmt --check
cargo check --workspace --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo doc --no-deps
cargo run -p xtask -- verify --all
cargo run -p xtask -- equation-batch plan --all-manifests --json > /tmp/equation_batch_plan.json
python3 -m json.tool /tmp/equation_batch_plan.json >/dev/null
cargo run -p xtask -- equation-batch report --all-manifests --out generated/equation_batch_status_report.json --check
cargo run -p xtask -- dependency-policy
cargo run -p aero-codex-cli -- self-check --json
```

The `python3 -m json.tool` calls are JSON syntax checks using the standard Python available on GitHub's Linux runner; they do not introduce a project Python, Jupyter, web, API, or external-service requirement.

## Gate intent

- `cargo fmt --check` verifies Rust formatting without rewriting files.
- `cargo clippy --all-targets --all-features -- -D warnings` treats Clippy warnings as failures for all configured targets and features.
- `cargo test --all` runs the workspace test suite through Cargo.
- `cargo doc --no-deps` builds local documentation without third-party dependency docs.
- `cargo run -p xtask -- verify --all` runs the repository governance and verification checks.
- `cargo run -p xtask -- equation-batch plan --all-manifests --json` checks that every current equation-batch manifest is still readable by the planning/reporting infrastructure and emits parseable JSON.
- `cargo run -p xtask -- equation-batch report --all-manifests --out generated/equation_batch_status_report.json --check` verifies that the checked-in equation-batch status report remains deterministic and current.

## Equation-batch verify-all diagnostic gate

RR-011 adds a CI-friendly command contract for checking every existing equation-batch manifest without changing validation status or committed generated artifacts:

```bash
cargo run -p xtask -- equation-batch verify \
  --all-manifests \
  --output-dir /tmp/acx-equation-batch-probes \
  --json \
  --check
```

RR-012 wires this command in `.github/workflows/research-readiness.yml` as a separate Linux Rust-stable diagnostic job that runs on `pull_request` and `push` to `main`. The job is explicitly named `equation-batch verify-all (diagnostic, non-required)` and uses GitHub Actions `continue-on-error: true`, so a nonzero verify-all result is not hidden as a passing blocking gate.

The diagnostic job runs:

```bash
rm -rf /tmp/acx-equation-batch-probes
cargo run -p xtask -- equation-batch verify \
  --all-manifests \
  --output-dir /tmp/acx-equation-batch-probes \
  --json \
  --check \
  > /tmp/equation_batch_verify_all.json
python3 -m json.tool /tmp/equation_batch_verify_all.json >/dev/null
```

Expected runtime depends on Cargo cache state because the command generates one temporary probe crate per `equation-batches/*.tsv` manifest and runs `cargo test` inside each generated probe. Output is deterministic in manifest-path order and returns nonzero if any manifest parse, generation, compile, cargo, or generated-test check fails. The probe output directory must be outside the repository and is safe to refresh only when it is empty or marked as AeroCodex-generated probe output.

This is not a formula status-promotion gate by itself. It does not edit equation-batch TSVs, validation cards, validation status files, generated registries, product CLI code, runtime formula code, M07 materials, or formula status. It does not claim that all manifests are certified, flight-ready, mission-ready, operational, or validated. Maintainers can make the verify-all job blocking only after the command is expected to pass for every current manifest and the cost is acceptable for every PR.

## Future placeholders

- Future registry generation/check placeholder: when deterministic formula-registry generation lands, CI should verify the generated registry is reproducible and checked in only from governed inputs.
- Future formula status/gating check placeholder: when formula execution and status gates expand, CI should verify that normal execution remains blocked unless the formula status and mode permit it.
