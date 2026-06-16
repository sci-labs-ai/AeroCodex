# Formula vault

This directory is a Stage 4 skeleton for future formula-vault metadata. It intentionally contains no imported M07 source code, no external archives, no generated binaries, and no public AeroCodex implementation APIs.

AeroCodex remains research and preliminary-design software. This skeleton does not imply certification, flight readiness, mission readiness, habitat-safety approval, medical-use approval, operational approval, or regulated-use approval.

## Current state

Stage 4 Chunk 3 defines the vault shape in documentation only:

- `docs/assurance/formula_vault_staging.md`
- `docs/source_intake/m07_formula_vault_intake.md`

Stage 4 Chunk 7A adds the first candidate-gate metadata package without selecting or implementing a formula:

- `docs/assurance/formula_vault_candidate_gate.md`
- `formula-vault/templates/implementation_candidate_slice.yaml`
- validation card `validation.formula_vault.candidate_gate`
- source seed `source.validation.formula_vault_candidate_gate.research_required`

Stage 4 Chunk 7B adds the first bounded metadata-only candidate slice without implementing formulas:

- `formula-vault/candidates/m00_angle_unit_conversions.yaml`
- `docs/assurance/formula_vault_m00_angle_unit_candidate.md`
- validation card `validation.formula_vault.m00_angle_unit_conversions`
- source seed `source.formula_vault.m00_angle_unit_conversions.research_required`

The Chunk 7B slice is limited to three M00 release-gate rows: `app_deg2rad`, `app_rad2deg`, and `app_wrap2pi`. It does not promote exact expressions, wrap endpoint behavior, tolerances, executable code, or public APIs.

The M07 source artifact remains registered externally as `stage4.m07_rust_port_v14.2026_06_15` in `data-governance/DATA_REGISTRY.yaml`.

## Future allowed contents

Future chunks may add reviewed metadata such as:

- formula inventory records;
- equation contract drafts;
- source artifact identifiers;
- source equation/table/page/function-row references;
- variables, units, coordinate/time assumptions, domains, exclusions, and singularities;
- tolerance and reference-oracle plans;
- promotion-gate checklists.

Future chunks must not add raw M07 source code or public API implementation files here unless the active prompt explicitly authorizes that bounded scope and the required gates pass.
