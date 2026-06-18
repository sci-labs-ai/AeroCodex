# Stage 5 Agent Operating Rules

These rules govern Stage 5 deployment-agent work after the Chunk 0 baseline freeze. They are intentionally conservative because Stage 5 contains parallel handoffs, governed inventory changes, source-boundary risks, and BioSim/Orekit deferred material.

## Canonical source of truth

- GitHub `main` is canonical.
- Begin every modifying chunk with `git fetch --prune origin`, `git checkout main`, and `git pull --ff-only origin main`.
- Local `main`, cached `origin/main`, and remote `refs/heads/main` must match before modification.
- Require a completed successful Rust GitHub Actions run for the exact main commit before beginning and for the exact deployed commit before closeout.
- Never reset, rebase, force-push, or rewrite history to recover an advisory commit.

## Branch and chunk discipline

- Use no long-lived branch forest.
- Use at most one short-lived local deployment branch for a modifying chunk, then fast-forward merge to `main`, push `main`, and delete the local branch.
- One modifying chunk per coordinator prompt.
- Do not merge all parallel handoffs together.
- Do not bundle iteration fallback with a modifying prompt; stop and report blockers instead.
- Do not deploy based solely on `git apply --check`; preflight is necessary but not sufficient.

## Outside-repository extraction and evidence

- All handoff extraction, preflight copies, logs, scripts, reports, and evidence must stay outside the repository.
- Do not commit ZIPs, patch/diff files, extracted temporary files, evidence/report files, generated `target/` output, credentials, tokens, `.env` files, local absolute paths, or deployment workspaces.
- Do not extract nested external source archives unless a later prompt explicitly authorizes that exact source-boundary action.
- Do not add optional data-governance registry entries for Stage 5 ZIPs unless a later chunk explicitly authorizes that governed change.

## Source boundaries

- M07 remains quarantined source material. Do not import raw M07 source, Scilab output, generated code, fixtures, or release-candidate runtime into public APIs.
- GPL BioSim and BioSim-RS materials must remain separated from the dual MIT/Apache AeroCodex core unless a deliberate future licensing path authorizes otherwise.
- Orekit is a reference oracle and architecture guide only. Do not copy Java source, class files, or class-hierarchy structure.
- BioSim/Orekit work uses `deferred_pending_deep_handoff` until explicit user authorization selects a bounded deep handoff or safe carve-out.


## Final deep BioSim/Orekit handoff arrival rule

When final deep BioSim/Orekit handoffs arrive:

- Pause the deployment queue after the active chunk closes.
- Inventory and hash the new bundles outside the repository.
- Compare them against Session E/F and legacy BioSim/Orekit drafts before changing queue order.
- Classify superseded material only with explicit evidence.
- Deploy BioSim/Orekit through serial subpatches only.
- Never deploy old B1a/O1a merely because they are smaller than the final v3 material.

## Governed-count and validation rules

- Never hardcode governed counts to satisfy a verifier.
- Before a modifying chunk, run and record the governed baseline counts from live `main`.
- After the change, recompute all governed counts and report before/after/delta.
- If a docs-only chunk changes any governed count, stop before commit, identify the cause, restore the intended scope, and rerun affected gates.
- Conservative validation status is normally `research_required`; do not imply certification, mission readiness, habitat safety, medical approval, or operational approval.

## Required gates

At minimum, a modifying chunk must run the focused gate for its touched surface plus:

```bash
git diff --check
cargo run -p xtask -- verify equation-inventory
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify data-registry
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- verify status-vocabulary
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps
python nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature
python nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json
python nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl
git diff --exit-code nomenclature/generated/terminology/index.jsonl
```

If bare `python` is unavailable, record the exact command as unavailable and rerun through a documented outside-repository `python` to `python3` shim. If Cargo creates an untracked root `Cargo.lock`, hash and record it outside the repository, remove it, and confirm the root lockfile is absent.

## Closeout evidence

Every closeout must report:

- base main commit and deployed main commit;
- commit hash and message;
- exact changed-file list;
- local/remote branch cleanup status;
- local gate results, including unavailable commands and fallbacks;
- before/after/delta governed counts;
- source-boundary and forbidden-material scan result;
- exact remote CI run ID, job ID if available, status, conclusion, URL, and head commit;
- outside-repository report and hash-manifest paths.

## Local-agent boundary

Deployment tasks must not modify local Winston/Hermes skills, profiles, prompts, or agent configuration. If agent behavior needs improvement, record it outside the modifying deployment flow and handle it only after the repository task is closed.
