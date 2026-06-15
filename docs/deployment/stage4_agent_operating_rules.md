# Stage 4 Agent Operating Rules

These rules govern Stage 4 deployment chunks for AeroCodex. They are intended for human operators and deployment agents.

## Non-negotiable caveat

AeroCodex is research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved. No Stage 4 agent may weaken this caveat.

## Canonical repository rule

- GitHub `main` is the canonical source of truth.
- There must be one current, testable `main`; do not leave competing integration branches for the user to reconcile.
- A short-lived branch may be used for a pull request, but it must be merged to `main` after checks pass and then deleted.
- The agent is responsible for the whole update loop: sync, inspect local source material, compare against current `main`, make minimal edits, run checks, commit, merge or fast-forward, push, and verify the final remote state.

## Source-boundary rules

- External archives are source material, not automatic public API.
- Do not bulk merge the M07 Scilab-to-Rust workspace into public APIs.
- Do not overwrite `crates/aero-codex-astrodynamics` with the M07 `aerocodex-astrodynamics` crate.
- Treat M07 as quarantined formula-vault candidate material until Rust continuous integration, Scilab equivalence, and SGP4 certification pass.
- Treat BioSim-RS as first-class but license-boundaried. Do not mix GPL-bound source or translated Java implementation details into the dual MIT/Apache core unless the licensing path is deliberately changed.
- Use Orekit as a reference oracle and architecture guide only. Do not copy its Java class hierarchy class-for-class.

## Chunk discipline

1. Read the active prompt and Stage 4 planning docs before editing.
2. Inspect archive inventories and hashes before taking content from any external bundle.
3. Copy only missing or newer planning/governance/docs content that belongs on `main`; never blindly overwrite current repository files.
4. Keep Chunk 0 documentation-only: no feature code, no new crates, no external archive import unless an archive item is already an intended repository doc.
5. Keep changes small, reviewable, and traceable to the source inventory.
6. Preserve Cargo package versions unless a release policy explicitly authorizes a semantic-version change.

## Required local checks for Chunk 0

Run these from the repository root and record exact pass, fail, or unavailable status:

```bash
git status --short
git diff --check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
python nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature
python nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json
python nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl
git diff --exit-code nomenclature/generated/terminology/index.jsonl
cargo doc --workspace --all-features --no-deps
```

If Rust, Cargo, Python, or another required tool is unavailable, report the exact unavailable check. Do not mark unavailable checks as passed.

## Merge and reporting duties

The final report for each Stage 4 chunk must include:

- files changed;
- checks run with exact status;
- branch name if used;
- commit hash;
- pushed `main` status;
- deleted-branch status if a branch was used;
- blocked or intentionally deferred work;
- the next recommended chunk.
