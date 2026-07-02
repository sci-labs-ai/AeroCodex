# Formula schema sidecars

`formula-schemas/` is the canonical root for optional per-formula schema sidecars.

## Naming convention

Use repository-relative YAML paths shaped as:

```text
formula-schemas/<family>/<subfamily>/<leaf>.yaml
```

Examples:

```text
formula-schemas/m00/angle/deg_to_rad.yaml
```

The path should mirror the canonical `formula_id` after the family prefix. For `m00.angle.deg_to_rad`, the expected sidecar path is `formula-schemas/m00/angle/deg_to_rad.yaml`.

## Formula identity policy

RR-018 formula identity policy lives at `docs/assurance/formula_id_policy.md`. Sidecars should use the canonical readable ID in `formula_id`, such as `m00.angle.deg_to_rad` or `m00.angle.rad_to_deg`, and should record the primary legacy `formula_vault.*` spelling in `legacy_formula_id` when one exists.

`legacy_formula_id` and any `cli_aliases` are traceability and migration metadata only. They must not bypass status gates, must not promote validation status, and do not make formulas executable. Later registry generation must preserve governed aliases deterministically and fail closed on ambiguous alias conflicts.

## What sidecars may contain

A sidecar may provide structured metadata for later registry enrichment:

- `schema_version`
- `formula_id`
- `legacy_formula_id`
- `name`
- `family`
- `inputs`
- `outputs`
- `units`
- `domain_constraints`
- `cli_aliases`
- `examples`
- `validation_notes`
- `warnings`

Sidecars supplement existing TSV manifests. They do not replace traceability and do not replace validation cards.

## Review posture

Checked-in sidecars require normal code-review discipline even when they are metadata-only. Review must confirm that the sidecar matches the governed formula ID, names any legacy alias explicitly, keeps units and domain constraints as metadata, and avoids certification or operational-readiness claims.

## Non-promotion rule

A sidecar cannot raise validation status and cannot promote validation status. Formula status comes from registry generation rules and validation evidence. M07 candidates remain blocked unless explicitly promoted by later governed family-specific tasks.

## No generated-registry rule

Adding or editing a sidecar does not generate, refresh, or hand-author `generated/formula_registry.json`, `generated/formula_registry.sha256`, or any other `generated/**` artifact. Registry generation belongs to later tasks.

## No runtime-execution rule

A sidecar does not make formulas executable and is not execution readiness. It does not add parser code, product CLI code, runtime formula code, formula execution code, validation status changes, or validation-card changes.

## RR-014 starter sidecar

RR-014 intentionally adds only one starter sidecar:

```text
formula-schemas/m00/angle/deg_to_rad.yaml
```

That file maps `m00.angle.deg_to_rad` to `formula_vault.m00.angle.deg2rad`, records input `degrees` in `deg`, records output `angle_radians` in `rad`, and states the finite-input constraint as metadata only.
