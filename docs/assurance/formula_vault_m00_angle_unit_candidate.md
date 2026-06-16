# Formula-vault M00 angle/unit conversion metadata slice

Stage 4 Chunk 7B adds the first bounded formula-vault candidate metadata record after the Chunk 7A candidate gate. It selects only three M00 helper rows from the registered M07 release-candidate archive:

- `app_deg2rad` / `foa_app_ast_app_deg2rad`;
- `app_rad2deg` / `foa_app_ast_app_rad2deg`;
- `app_wrap2pi` / `foa_app_ast_app_wrap2pi`.

The machine-readable record is `formula-vault/candidates/m00_angle_unit_conversions.yaml`. Stage 4 Chunk 7D adds the companion per-candidate manifest `formula-vault/manifests/m00_angle_unit_conversions_manifest.yaml` to deepen row/function/source-file reference links without implementing formulas.

## Source grounding

The source artifact remains external and quarantined:

- data-governance artifact: `stage4.m07_rust_port_v14.2026_06_15`;
- registered bundle SHA256: `15b1ca3a39267187167c43ea1228f28fd4736c4456f65d072dc42a32a7b19190`;
- validation-doc nested ZIP SHA256 observed during Chunk 7B inspection: `5f228a2f885bbe67b2c280fb18d3bbd0c2386435f9758112e487ba6f9dc0142e`.

The candidate record cites M07 status metadata only:

- `release-gate status csv` rows 3, 4, and 5;
- source file locator `.../appendices_scilab_astrodynamics/src/constants/ast_app_constants_conversions.sci`;
- Scilab equivalence job `equivalence job 002` from `Scilab equivalence jobs csv` row 3.

No M07 source file, archive, executable fixture, generated output, or source-code expression is committed to AeroCodex by this slice.

## Candidate contract status

This slice drafts metadata for variables, units, non-frame dependence, non-time-scale dependence, finite-domain requirements, invalid non-finite inputs, and future equivalence evidence. It deliberately leaves exact numerical expressions, wrap endpoint behavior, and tolerances blocked until a later implementation-authorized chunk performs source review and writes tests.

The status remains `research_required`. The record is an equation-contract draft and not an implementation candidate for public use.

## Promotion boundary

Chunk 7B does not authorize:

- no formula implementation;
- no source translation;
- no Scilab execution;
- no reference-fixture import;
- no public application programming interface promotion;
- no crate/workspace/dependency changes;
- no certification evidence;
- no flight-readiness, mission-readiness, operational-use, or regulated-use claims.

Any later promotion must be limited to this bounded formula list and must satisfy the per-slice gate defined in `docs/assurance/formula_vault_candidate_gate.md`.


## Chunk 7F contract cross-link

Chunk 7F adds `formula-vault/contracts/m00_angle_unit_conversions_contract.yaml` as a metadata-only source-expression and test-vector contract for this same bounded candidate list. The contract records independent mathematical summaries and endpoint-sensitive `wrap2pi` expectations while keeping implementation, public API promotion, Scilab output import, and validation-status upgrade blocked.
