# AeroCodex Nomenclature PR Checklist

Use this checklist for any PR that touches terminology, schemas, math specs, source ingestion, or durable Rust identifiers.

## Terms, aliases, and acronyms

- [ ] I introduced no new canonical term, or I added a NOM proposal.
- [ ] I introduced no new alias, or I added/updated `registry/aliases.yaml`.
- [ ] I introduced no new acronym meaning, or I added an acronym proposal and updated `registry/acronyms.yaml`.
- [ ] Any new external acronym source is represented in `registry/terminology_sources.yaml`.
- [ ] Ambiguous aliases are marked `status: ambiguous` and require context.
- [ ] Ambiguous acronyms use `collision_group` and disambiguation signals.
- [ ] Candidate acronym meanings are not treated as approved terminology in durable outputs.
- [ ] Deprecated terms are not used in new schemas or durable Rust identifiers.

## Math and symbols

- [ ] Every equation has a symbol table.
- [ ] Every symbol has a namespace, scope, semantic role, domain/type, and status.
- [ ] Every physical quantity has a unit.
- [ ] Every frame-dependent quantity has a frame.
- [ ] Every time-bearing quantity has a time scale when needed.
- [ ] Every subscript/superscript has declared semantics.
- [ ] I did not reuse a glyph in a shared scope without explicit shadowing.

## Rust

- [ ] Durable Rust code uses semantic identifiers rather than bare `n`, `x`, `dt`, etc.
- [ ] Const generics use semantic names like `ROWS`, `COLS`, `SAMPLE_COUNT`, or have a bridge waiver.
- [ ] Public lifetimes are semantic where helpful, such as `'src` or `'de`.
- [ ] Primitive numeric values include unit suffixes in identifiers.
- [ ] Frame-dependent values encode the frame in the type or identifier.
- [ ] Raw identifiers such as `r#type` have waivers unless generated from external bindings.

## Source ingestion

- [ ] Source-original field names and values are preserved.
- [ ] Canonical mappings use approved rules, not spelling heuristics.
- [ ] Source-context-specific terms are disambiguated.
- [ ] Acronym expansions imported from external sources preserve source authority and locator.
- [ ] Ambiguous source terms and acronym tokens fail safely.

## Registry, AI packs, and CI

- [ ] I updated registry files where needed.
- [ ] I updated bridge files where needed.
- [ ] I generated or reviewed an AI terminology pack for acronym-heavy docs.
- [ ] I added a waiver if I intentionally violated a rule.
- [ ] I ran `cargo run -p xtask -- verify --all`.
- [ ] I reviewed durable acronym, symbol, terminology, and waiver changes against the nomenclature registries.
- [ ] I ran Rust formatting/linting if Rust code changed.
