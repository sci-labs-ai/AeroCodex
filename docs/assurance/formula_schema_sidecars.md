# Formula schema sidecars

RR-014 defines the optional per-formula schema sidecar format for AeroCodex formula metadata. The canonical schema contract is `schemas/formula_sidecar.schema.json`; checked-in sidecars live under `formula-schemas/`.

## Purpose

Sidecars supplement existing TSV manifests when a row needs structured metadata that does not fit cleanly into the manifest columns. A sidecar may document inputs, outputs, units, domain constraints, CLI aliases, examples, validation notes, and warnings for a formula such as `m00.angle.deg_to_rad` mapped to `formula_vault.m00.angle.deg2rad`.

Sidecars are metadata enrichment only:

- Sidecars supplement existing TSV manifests; they do not replace those manifests.
- Sidecars do not replace traceability to source contracts, source seeds, or equation-batch records.
- Sidecars do not replace validation cards or validation evidence.
- Sidecars cannot raise validation status, cannot promote validation status, and cannot make a formula executable.
- Sidecars are not execution readiness; they are not formula execution code and are not parser implementation work.
- Sidecars do not generate `generated/formula_registry.json` and must not be treated as hand-authored generated registry artifacts.

## Status and execution authority

Formula status comes from registry generation rules and validation evidence, not from sidecar metadata. A later registry-generation task may read sidecars as inputs, but the generator must still enforce the formula status gate policy and validation evidence requirements.

Normal formula execution remains gated by the status policy. A formula can become normally executable only through later governed tasks that supply the required implementation and evidence. RR-014 does not change formula status, runtime code, product CLI code, equation-batch manifests, validation cards, or generated registry artifacts.

M07 candidates remain blocked unless explicitly promoted by later governed family-specific tasks. A sidecar for an M07 candidate, if introduced by a future task, would remain metadata only and would not override quarantine or execution-blocking policy.

## Review requirements

Reviewers should confirm each sidecar:

1. Uses schema version `aerocodex.formula_sidecar.v1`.
2. Names the canonical `formula_id` and any `legacy_formula_id` alias explicitly.
3. Keeps source traceability in the governed manifest/card/source records instead of claiming to replace them.
4. States units and domain constraints as metadata, not as validation proof.
5. Avoids certification, flight-readiness, operational-readiness, or regulatory claims.
6. Includes warnings/non-claims when examples are illustrative or future-facing.

## Starter sidecar

RR-014 adds one starter sidecar only:

```text
formula-schemas/m00/angle/deg_to_rad.yaml
```

It describes `m00.angle.deg_to_rad`, maps it to `formula_vault.m00.angle.deg2rad`, names input `degrees` as `f64` with unit `deg`, names output `angle_radians` as `f64` with unit `rad`, records a finite-input domain constraint, and includes future-facing describe/run examples as schema examples only.
