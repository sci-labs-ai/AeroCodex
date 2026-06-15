# AeroCodex Nomenclature Protocol

**Protocol:** `ACX-NOM-001`
**Version:** `0.2.0-draft`
**Purpose:** Prevent symbol, acronym, term, unit, frame, source-term, and Rust identifier ambiguity across AeroCodex math, schemas, ingestion, documentation, AI workflows, and Rust code.

This package is integrated in the main AeroCodex repository under `nomenclature/` and should be refined through normal pull requests.

## Core rule

> In math, declare every symbol. In Rust, spell out every meaning. In schemas, use only canonical terms. In ingestion, preserve source wording but map it explicitly. For AI, provide a compact terminology pack rather than a giant glossary dump.

A spelling, glyph, or acronym such as `n`, `N`, `T`, `x`, `station`, `tail_number`, `RCS`, `CDR`, `AC`, `id`, or `leg` is only a **surface form**. It has no approved meaning until resolved against a namespace, scope, domain, unit, frame, source authority, and provenance record.

## What changed in v0.2

This version integrates aerospace/space acronym handling directly into the protocol:

- `registry/acronyms.yaml` — one record per acronym meaning, including collisions such as `RCS`, `CDR`, and `AC`.
- `registry/terminology_sources.yaml` — source registry for NASA, ECSS, FAA, CCSDS, DoD, AIAA, and project-local vocabularies.
- `docs/acronym_terminology_protocol.md` — rules for first use, ambiguity, source precedence, and promotion from candidate to approved.
- `docs/ai_terminology_integration.md` — how AI gets small task-specific terminology packs.
- `tooling/aerocodex_terminology.py` — exact lookup, JSONL export, and AI terminology-pack generation.
- Extended linting for acronym registries and optional unknown-acronym scanning.
- `tooling/aerocodex_acronym_inventory.py` — repository-wide adoption inventory and future new-token guard.

## Included files

```text
.
├── README.md
├── CHANGELOG.md
├── ADOPTION_PLAN.md
├── REFERENCES.md
├── docs/
│   ├── ACX-NOM-001.md
│   ├── acronym_terminology_protocol.md
│   ├── ai_terminology_integration.md
│   ├── math_symbol_protocol.md
│   ├── rust_identifier_profile.md
│   └── source_ingestion_protocol.md
├── registry/
│   ├── acronyms.yaml
│   ├── aliases.yaml
│   ├── bridges.yaml
│   ├── concepts.yaml
│   ├── frames.yaml
│   ├── symbols.yaml
│   ├── terminology_sources.yaml
│   ├── units.yaml
│   └── waivers.yaml
├── schemas/
│   ├── acronym.schema.json
│   ├── alias.schema.json
│   ├── bridge.schema.json
│   ├── concept.schema.json
│   ├── symbol.schema.json
│   ├── terminology_source.schema.json
│   └── waiver.schema.json
├── templates/
│   ├── ACRONYM_PROPOSAL.md
│   ├── AI_TERMINOLOGY_PACK.md
│   ├── EQUATION_SYMBOL_TABLE.yaml
│   ├── NOM_PROPOSAL.md
│   ├── PR_CHECKLIST.md
│   ├── RUST_MATH_BRIDGE.yaml
│   └── SYMBOL_PROPOSAL.md
├── tooling/
│   ├── aerocodex_acronym_inventory.py
│   ├── aerocodex_nom_lint.py
│   └── aerocodex_terminology.py
├── generated/
│   ├── current_repo_acronym_baseline.json
│   ├── current_repo_acronym_inventory.csv
│   ├── current_repo_acronym_inventory.json
│   ├── current_repo_acronym_inventory.md
│   └── terminology/index.jsonl
├── ci/
│   ├── github-actions-example.yml
│   └── nom-rules.yaml
└── examples/
    ├── ai/terminology_pack_for_acronym_demo.md
    ├── ai/terminology_pack_for_acronym_demo.generated.md
    ├── rust/good_bad_examples.rs
    └── specs/
        ├── acronym_resolution_demo.md
        └── trajectory_fit_E014.md
```

## Quick start

1. Work from the AeroCodex repository root.
2. Start with the canonical registry files in `nomenclature/registry/`.
3. Promote only reviewed acronym meanings from `candidate` to `approved`.
4. Require any new specification with equations to include a local symbol table.
5. Require any implementation of a formula to include a Rust–math bridge entry.
6. Generate an AI terminology pack for any aerospace document with acronym-heavy content.
7. Run the lint scaffold and new-token guard:

```bash
python nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature
python nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json
```

Regenerate the repository-wide adoption inventory when deliberately updating the baseline:

```bash
python nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --write-generated
```

Lookup and AI pack examples:

```bash
python nomenclature/tooling/aerocodex_terminology.py --root nomenclature lookup RCS

python nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl

python nomenclature/tooling/aerocodex_terminology.py --root nomenclature pack \
  --text-file docs/phase_0_001/final_microtasks_001_020_report.md \
  --domain life_support \
  --domain source_traceability
```

The lint and terminology tools are intentionally conservative. They are starting points for CI enforcement and AI integration, not final static analyzers.

## Decision model

When a name, symbol, or acronym appears, resolve it using this identity tuple:

```text
(namespace, scope, semantic_role, domain/type, unit/frame/time_scale, source_authority, provenance)
```

Examples:

```text
n in trajectory fitting  -> sample_count
n in graph topology      -> node_count
N in N-number            -> part of an aircraft registration source term
N in NED frame           -> north component
N in const generic       -> compile-time value, preferably replaced with SAMPLE_COUNT, ROWS, COLS, etc.
RCS near thrusters       -> Reaction Control System candidate
RCS near radar signature -> Radar Cross Section candidate
CDR near lifecycle       -> Critical Design Review candidate
CDR near recorder/data   -> data-recorder candidate or unresolved ambiguity
```

Those are not interchangeable.

## AI visibility model

Do **not** inject the entire registry into every model prompt. Generate a scoped pack containing only terms relevant to the current artifact.

```text
registry/*.yaml
   ↓
tooling/aerocodex_terminology.py pack
   ↓
small terminology context for the current AI task
   ↓
AI resolves known terms, surfaces ambiguity, and avoids invented expansions
```

## Status

Draft. Adopt the invariants immediately, use candidate acronym records for discovery, and phase in approval plus mechanical checks.
