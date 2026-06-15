# Acronym Proposal

Use this template when adding a new acronym, abbreviation, or initialism to `registry/acronyms.yaml`.

## Proposed record

```yaml
acronym_id: acx:acr:<domain>:<token_slug>:v1
token: <TOKEN>
preferred_token: <TOKEN>
expansion: <Expansion>
namespace: acronym
canonical: <canonical_snake_case_term>
canonical_namespace: domain.term
domains:
  - <domain>
status: candidate
source:
  authority: <project|agency|standard|source>
  source_id: acx:source:<source_slug>:v1
  authority_rank: <project_glossary|standards_body|agency_source|project_seed>
  evidence: <quote-safe short evidence or locator; avoid long copied text>
  source_locator: <section/table/page/url fragment if available>
  retrieved_at: <YYYY-MM-DD or null>
first_use:
  requires_definition: true
  allowed_after_definition: true
disambiguation:
  signals:
    - <nearby context signal>
  reject_if_near:
    - <context that points to another meaning>
ai:
  inject_when:
    - token_detected
    - domain_match
  summary: <one-sentence AI-safe meaning and caveat>
  confidence: <low|medium|high>
```

## Review questions

- [ ] Is this an acronym/abbreviation rather than a normal alias or source field?
- [ ] Is the source authority recorded?
- [ ] Is the source compatible with the document or project scope?
- [ ] Does the canonical concept already exist in `registry/concepts.yaml`?
- [ ] Are known collisions represented with `collision_group`?
- [ ] Are disambiguation signals included for colliding tokens?
- [ ] Is first-use behavior explicit?
- [ ] Should this remain `candidate`, become `approved`, or be marked `external` only?

## Approval

```yaml
approved_by: <name_or_role>
approval_date: <YYYY-MM-DD>
status_after_review: <candidate|approved|external|rejected>
notes:
  - <decision note>
```
