# Formula Registry Contract v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md`, tasks `LOCK-003`, `LOCK-004`, and `RR-013`.

This contract fixes the registry layers, Formula Registry v1 schema fields, enum strings, and generated artifact paths before parser, registry generator, CLI, status-gate, and runtime implementation tasks rely on them. It is a contract only: it does not generate the registry, make formulas runnable, promote formula status, or modify runtime artifacts.

## Registry layering

```text
source manifests / equation-batch TSVs
        ↓
formula sidecar YAML where needed
        ↓
generated/formula_registry.json
        ↓
generated/formula_registry.sha256
        ↓
crates/aero-codex-registry/src/generated.rs
        ↓
CLI + Rust registry/runtime boundary
```

Agents must not add a parallel registry JSON path, a second generated Rust registry module path, or a separate formula catalog format.

## Canonical generated artifact paths

The following paths are the only approved checked-in generated/schema/sidecar/promotion locations for the registry infrastructure unless a later explicit contract task updates this file:

```text
schemas/formula_registry.schema.json
schemas/formula_sidecar.schema.json
schemas/formula_promotion_packet.schema.json
formula-schemas/m00/**/*.yaml
formula-schemas/a4/**/*.yaml
promotion-packets/m00/**/*.yaml
promotion-packets/a4/**/*.yaml
generated/formula_registry.json
generated/formula_registry.sha256
generated/runtime_equation_inventory.json
generated/runtime_equation_inventory.sha256
generated/m07_candidate_registry.json
generated/m07_candidate_registry.sha256
crates/aero-codex-registry/src/generated.rs
```

Path-family contract:

- `schemas/*.schema.json` stores stable JSON Schema contracts.
- `formula-schemas/**` stores formula sidecars and per-formula schema inputs.
- `promotion-packets/**` stores promotion packet YAML inputs.
- `generated/*.json` stores deterministic generated JSON artifacts.
- `generated/*.sha256` stores SHA-256 sidecars for checked-in generated JSON artifacts.
- `crates/aero-codex-registry/src/generated.rs` stores the generated Rust registry module.

## Required top-level registry fields

`schemas/formula_registry.schema.json` is the Formula Registry v1 JSON Schema contract. The schema-required top-level fields are exactly these RR-013 fields:

- `schema_version`
- `generator_version`
- `generated_by`
- `source_hash`
- `formula_count`
- `non_claims`
- `formulas`

`source_hash` is deterministic build-input evidence: it is a SHA-256 digest of declared source/build inputs for the generated registry. It must not contain wall-clock timestamps, machine-local absolute paths, usernames, credentials, host-specific cache paths, or certification claims. Checked-in generated registries must not include wall-clock timestamps.

## Required formula fields

Every Formula Registry v1 formula entry must include the RR-013 required fields below. The current merged schema also preserves the existing compatibility fields `aliases` and `summary`; they do not authorize alias rewrites or execution.

- `formula_id`
- `legacy_formula_id`
- `aliases`
- `name`
- `summary`
- `family`
- `batch_id`
- `status`
- `quarantine_state`
- `execution_policy`
- `source_trace`
- `inputs`
- `outputs`
- `units`
- `domain_constraints`
- `implementation_path`
- `runtime_symbol`
- `test_vectors`
- `warnings`

The registry must preserve declared input order, declared output order, deterministic test-vector order, source/batch traceability, validation/status metadata, quarantine metadata, execution policy metadata, implementation path and runtime symbol when executable, and warnings/non-claims where applicable.

## Canonical enum values

Formula Registry v1 uses the following status vocabulary for public registry exposure:

- `research_required`
- `equation_traceable`
- `implementation_verified`
- `reference_validated`

Do not add `experiment_validated` to Formula Registry v1 unless a later explicit task updates the schema and status-gate policy.

`execution_policy` is exactly one of:

- `blocked`
- `preliminary_flag_required`
- `normal_research`
- `publication_supporting`

`quarantine_state` is exactly one of:

- `none`
- `m07_candidate_blocked`
- `missing_metadata_blocked`
- `below_execution_threshold`
- `deprecated_alias`

## Exposure and non-promotion rules

Schema validation is not formula validation. Registry inclusion is not execution readiness. A formula with `status: "research_required"` is visible inventory only and does not mean validated. Normal research execution remains gated by `implementation_verified` or a higher explicitly supported registry status, plus registry freshness, runtime symbol, domain-constraint, and test-vector gates.

M07 candidates may be visible in registry-derived inventory, counts, and blocked-candidate reports, but this contract does not authorize execution, family promotion, or validation-status changes. M07 remains visible but blocked with `quarantine_state: "m07_candidate_blocked"` and `execution_policy: "blocked"` until a later explicit family-promotion task changes status.

## Example: M00 implementation-verified entry

```json
{
  "formula_id": "m00.angle.deg_to_rad",
  "legacy_formula_id": "formula_vault.m00.angle.deg2rad",
  "aliases": ["formula_vault.m00.angle.deg2rad"],
  "name": "Degrees to radians",
  "summary": "Converts a finite angle in degrees to radians.",
  "family": "m00.angle",
  "batch_id": "m00-angle-vector",
  "status": "implementation_verified",
  "quarantine_state": "none",
  "execution_policy": "normal_research",
  "source_trace": {
    "contract_path": "formula-vault/contracts/m00_angle.yaml",
    "validation_card_path": "validation/cards/m00_angle_deg_to_rad.yaml",
    "source_seed_path": "formula-vault/source-seeds/m00_angle.md"
  },
  "inputs": [
    {"name": "degrees", "type": "f64", "unit": "deg", "required": true}
  ],
  "outputs": [
    {"name": "radians", "type": "f64", "unit": "rad"}
  ],
  "units": {"input": "deg", "output": "rad"},
  "domain_constraints": [
    {"name": "finite_input", "description": "Input must be finite."}
  ],
  "implementation_path": {
    "package": "aero-codex-core",
    "crate_name": "aero_codex_core",
    "path": "crates/aero-codex-core/src/units.rs"
  },
  "runtime_symbol": "aero_codex_core::units::deg_to_rad",
  "test_vectors": [
    {
      "id": "deg_to_rad_180",
      "inputs": {"degrees": 180.0},
      "expected": {"radians": 3.141592653589793},
      "abs_tol": 1e-12
    }
  ],
  "warnings": [
    "Normal research execution still requires registry freshness, domain checks, and test-vector gates."
  ]
}
```

## Example: blocked M07 candidate entry

```json
{
  "formula_id": "m07.candidate.some_formula",
  "legacy_formula_id": null,
  "aliases": [],
  "name": "M07 blocked candidate formula",
  "summary": "Visible inventory-only M07 candidate; not executable.",
  "family": "m07.candidate",
  "batch_id": null,
  "status": "research_required",
  "quarantine_state": "m07_candidate_blocked",
  "execution_policy": "blocked",
  "source_trace": null,
  "inputs": [],
  "outputs": [],
  "units": null,
  "domain_constraints": [],
  "implementation_path": null,
  "runtime_symbol": null,
  "test_vectors": [],
  "warnings": [
    "research_required does not mean validated.",
    "M07 candidates remain blocked unless later promoted through explicit tasks."
  ]
}
```

## Non-claims

Registry artifacts may carry conservative non-claims. AeroCodex is professional research/preliminary-design software and is not certified as operational aerospace software. Do not claim NASA readiness, flight readiness, mission readiness, operational approval, habitat safety approval, life-support certification, or regulated-use certification. `source_hash` is deterministic build-input evidence, not a certification claim.
