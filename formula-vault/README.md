# Formula vault

This directory is a Stage 4 skeleton for future formula-vault metadata. It intentionally contains no imported M07 source code, no external archives, no generated binaries, and no public AeroCodex implementation APIs.

AeroCodex remains research and preliminary-design software. This skeleton does not imply certification, flight readiness, mission readiness, habitat-safety approval, medical-use approval, operational approval, or regulated-use approval.

## Current state

Stage 4 Chunk 3 defines the vault shape in documentation only:

- `docs/assurance/formula_vault_staging.md`
- `docs/source_intake/m07_formula_vault_intake.md`

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
