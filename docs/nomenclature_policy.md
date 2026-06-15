# AeroCodex Nomenclature and Acronym Policy

AeroCodex treats terminology as a governed repository artifact. The canonical protocol lives in [`../nomenclature/docs/ACX-NOM-001.md`](../nomenclature/docs/ACX-NOM-001.md), with registries under [`../nomenclature/registry/`](../nomenclature/registry/), templates under [`../nomenclature/templates/`](../nomenclature/templates/), and tooling under [`../nomenclature/tooling/`](../nomenclature/tooling/).

## What this adds to the system

The policy introduces a scoped terminology registry, an acronym registry, a terminology-source registry, symbol/alias/bridge registries, AI terminology-pack generation, and a repository-wide acronym guard. The goal is to answer: what does this token mean, in this document, under this authority, for this project?

## Existing-system adoption snapshot

The current repository has been scanned and captured in:

- [`../nomenclature/generated/current_repo_acronym_inventory.md`](../nomenclature/generated/current_repo_acronym_inventory.md)
- [`../nomenclature/generated/current_repo_acronym_inventory.csv`](../nomenclature/generated/current_repo_acronym_inventory.csv)
- [`../nomenclature/generated/current_repo_acronym_inventory.json`](../nomenclature/generated/current_repo_acronym_inventory.json)
- [`../nomenclature/generated/current_repo_acronym_baseline.json`](../nomenclature/generated/current_repo_acronym_baseline.json)

The baseline is not an approval list. It is the adoption snapshot that prevents future changes from silently introducing new unregistered acronym-like tokens.

## Required workflow for new content

When adding docs, Rust code, validation cards, source-registry entries, or data manifests:

1. Expand new acronyms at first durable use.
2. Add governed acronym records to `nomenclature/registry/acronyms.yaml`.
3. Add or reuse a source in `nomenclature/registry/terminology_sources.yaml`.
4. Add collision metadata when a token has multiple plausible meanings.
5. Regenerate the AI retrieval index when registries change.
6. Run the nomenclature lint and acronym guard before merge.

## Local commands

```bash
python -m pip install pyyaml
python nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature
python nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json
python nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl
```

Generate an AI terminology pack for a document:

```bash
python nomenclature/tooling/aerocodex_terminology.py --root nomenclature pack --text-file docs/phase_0_001/final_microtasks_001_020_report.md --domain life_support --output nomenclature/generated/terminology/final_report_pack.md
```

Look up a token:

```bash
python nomenclature/tooling/aerocodex_terminology.py --root nomenclature lookup BLSS
```
