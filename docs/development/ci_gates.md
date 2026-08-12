# CI and local verification gates

RR-003 defines the minimum first-milestone CI/local gates for AeroCodex as professional research/preliminary-design software. RR-012 wires the lightweight GitHub Actions coverage for those gates and for the equation-batch infrastructure that exists after RR-011. These gates support the public wording posture that AeroCodex is not certified operational aerospace software and is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

Run the baseline gates from the repository root:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo doc --no-deps
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify-release-manifest
cargo run -p xtask -- verify-checksums
cargo run -p xtask -- verify-generated
cargo run -p xtask -- formula-registry check
```

## Blocking GitHub Actions gate

`.github/workflows/ci.yml` runs on `pull_request` and on `push` to `main` using Rust stable on Linux (`ubuntu-latest`). Checkout uses `fetch-depth: 0` because the release-manifest gate must resolve the pinned object and prove that it is a commit ancestor of `HEAD`. It keeps the repository cargo-first and runs the baseline gates plus lightweight equation-batch inventory/status checks:

```bash
cargo fmt --all -- --check
bash scripts/tests/agent_pr_check_cargo_lock.sh
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify-release-manifest
cargo run -p xtask -- verify-checksums
cargo run -p xtask -- equation-batch plan --all-manifests --json > /tmp/equation_batch_plan.json
python3 -m json.tool /tmp/equation_batch_plan.json >/dev/null
cargo run -p xtask -- verify-generated
cargo run -p xtask -- dependency-policy
cargo run -p aero-codex-cli -- self-check --json
```

The `python3 -m json.tool` calls are JSON syntax checks using the standard Python available on GitHub's Linux runner; they do not introduce a project Python, Jupyter, web, API, or external-service requirement.

## Gate intent

- `cargo fmt --check` verifies Rust formatting without rewriting files.
- `bash scripts/tests/agent_pr_check_cargo_lock.sh` exercises exact transient-lock file identity and replacement preservation on Ubuntu, including the native symbolic-link substitution case that may be unavailable on Windows hosts.
- `cargo clippy --all-targets --all-features -- -D warnings` treats Clippy warnings as failures for all configured targets and features.
- `cargo test --all` runs the workspace test suite through Cargo.
- `cargo doc --no-deps` builds local documentation without third-party dependency docs.
- `cargo run -p xtask -- verify --all` runs the repository governance and verification checks.
- `cargo run -p xtask -- verify-release-manifest` parses the release manifest, requires its metadata and references, rejects duplicate IDs, duplicate runtime symbols, and unsafe status/policy combinations, and requires exact formula-ID/runtime-symbol equality with the structured CLI dispatch metadata and the governed registry. Every evidence reference must name an existing regular file through a normalized repository-relative path; absolute paths, traversal, final symlinks, and canonical symlink escapes fail explicitly. This containment check is filesystem-based and still works in a source archive. The separate pinned-base check requires `.git`, resolves the base to a commit, and proves it is an ancestor of `HEAD`.
- `cargo run -p xtask -- verify-checksums` requires exact set equality between the manifest and the documented governed files. Native governed paths must have one valid UTF-8 representation on every supported platform; discovery fails before hashing or set comparison and reports invalid Unix bytes or Windows UTF-16 deterministically, so no lossy omission or alias is possible. It rejects malformed hashes, unsafe/noncanonical/duplicate/excluded paths, missing or changed content, and both missing and extra set members. Digests include a versioned object-kind and payload-length frame. CRLF normalization applies only to regular files on the explicit text-format allowlist; other regular content is byte-exact. Symbolic links use unresolved raw Unix target bytes or a lossless deterministic Windows UTF-16 representation and are never followed or slash-normalized. `cargo run -p xtask -- generate-checksums` renders a candidate through the same discovery policy without installing it.
- `cargo run -p xtask -- equation-batch plan --all-manifests --json` checks that every current equation-batch manifest is still readable by the planning/reporting infrastructure and emits parseable JSON.
- `cargo run -p xtask -- verify-generated` runs the equation-batch status-report and formula-registry read-only checks, requires every governed output to be a regular file, and compares the exact Git-aware state before and after. Tracked staged and unstaged content, executable modes, and all untracked paths are covered independently of `.gitignore`, `.git/info/exclude`, and `core.excludesFile`; files beneath the generator-owned `generated/` tree never receive transient exclusions. Raw symbolic-link identities detect creation, deletion, retargeting, and link/file replacement. Only the documented root `Cargo.lock`, `.git/`, `target/`, `.DS_Store`, `*.tmp`, and `*.rs.bk` paths outside generator-owned trees are excluded. A dirty baseline is permitted only when the action produces zero state delta. This is a software consistency gate, not formula validation, status promotion, certification, or formula execution.

The release-manifest pinned-base/ancestry check and the generated-artifact gate require a Git worktree with sufficient object history. Release evidence-path containment itself remains valid without `.git`; a complete source archive still cannot pass the Git-dependent portions and needs a separately designed archive-attestation workflow.

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

## Additional gate notes

- Registry consistency gate: `cargo run -p xtask -- formula-registry check` verifies the generated formula registry artifacts are reproducible and checked in only from governed inputs.
- Future formula status/gating check placeholder: when formula execution and status gates expand, CI should verify that normal execution remains blocked unless the formula status and mode permit it.
