# AeroCodex Nomenclature Protocol

**Protocol:** `ACX-NOM-001`
**Version:** `0.2.0-draft`
**Purpose:** Prevent symbol, acronym, term, unit, frame, source-term, and Rust identifier ambiguity across AeroCodex math, schemas, ingestion, documentation, AI workflows, and Rust code.

This package is integrated in the main AeroCodex repository under `nomenclature/` and should be refined through normal pull requests.

## Core rule

> In math, declare every symbol. In Rust, spell out every meaning. In schemas, use only canonical terms. In ingestion, preserve source wording but map it explicitly. For AI, provide compact terminology context rather than a giant glossary dump.

A spelling, glyph, or acronym such as `n`, `N`, `T`, `x`, `station`, `tail_number`, `RCS`, `CDR`, `AC`, `id`, or `leg` is only a **surface form**. It has no approved meaning until resolved against a namespace, scope, domain, unit, frame, source authority, and provenance record.

## Included public files

```text
.
├── README.md
├── CHANGELOG.md
├── ADOPTION_PLAN.md
├── REFERENCES.md
├── docs/
├── examples/
├── registry/
├── schemas/
└── templates/
```

The public repository intentionally does not track deployment helper scripts or generated acronym-inventory outputs. Public CI should use the Rust `xtask` gates from the repository root.

## Public checks

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
```

## Working with terminology

1. Add or edit records in `registry/`.
2. Use the templates in `templates/` for durable new symbols, acronyms, aliases, and waivers.
3. Update docs and examples when a registry change affects public usage.
4. Keep candidate external terms marked as candidate/source-scoped until reviewed.
5. Do not use unreviewed acronym expansions to imply certification, operational readiness, source parity, or safety approval.

Maintainers may use private tooling outside the public repository to generate compact terminology packs or acronym inventories, but those generated artifacts are not part of the public tree.
