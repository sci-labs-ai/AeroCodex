# Acronym and External Terminology Protocol

## Purpose

Aerospace acronyms are dense, overloaded, and authority-dependent. AeroCodex therefore treats acronyms as scoped terminology bindings, not as a flat lookup table.

This protocol extends `ACX-NOM-001` with two registries:

```text
registry/acronyms.yaml
registry/terminology_sources.yaml
```

The acronym registry stores approved and candidate meanings. The terminology-source registry stores the authority, scope, and ingestion rules for external sources such as NASA, ECSS, FAA, CCSDS, DoD, and project-local glossaries.

## Core rule

> An acronym token is never self-defining. It must resolve by token, scope, domain, document type, source authority, and nearby context.

Examples:

```text
CDR near design lifecycle        -> Critical Design Review
CDR near recorder/telemetry      -> candidate data-recorder meaning, if adopted by the project
RCS near thrusters/attitude      -> Reaction Control System
RCS near radar/signature         -> Radar Cross Section
AC near FAA/circular            -> Advisory Circular
AC near voltage/frequency        -> Alternating Current
```

If multiple meanings remain plausible, the resolver must return an ambiguity instead of guessing.

## Acronym record

Acronym records use one record per meaning, not one record per token. This allows real collisions to be represented directly.

```yaml
acronym_id: acx:acr:systems:cdr:critical_design_review:v1
token: CDR
preferred_token: CDR
expansion: Critical Design Review
namespace: acronym
canonical: critical_design_review
canonical_namespace: domain.term
domains:
  - systems_engineering
  - program_lifecycle
status: candidate
collision_group: acx:collision:acr:cdr:v1
source:
  authority: AeroCodex seed
  source_id: acx:source:internal_seed:v1
  authority_rank: project_seed
  evidence: Verify against contract or program lifecycle document before approving.
first_use:
  requires_definition: true
  allowed_after_definition: true
disambiguation:
  signals:
    - design review
    - review board
    - design maturity
    - baseline
  reject_if_near:
    - recorder
    - data recorder
ai:
  inject_when:
    - token_detected
    - document_type_match
  summary: CDR often means Critical Design Review in program lifecycle contexts, but the token is ambiguous.
  confidence: medium
```

## Status model

```text
candidate    useful seed, not yet project-authoritative
approved     approved for the declared scope and domains
ambiguous    known token collision requiring contextual resolution
deprecated   historical or source-only usage; not allowed in durable canonical contexts
superseded   replaced by a newer record
rejected     explicitly not an AeroCodex meaning
external     preserved from an external source but not promoted to canonical AeroCodex usage
```

Candidate records are allowed in the starter kit so the system can bootstrap useful suggestions. Production CI may choose to forbid candidate records in release branches.

## Source precedence

For any document, resolve terminology using this precedence order:

1. Project or contract glossary governing that artifact.
2. Applicable standard or regulation named by the artifact.
3. Agency or standards-body glossary in the same domain.
4. Controlled vocabulary or thesaurus.
5. Community/reference source.
6. AeroCodex candidate seed.
7. AI-proposed candidate.

A lower-precedence source may not silently override a higher-precedence source.

## First-use policy

Acronyms should be expanded at first durable use unless the project explicitly whitelists the token for a narrow audience and document family.

Good:

```text
The Preliminary Design Review (PDR) package shall include the interface-control baseline.
```

Bad:

```text
The PDR package shall include the ICD baseline.
```

unless both `PDR` and `ICD` have already been introduced or are declared in a document-level terminology pack.

## Acronyms versus aliases

Acronyms and aliases are related but distinct.

```text
Acronym:  GNC -> Guidance, Navigation, and Control
Alias:    tail_number -> aircraft_registration
Source:   Tail No. -> aircraft_registration in a specific customer import
Symbol:   Δt -> sample_interval within an equation scope
```

Do not put source field names such as `Tail No.` into `registry/acronyms.yaml` unless they are actually acronyms or abbreviations that need token-level resolution. Use `registry/aliases.yaml` or ingestion rules for ordinary source terms.

## AI visibility

The AI should not receive the full acronym registry on every request. Instead, AeroCodex should generate a compact terminology pack for the current artifact or prompt.

The generated pack should include:

```text
- Acronyms actually detected in the text.
- Acronyms that match the declared project/domain.
- Ambiguous tokens with all plausible meanings.
- Source authority and status for each meaning.
- First-use/definition requirements.
- A clear instruction to ask for disambiguation or mark ambiguity when context is insufficient.
```

Example:

```text
Terminology context:
- PDR = Preliminary Design Review [systems_engineering, candidate]
- RCS has multiple candidate meanings:
  - Reaction Control System [spacecraft, propulsion]
  - Radar Cross Section [radar, electromagnetics]
Resolver rule: do not choose between RCS meanings unless local text contains subsystem/thruster or radar/signature context.
```

## External-source ingestion

External source ingestion must be exact and auditable.

Every imported acronym should preserve:

```yaml
token: ADS-B
expansion: Automatic Dependent Surveillance-Broadcast
source:
  authority: FAA
  source_id: acx:source:faa_acronyms:v1
  source_locator: page/section/table locator
  retrieved_at: 2026-06-15
status: candidate
```

Do not scrape external sources directly into `approved` status. Bulk-imported records enter as `candidate` or `external` and require review before they can become project-authoritative.

## Required rejection behavior

When the resolver sees an acronym token with unresolved collisions:

1. Preserve the token.
2. Return all plausible expansions.
3. Do not normalize to a canonical term.
4. Emit `ACX-NOM-E014` for durable contexts.
5. Require context, a definition at first use, or a document-level terminology pack.

## Promotion checklist

Before promoting a candidate acronym to approved:

- [ ] The governing source authority is recorded.
- [ ] The scope and domains are explicit.
- [ ] The expansion capitalization is normalized.
- [ ] Known collisions are represented with `collision_group`.
- [ ] Disambiguation signals are present for colliding tokens.
- [ ] First-use requirements are declared.
- [ ] The canonical term exists or a concept proposal has been submitted.
