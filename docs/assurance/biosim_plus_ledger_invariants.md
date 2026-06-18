# BioSim-plus Ledger Invariants

Status: `research_required` docs/contracts-only assurance note for Stage 5 prep.

This note defines ledger invariants for the BioSim-plus clean-room scenario schema. It does not add implementation code, public APIs, external fixtures, biological dynamics, habitat-control behavior, medical claims, operational claims, certification evidence, or regulated-use approval.

## Boundary

The invariants are clean-room accounting rules over declared resources and synthetic module-stub deltas. They must not be interpreted as BioSim parity, environmental-control validation, physiological validation, crop-model validation, or hardware performance evidence.

## Required resource families

The scenario subset must account for at least these resource concepts:

| Concept | Scenario resource keys | Unit | Ledger stance |
| --- | --- | --- | --- |
| O2 | `o2_gas` | `kg` | mass store accounting only |
| CO2 | `co2_gas` | `kg` | mass store accounting only |
| H2O | `h2o_potable`, `h2o_waste` | `kg` | mass store accounting only |
| Biomass | `biomass_edible` | `kg` | mass store accounting only |
| Food | `food_stored` | `kg` | contract-only token pending implementation review |
| Power | `power_electrical` | `kWh` | energy-store accounting only |

A future implementation may add aliases or additional resources only through a separate schema-version review.

## Invariant set

### L0: schema and status gate

A ledger report is in domain only when the scenario contract uses a recognized schema version and declares `status: research_required`. Any future status upgrade requires separate source review and validation evidence.

### L1: finite tick grid

Every replay uses a finite tick grid:

- tick count is a positive integer;
- tick duration is finite and strictly positive;
- tick indices are consecutive from zero or from an explicitly declared start index;
- overflow of tick index arithmetic is out of domain.

### L2: resource identity uniqueness

Within one scenario contract, resource keys must be unique. Each key must have exactly one canonical unit. A resource key may appear in multiple stores, but every store must declare the same unit as the resource key.

### L3: finite quantities

Initial amounts, source terms, sink terms, transfer amounts, and module deltas must be finite numbers. NaN and infinity are out of domain.

### L4: nonnegative stores

Every store amount exposed in a before or after snapshot must be nonnegative. A module-delta proposal that would create a negative store balance must be rejected before a replay report is promoted.

### L5: atomic tick commit

For each tick, all accepted module-stub deltas form one atomic commit. The replay report must expose either the complete after-state for the tick or a failed report with no partially accepted after-state.

### L6: one row per resource key per tick

The ledger summary must aggregate stores by `(tick_index, resource_key, canonical_unit)`. Duplicate summary rows for the same tuple are out of domain.

### L7: residual equation

For each resource key at each tick:

```text
observed_delta = after_total - before_total
residual = observed_delta - accounted_sources + accounted_sinks
passed = abs(residual) <= tolerance_abs
```

The tolerance must be finite, nonnegative, and declared in the canonical unit of the resource key. This is an accounting residual, not proof of physical validity.

### L8: transfer accounting

A transfer between stores with the same resource key should contribute equal and opposite store deltas and should not change the resource-key aggregate total. A transfer across different resource keys is a conversion and must be declared as separate source/sink or module-delta terms.

### L9: cross-resource conversions are declared, not inferred

The schema must not infer biological or hardware conversions from resource names. Oxygen, carbon dioxide, water, biomass, food, and power conversions may appear only as explicit module-stub deltas with declared units, signs, and uncertainty status.

### L10: power is not mass

`power_electrical` uses `kWh` and must not be added to mass totals. Cross-domain calculations that relate power to mass-producing or mass-consuming modules remain out of scope until a future reviewed model supplies equations and validation evidence.

### L11: deterministic row ordering

Ledger rows must be sorted by tick index, resource key, canonical unit, and store identifier when detailed rows are emitted. Summary rows must be sorted by tick index, resource key, and canonical unit.

### L12: explicit failure status

A failed invariant produces a `research_required` failed report with a specific failure code. It must not produce corrected data, hidden clipping, hidden re-normalization, or a readiness claim.

## Minimal failure-code vocabulary

A future implementation should reserve these failure codes or equivalent stable labels:

| Code | Meaning |
| --- | --- |
| `schema_version_not_supported` | Contract version is not recognized. |
| `status_not_research_required` | Contract attempts to promote validation status. |
| `nonfinite_tick_duration` | Tick duration is NaN or infinite. |
| `nonpositive_tick_duration` | Tick duration is zero or negative. |
| `resource_key_duplicate` | Duplicate resource key appears in the resource list. |
| `resource_unit_mismatch` | Store or module unit disagrees with canonical resource unit. |
| `nonfinite_quantity` | Amount, source, sink, delta, or tolerance is nonfinite. |
| `negative_store_amount` | Store amount is negative before or after a commit. |
| `negative_store_after_commit` | Module deltas would produce a negative store. |
| `duplicate_ledger_row` | Duplicate summary row appears for one tick/resource/unit tuple. |
| `residual_out_of_tolerance` | Absolute residual exceeds the declared tolerance. |
| `hidden_randomness_blocked` | Replay depends on unselected stochastic behavior. |

## Non-claims

Passing these invariants means only that a clean-room scenario contract obeyed a deterministic resource-accounting contract under declared assumptions. It does not establish physical validity, biological accuracy, habitat safety, medical suitability, flight readiness, mission readiness, operational readiness, certification, or regulated-use approval.
