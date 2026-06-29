# AeroCodex Acronym/Terminology Integration Summary

This package upgrades the original nomenclature protocol from `0.1.0-draft` to `0.2.0-draft` by integrating aerospace/space acronym handling and AI-facing terminology context.

## Public content retained

- `registry/acronyms.yaml` — scoped acronym records with candidate status, sources, first-use policy, collision groups, disambiguation signals, and AI summaries.
- `registry/terminology_sources.yaml` — source registry for project seed vocabulary and external source families.
- `schemas/acronym.schema.json` and `schemas/terminology_source.schema.json`.
- `docs/acronym_terminology_protocol.md` — formal acronym protocol.
- `docs/ai_terminology_integration.md` — AI pack architecture and runtime flow.
- `templates/ACRONYM_PROPOSAL.md` and `templates/AI_TERMINOLOGY_PACK.md`.
- `examples/specs/acronym_resolution_demo.md` and example terminology-pack material.

## Public tooling stance

Deployment helper scripts and generated inventory outputs are no longer tracked in the public repository. Public verification is through the Rust workspace and `xtask` gates:

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
```

## Current review status

Known demonstration collisions such as `CDR`, `RCS`, `AC`, and reused math surface form `n` remain registry examples. New durable uses should be registered, waived, or explicitly reviewed.
