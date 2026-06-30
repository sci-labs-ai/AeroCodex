# Formula Registry Contract v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md`, task `LOCK-003`.

This contract fixes the registry layers and generated artifact paths before parser, registry generator, CLI, status-gate, and runtime implementation tasks rely on them. It is docs-only and does not generate or modify runtime artifacts.

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

Later schema work may add field-level detail, but generators must preserve these top-level concepts and names:

- `schema_version`
- `generator_version`
- `generated_by`
- `source_hash`
- `build_input_hash`
- `formula_count`
- `non_claims`
- `formulas`

`source_hash` and `build_input_hash` must be deterministic digests of declared build inputs. They must not contain wall-clock timestamps, machine-local absolute paths, usernames, credentials, or host-specific cache paths.

## Required per-formula field families

The registry must preserve enough information for list, describe, gate, and run tasks without inventing alternate formats:

- canonical formula identifier
- legacy formula identifier or alias list where applicable
- human-readable name and family
- source/batch trace
- validation/status and quarantine metadata
- execution policy metadata
- input definitions in declared order
- output definitions in declared order
- units and domain constraints
- implementation path and runtime symbol when executable
- test vectors in deterministic order
- warnings/non-claims where applicable

## M07 and execution posture

M07 candidates may be visible in registry-derived inventory, counts, and blocked-candidate reports, but this LOCK-003 contract does not authorize execution, family promotion, or validation-status changes. M07 remains visible but blocked until a later explicit family-promotion task changes status.

## Non-claims

Registry artifacts may carry conservative non-claims. Use wording consistent with professional research/preliminary-design software. Do not claim NASA readiness, flight readiness, mission readiness, operational approval, habitat safety approval, life-support certification, or regulated-use certification.
