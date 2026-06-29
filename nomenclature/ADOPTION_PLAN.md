# AeroCodex Nomenclature Adoption Plan

## Phase 0 — Immediate human rule

Adopt this rule immediately for all new work:

> No symbol, identifier, abbreviation, acronym, glyph, or source term carries meaning by implication alone.

Concrete enforcement:

- Every durable equation gets a symbol table.
- Every implemented equation gets a Rust–math bridge.
- Every schema field must use a canonical term.
- Every source-original term must be preserved separately from its canonical mapping.
- Every acronym either expands at first durable use or resolves through a document-level terminology pack.
- Durable Rust code avoids ambiguous single-letter variables and unexplained acronym identifiers.

## Phase 1 — Registry bootstrapping

Create and maintain these registry files:

```text
registry/concepts.yaml
registry/aliases.yaml
registry/acronyms.yaml
registry/terminology_sources.yaml
registry/symbols.yaml
registry/units.yaml
registry/frames.yaml
registry/bridges.yaml
registry/waivers.yaml
```

Assign one owner for nomenclature changes.

## Phase 2 — Review workflow

Require `templates/PR_CHECKLIST.md` for PRs that touch specs, equations, schemas, Rust public APIs, source ingestion, user-facing terminology, acronym lists, glossary pages, AI context packs, or external terminology-source mappings.

Use `templates/ACRONYM_PROPOSAL.md` for any new acronym meaning or collision.

## Phase 3 — Public CI enforcement

Use the repository-level Rust gate:

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
```

Human reviewers remain responsible for acronym collisions, unknown durable acronyms, candidate-source promotion, and waiver justification.

## Phase 4 — Maintainer-only generated artifacts

Generated acronym inventories and compact terminology packs are maintainer artifacts, not public repository payloads. They may be regenerated outside the public repository when needed, then summarized through normal review notes.
