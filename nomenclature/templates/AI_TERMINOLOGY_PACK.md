# AeroCodex AI Terminology Pack

Generated for: `<artifact-or-task>`
Generated at: `<YYYY-MM-DDTHH:MM:SSZ>`
Domains: `<domain list>`

## Governing instruction

Use this pack as scoped terminology context under `ACX-NOM-001`. Do not assume acronym meanings that are absent or marked ambiguous. When a token has multiple plausible meanings, return the ambiguity and ask for or infer only from explicit local evidence.

## Detected acronyms

```yaml
- token: <TOKEN>
  selected_meaning: <expansion or null>
  status: <approved|candidate|ambiguous|external>
  domains:
    - <domain>
  source:
    authority: <authority>
    source_id: <source_id>
  ai_note: <short note>
```

## Ambiguous acronyms

```yaml
- token: <TOKEN>
  candidates:
    - expansion: <Expansion A>
      domains: [<domain>]
      evidence_signals: [<signal>]
    - expansion: <Expansion B>
      domains: [<domain>]
      evidence_signals: [<signal>]
  resolver_rule: Use local context; otherwise mark ambiguous.
```

## Canonical terms and aliases

```yaml
- canonical: <canonical_term>
  display_label: <Display Label>
  aliases:
    - <alias>
  status: <approved|candidate|deprecated|ambiguous>
```

## First-use reminders

```yaml
- token: <TOKEN>
  requirement: Expand at first durable use unless already defined in this artifact.
```
