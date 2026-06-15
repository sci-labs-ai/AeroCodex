# AeroCodex Acronym/Terminology Integration Summary

This package upgrades the original nomenclature protocol from `0.1.0-draft` to `0.2.0-draft` by integrating aerospace/space acronym handling and AI-facing terminology context.

## Added

- `registry/acronyms.yaml` — scoped acronym records with candidate status, sources, first-use policy, collision groups, disambiguation signals, and AI summaries.
- `registry/terminology_sources.yaml` — source registry for project seed vocabulary, NASA, ECSS, FAA, CCSDS, DoD, and AIAA source families.
- `schemas/acronym.schema.json` and `schemas/terminology_source.schema.json`.
- `docs/acronym_terminology_protocol.md` — formal acronym protocol.
- `docs/ai_terminology_integration.md` — AI pack architecture and runtime flow.
- `tooling/aerocodex_terminology.py` — lookup, AI pack generation, and JSONL export.
- `generated/terminology/index.jsonl` — retrieval-friendly seed index.
- `templates/ACRONYM_PROPOSAL.md` and `templates/AI_TERMINOLOGY_PACK.md`.
- `examples/specs/acronym_resolution_demo.md` and generated example terminology pack.

## Integrated

- `docs/ACX-NOM-001.md` now includes acronym namespaces, acronym invariants, source precedence, first-use rules, ambiguity behavior, and AI terminology packs.
- `README.md`, `ADOPTION_PLAN.md`, `REFERENCES.md`, `CHANGELOG.md`, `ci/nom-rules.yaml`, `ci/github-actions-example.yml`, and `templates/PR_CHECKLIST.md` were updated.
- Existing canonical term and alias registries were extended with candidate acronym concepts and ambiguous acronym aliases.
- Rust identifier guidance now includes acronym collision guidance.
- Source ingestion guidance now covers acronym source-token preservation and ambiguous acronym rejection.

## Tooling

Run validation:

```bash
python tooling/aerocodex_nom_lint.py --root .
```

Run optional unknown-acronym discovery:

```bash
python tooling/aerocodex_nom_lint.py --root . --scan-acronyms
```

Look up a token:

```bash
python tooling/aerocodex_terminology.py --root . lookup RCS
```

Generate an AI pack:

```bash
python tooling/aerocodex_terminology.py --root . pack \
  --text-file examples/specs/acronym_resolution_demo.md \
  --domain spacecraft \
  --domain systems_engineering \
  --domain aviation
```

Export retrieval index:

```bash
python tooling/aerocodex_terminology.py --root . export-jsonl --output generated/terminology/index.jsonl
```

## Current lint status

The default lint run exits successfully. It intentionally emits warnings for known demonstration collisions:

- `CDR`
- `RCS`
- `AC`
- reused math surface form `n` across disjoint scopes

These warnings are expected in the starter kit because they demonstrate safe collision handling.
