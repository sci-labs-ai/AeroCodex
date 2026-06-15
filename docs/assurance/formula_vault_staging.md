# Formula-vault staging design

This Stage 4 design defines how quarantined M07 Scilab-to-Rust astrodynamics material may be shaped into future AeroCodex formula-vault records. It is governance and design documentation only. It does not import M07 source code, does not create public Rust application programming interfaces, and does not overwrite `crates/aero-codex-astrodynamics`.

AeroCodex remains research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Source boundary

The governed M07 source artifact is registered as `stage4.m07_rust_port_v14.2026_06_15` in `data-governance/DATA_REGISTRY.yaml` with logical path `external://stage4/aerocodex_rust_port_v14_m07_final_bundle.zip`.

The M07 package reports 1,350 represented function rows and 188 Scilab equivalence jobs. Those counts are intake facts only. They do not promote the package to a certified or public AeroCodex implementation. M07 remains release-candidate / not-certified source material until Rust continuous integration, Scilab equivalence, source traceability, and Simplified General Perturbations 4 (SGP4) certification gates are satisfied for a specific promoted scope.

## Formula-vault purpose

The formula vault is a quarantine and promotion-planning boundary for candidate equations, tables, and algorithms. It exists to make future implementation work source-traceable before any code is promoted.

The vault may contain, in future chunks:

- formula inventory records;
- equation contract drafts;
- source artifact references;
- units, domains, singularity, and tolerance metadata;
- equivalence-test planning records;
- promotion checklists;
- rejection or supersedence notes.

The vault must not contain raw external archives, generated binaries, copied M07 source code, or public API implementations unless a later chunk explicitly approves and verifies that narrower scope.

## Required per-formula contract fields

Every future formula-vault contract must define at least these fields before implementation promotion is considered:

| Field | Required meaning |
| --- | --- |
| `formula_id` | Stable AeroCodex-local formula identifier, unique within the vault. |
| `source_artifact_id` | Data-governance artifact identifier, for M07 normally `stage4.m07_rust_port_v14.2026_06_15`. |
| `source_equation_reference` | Source equation, table, page, function row, file-local label, or other human-reviewable locator. |
| `variables` | Input and output variable names, meanings, dimensions, and units. |
| `coordinate_and_time_assumptions` | Coordinate frame, epoch, timescale, sign convention, body constants, and branch assumptions where applicable. |
| `valid_domain` | Domain over which the formula is intended to be evaluated. |
| `singularities_and_exclusions` | Singular points, invalid regions, convergence hazards, discontinuities, branch cuts, and near-boundary behavior. |
| `numerical_tolerance_policy` | Absolute/relative tolerances, precision expectations, and rationale tied to the formula domain. |
| `reference_oracle_plan` | Independent reference values, Scilab equivalence jobs, SGP4 checks, analytical identities, or other planned oracle comparisons. |
| `validation_status` | Allowed validation/status vocabulary value where the record maps to validation cards; do not invent readiness statuses. |
| `promotion_gate` | Explicit checklist that must pass before the formula can leave quarantine. |

A future machine-readable contract may use YAML, TOML, or another repo-approved source format. The format choice is deferred; the field requirements are not.

## Quarantined M07 lifecycle states

These lifecycle states are formula-vault quarantine labels. They are not data-governance `validation_status` values and do not change `validation/status_vocabulary.yaml` by themselves.

1. `registered_source_material`: the M07 archive is registered, hashed, and license/status tagged.
2. `parsed_inventory_only`: a formula/function inventory exists, but no implementation or public API promotion is implied.
3. `equation_contract_drafted`: the contract fields above are drafted for a formula or formula family.
4. `implementation_candidate`: a bounded Rust candidate is proposed behind review gates, with no automatic public API exposure.
5. `equivalence_tested`: source-equivalence or reference-oracle jobs have run for the bounded candidate and have recorded tolerances/results.
6. `reference_validated`: the candidate has passed the selected reference-oracle plan for its declared scope.
7. `rejected_or_superseded`: the candidate is intentionally blocked, replaced, or retained only for traceability.

A formula may move forward only one reviewable step at a time. A chunk may also move a formula backward if licensing, source traceability, or numerical evidence fails.

## Promotion gate

No M07 formula may become public AeroCodex code until a later chunk records all of the following for that specific formula or family:

- source artifact identifier and source equation/table/page/function-row reference;
- license/source-boundary decision;
- equation contract with variables, units, domains, assumptions, and singularities;
- tolerance policy and numerical method notes;
- reference-oracle or source-equivalence test plan;
- passing local Rust gates for the changed workspace;
- Scilab equivalence evidence where applicable;
- SGP4 certification evidence where applicable;
- validation card or equivalent traceability record;
- documentation that repeats the research/preliminary-design caveat.

Public API promotion remains blocked by default. Stage 4 Chunk 3 does not authorize implementation promotion.

## Future machine checks

This chunk intentionally does not add an `xtask` verifier because it adds only design documentation and an empty vault skeleton. A later chunk that adds machine-readable contract files should extend `cargo run -p xtask -- verify --all` with a formula-vault check that rejects:

- duplicate `formula_id` values;
- missing source artifact identifiers;
- missing equation references;
- missing variables or units;
- missing valid-domain or singularity metadata;
- missing tolerance policy;
- forbidden readiness claims;
- promotion without explicit gate evidence.
