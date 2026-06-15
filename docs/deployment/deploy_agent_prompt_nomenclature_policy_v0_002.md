# Agent prompt: merge AeroCodex nomenclature/acronym policy into GitHub main

You are operating on the AeroCodex GitHub repository. Your goal is to make the nomenclature/acronym policy part of the single canonical `main` branch, not a side package.

## Goal

Merge the provided `AeroCodex-main/` tree into the GitHub repository so the main repo has:

- a governed `nomenclature/` directory;
- acronym, terminology-source, concept, alias, symbol, unit, frame, bridge, and waiver registries;
- a repository-wide adoption inventory and baseline for existing acronym-like tokens;
- an AI-facing terminology JSONL index;
- CI enforcement so future additions cannot silently introduce unregistered acronym-like tokens;
- README/docs/PR-template updates so contributors use the policy.

Do not change the Cargo workspace package version. The human roadmap phase remains `Phase 0.001`, and Cargo package versions remain `0.0.1`.

## Merge procedure

1. Start from the current GitHub `main` branch.
2. Create a branch named `nomenclature-policy-v0.2`.
3. Copy the provided `AeroCodex-main/` overlay into the repository root. Preserve current GitHub changes if they are newer than this package; resolve conflicts in favor of the latest project content while keeping all nomenclature policy files and CI hooks.
4. Confirm these paths exist after merge:
   - `nomenclature/docs/ACX-NOM-001.md`
   - `nomenclature/registry/acronyms.yaml`
   - `nomenclature/registry/terminology_sources.yaml`
   - `nomenclature/tooling/aerocodex_nom_lint.py`
   - `nomenclature/tooling/aerocodex_terminology.py`
   - `nomenclature/tooling/aerocodex_acronym_inventory.py`
   - `nomenclature/generated/current_repo_acronym_baseline.json`
   - `nomenclature/generated/terminology/index.jsonl`
   - `docs/nomenclature_policy.md`
   - `.github/PULL_REQUEST_TEMPLATE.md`
5. Keep candidate acronym records as `candidate` unless a human source review explicitly promotes them to `approved`.
6. Do not bulk-approve external acronym lists. Source registries may identify NASA, ECSS, FAA, CCSDS, DoD, AIAA, and AeroCodex seed families, but imported terms must remain candidate/source-scoped until reviewed.

## Required checks

Run these from the repository root:

```bash
python -m pip install pyyaml
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

Expected nomenclature-lint warnings are acceptable at adoption time:

- `CDR` has multiple candidate meanings.
- `RCS` has multiple candidate meanings.
- `AC` has multiple candidate meanings.
- math symbol `n` is reused across disjoint declared scopes.

Any `ERROR` from the nomenclature linter must be fixed before merge.

## Existing-system capture

The provided repository has already been scanned. The adoption snapshot lives in:

- `nomenclature/generated/current_repo_acronym_inventory.md`
- `nomenclature/generated/current_repo_acronym_inventory.csv`
- `nomenclature/generated/current_repo_acronym_inventory.json`
- `nomenclature/generated/current_repo_acronym_baseline.json`

The baseline is not an approval list. It only records tokens already present at policy adoption so future changes can be gated. Do not regenerate or expand the baseline just to bypass CI. If a new token is legitimate, add a registry record or waiver first.

## Future-addition rule

For every new durable doc, Rust module, validation card, source-registry entry, data manifest, or generated artifact:

1. Expand acronyms at first durable use.
2. Add a record to `nomenclature/registry/acronyms.yaml` for new acronym meanings.
3. Add a source to `nomenclature/registry/terminology_sources.yaml` if no existing source applies.
4. Add `collision_group` and `disambiguation.signals` for ambiguous tokens.
5. Add waivers only when a token is intentionally not an acronym or must preserve source notation.
6. Regenerate `nomenclature/generated/terminology/index.jsonl` whenever the registries change.
7. Run the acronym guard before opening the PR.

## AI integration requirement

Agents working on AeroCodex should not paste the whole glossary into prompts. They should generate a task-local terminology pack, for example:

```bash
python nomenclature/tooling/aerocodex_terminology.py --root nomenclature pack \
  --text-file docs/phase_0_001/final_microtasks_001_020_report.md \
  --domain life_support \
  --domain source_traceability \
  --output nomenclature/generated/terminology/final_report_pack.md
```

Use the generated pack as scoped context. If an acronym is ambiguous, the agent must state the ambiguity unless local evidence resolves it.

## Commit and PR

Suggested commit message:

```text
Add governed nomenclature and acronym policy
```

Suggested PR title:

```text
Add AeroCodex nomenclature/acronym registry and CI guard
```

Suggested PR body:

```markdown
## Summary

Adds ACX-NOM-001 as a governed repository policy, integrates acronym/source registries, captures the current repository acronym inventory and adoption baseline, exports an AI-facing terminology JSONL index, and adds CI/PR checks for future terminology additions.

## Notes

- Candidate acronym records are not approvals.
- Existing repository tokens are captured in the adoption baseline to prevent future silent drift.
- Expected adoption warnings: ambiguous `CDR`, `RCS`, `AC`, and scoped reuse of math symbol `n`.

## Checks

- [ ] cargo fmt
- [ ] cargo clippy
- [ ] cargo test
- [ ] cargo run -p xtask -- verify --all
- [ ] cargo run -p xtask -- dependency-policy
- [ ] nomenclature lint
- [ ] acronym guard
- [ ] terminology index regenerated and clean
```
