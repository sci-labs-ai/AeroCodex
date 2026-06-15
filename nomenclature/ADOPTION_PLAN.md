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

Seed external terminology sources as `candidate_source`, not as authoritative imports. Bulk-imported acronyms should start as `candidate` or `external` until reviewed.

## Phase 2 — Review workflow

Require `templates/PR_CHECKLIST.md` for PRs that touch:

- Specs.
- Equations.
- Schemas.
- Rust identifiers in public APIs.
- Unit-bearing or frame-bearing values.
- Source ingestion rules.
- User-facing terminology.
- Acronym lists, glossary pages, AI context packs, or external terminology-source mappings.

Use `templates/ACRONYM_PROPOSAL.md` for any new acronym meaning or collision.

## Phase 3 — Soft linting

Run:

```bash
python tooling/aerocodex_nom_lint.py --root .
```

Treat the first run as discovery. Do not fail CI until the baseline is triaged.

For acronym-heavy repositories, also run:

```bash
python tooling/aerocodex_nom_lint.py --root . --scan-acronyms
```

Unknown-acronym scanning is intentionally noisy at first. Use it to populate candidate records and waivers.

## Phase 4 — AI terminology-pack pilot

For requirements, ICDs, review packages, flight/software specs, source imports, and aerospace summaries, generate a scoped AI pack:

```bash
python tooling/aerocodex_terminology.py --root . pack \
  --text-file path/to/document.md \
  --domain spacecraft \
  --domain systems_engineering
```

Pilot rule:

- AI may use a single unambiguous approved term directly.
- AI must flag candidate-only meanings when they matter.
- AI must return ambiguity for collisions such as `RCS`, `CDR`, or `AC` unless local context resolves them.
- AI must not invent expansions for unregistered acronyms in durable outputs.

## Phase 5 — CI enforcement

Add the GitHub Actions example from `ci/github-actions-example.yml`.

Fail CI for:

- Unbound symbols in declared specs.
- Ambiguous alias records.
- Ambiguous acronym records without disambiguation metadata.
- Unknown acronyms in durable docs after the baseline is established.
- Missing terminology-source references for imported external acronyms.
- Missing physical units.
- Missing frames where required.
- Missing Rust–math bridge records.
- Deprecated canonical terms in schemas or durable Rust code.
- Raw Rust identifiers without waiver.

## Phase 6 — Generated docs and IDE integration

Longer-term targets:

- Generate glossary pages from registry files.
- Generate acronym collision pages from `registry/acronyms.yaml`.
- Generate symbol tables from `symbols.yaml`.
- Generate Rust doc links from bridge records.
- Generate AI terminology packs on demand for editor, review, and assistant workflows.
- Add editor diagnostics for ambiguous symbols and acronyms.
- Add schema validation for ingestion mappings.
