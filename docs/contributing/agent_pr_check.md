# Agent PR check

RR-054 adds one local command for coding agents to run before handoff:

```bash
bash scripts/agent_pr_check.sh
```

The script is a local handoff gate for AeroCodex software consistency. It does not replace maintainer review, GitHub Actions, or the pull-request review process. Passing it is not formula validation, status promotion, certification, formula execution approval, readiness approval, or regulatory approval.

## What the script runs

Run it from any directory inside the repository. The script resolves the repository root with `git rev-parse --show-toplevel`, changes to that root, prints section headers, and exits nonzero on a blocking failure.

The current gate mirrors the repository's baseline CI/local expectations:

```bash
git diff --check
cargo fmt --check
cargo check --workspace --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo doc --no-deps
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify-release-manifest
cargo run -p xtask -- verify-checksums
cargo run -p xtask -- verify-generated
cargo run -p xtask -- equation-batch plan --all-manifests --json
python3 -m json.tool /tmp/equation_batch_plan.json >/dev/null
cargo run -p xtask -- equation-batch report --all-manifests --out generated/equation_batch_status_report.json --check
cargo run -p xtask -- formula-registry check
cargo run -p xtask -- dependency-policy
cargo run -p aero-codex-cli -- self-check --json
```

It also runs a local forbidden-claim grep/review gate over existing repository documentation, scripts, workflows, Rust tooling, crates, tests, formula schemas, and schemas. The review accepts negative caveats, guardrail/prohibition text, and other non-claim references, but fails on positive or ambiguous public wording that could imply operational approval or certification.

The known `equation-batch verify-all (diagnostic, non-required)` job remains outside this script's required gate. Maintainers may run that diagnostic separately, but RR-054 does not make it blocking.

## Local-only behavior

`scripts/agent_pr_check.sh` intentionally has no network, GitHub, authentication, pull-request, merge, push, branch-deletion, or CI-polling behavior. It does not run `git fetch`, `gh`, `curl`, or remote probes. It does not install, configure, require, or invoke pre-commit hooks.

The cargo commands are the repository's normal local build/test/doc/tooling checks. The script itself adds no external service dependency.

## Cargo.lock handling

This repository normally does not keep a root `Cargo.lock` in the RR-054 baseline. The handoff script classifies the path before any check and runs a dedicated ownership-guard harness. Its cleanup policy is fail-closed:

- a tracked, regular `Cargo.lock` is hashed, preserved, and required to remain byte-identical;
- a preexisting untracked file, symlink, missing tracked path, or other ambiguous state is preserved and stops the run before Cargo commands;
- when the path is absent and covered by the repository ignore policy, the script atomically creates a minimal transient lockfile and thereby establishes ownership before Cargo can update it;
- after every step and again in the exit trap, an owned lock must remain a regular, ignored, untracked path; if any condition changes, cleanup preserves it and fails;
- only a path atomically created and continuously owned by this invocation is removed. Failure and interruption paths run the same guarded cleanup.

Successful removal records the final SHA-256 and size and prints:

```text
removed_owned_transient=yes sha256=<digest> size=<bytes>
```

The focused harness at `scripts/tests/agent_pr_check_cargo_lock.sh` covers absent/owned, preexisting untracked, clean tracked, preexisting dirty tracked, tracked content modified during a run, command-failure, interruption, and repository paths containing spaces. Agents should not include a transient root `Cargo.lock` in an RR-054 handoff unless maintainers explicitly change repository policy.

## Shellcheck

Shellcheck is useful for reviewing this script when available, but it is not a hard dependency and is not installed by RR-054. Agents may run:

```bash
shellcheck scripts/agent_pr_check.sh scripts/lib/cargo_lock_guard.sh scripts/tests/agent_pr_check_cargo_lock.sh
```

If shellcheck is absent, record the skip in the handoff notes, for example:

```text
shellcheck_unavailable=SKIP
```

Do not fail an otherwise valid local handoff only because shellcheck is not installed.

## How to report output in PR notes

For a handoff or future PR note, paste or quote the command shape and the final PASS marker:

```text
Local handoff gate:
bash scripts/agent_pr_check.sh

Result:
agent_pr_check=PASS
```

If space is limited, include the section headers for any failed step plus the exact error output. If the forbidden-claim review reports matches, state whether they are negative caveats, guardrail/prohibition text, or another non-claim reference.

## Scope and non-claims

This gate is governance and handoff tooling only. It does not certify formulas, execute formulas for approval, promote validation statuses, approve mission or flight use, approve habitat or life-support use, approve regulated use, or make lower-status formulas executable. It does not replace CI, maintainer review, domain review, traceability review, or status-promotion evidence.
