# Changelog

## 0.2.0-draft

Integrated acronym and external terminology handling into the AeroCodex nomenclature protocol.

Added:

- `registry/acronyms.yaml` with candidate aerospace acronym records and collision examples.
- `registry/terminology_sources.yaml` for NASA, ECSS, FAA, CCSDS, DoD, AIAA, and internal seed sources.
- `schemas/acronym.schema.json` and `schemas/terminology_source.schema.json`.
- `docs/acronym_terminology_protocol.md`.
- `docs/ai_terminology_integration.md`.
- `templates/ACRONYM_PROPOSAL.md`.
- `templates/AI_TERMINOLOGY_PACK.md`.
- Public repository now retains registry data, schemas, templates, examples, and docs; generated terminology-pack tooling is maintained outside the public tree.
- Example acronym-resolution spec, generated example AI pack, and retrieval-friendly `generated/terminology/index.jsonl`.

Changed:

- Expanded `ACX-NOM-001` to include acronym namespaces, source precedence, AI pack rules, and acronym lint codes.
- Expanded linting to validate acronym and terminology-source registries.
- Updated CI example to export a terminology index artifact.
- Updated PR checklist and adoption plan for acronym review and AI integration.

## 0.1.0-draft

Initial AeroCodex nomenclature starter kit.

Included:

- Formal protocol `ACX-NOM-001`.
- Math symbol declaration protocol.
- Rust identifier profile.
- Source ingestion protocol.
- Registry seed files for concepts, aliases, symbols, units, frames, bridges, and waivers.
- JSON schema seed files.
- CI rule catalog.
- Lightweight lint scaffold.
- PR and proposal templates.
- Example trajectory fitting specification and Rust examples.
