# Worked example: degrees to radians

This example uses the promoted formula `m00.angle.deg_to_rad` to convert 180 degrees.

```bash
aerocodex formula describe m00.angle.deg_to_rad --json
aerocodex formula run m00.angle.deg_to_rad --degrees 180
```

The calculation output is:

```text
command=formula run
formula_id=m00.angle.deg_to_rad
canonical_formula_id=m00.angle.deg_to_rad
requested_formula_id=m00.angle.deg_to_rad
legacy_formula_id=formula_vault.m00.angle.degrees_to_radians
runtime_symbol=m00_degrees_to_radians
input_syntax=flag_style
angle_radians=3.141592653589793
validation_status=implementation_verified
```

Trace the result through:

- release authority: `release/release-manifest.toml`;
- batch row: `equation-batches/m00-angle-vector.tsv`;
- formula contract: `formula-vault/contracts/m00_angle_unit_conversions_contract.yaml`;
- validation record: `validation/cards/validation_formula_vault_m00_angle_unit_conversions.yaml`;
- source record: the source seed linked by that batch row;
- promotion packet: `validation/release_slice/promotion_packets/m00_angle_deg_to_rad.yaml`;
- runtime symbol: `m00_degrees_to_radians` in `aero-codex-astrodynamics`;
- analytical check: 180 × π / 180 = π radians, with the governed binary64 tolerance policy.

`implementation_verified` means that this bounded implementation and public dispatch path passed the linked software evidence. It does not claim independent scientific reference validation, certification, or suitability for operational, flight, mission, habitat, medical, or regulated use.
