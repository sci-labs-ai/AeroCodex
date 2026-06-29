# External M07 orbital-geometry and conic-branch Wave 1 resolution

A16 assigns terminal metadata dispositions to the first 40 source-ordered rows from classifier group `9A_classical_elements_and_9E_mission_design_contracts`. The classifier risk tier remains `medium_risk_requires_contract_review`; A16 does not downgrade it and does not authorize implementation.

## Results

- 2 rows are provenance aliases of existing governed A7 astrodynamics runtimes: specific angular momentum and eccentricity vector;
- 10 rows are excluded from formula scope because they are generic math, gravitational-parameter lookup/wrapper, vector-force/acceleration, or state-derivative support algorithms rather than standalone bounded formula nodes;
- 28 rows remain contract-blocked pending explicit state/frame/angle, energy/conic-branch, semilatus-rectum, apsis/ellipse, conic-classification, true-anomaly, parabolic-boundary, or hyperbolic mission-geometry contracts;
- 337 rows remain in the classifier group for later bounded waves;
- no Rust kernel, formula node, validation card, or source seed is added;
- validation remains `research_required`.

Existing A7 batch and family-contract metadata are reused directly for the two aliases. `target_resolution_id` remains empty because the targets are governed equation-batch rows rather than A10 metadata-candidate resolution records.

## Verification

```text
cargo run -p xtask -- verify formula-vault --self-test
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify --all
```

A16 raises cumulative terminally processed external rows from 161 to 201 and reduces the remaining backlog from 1,162 to 1,122. It uses classifier metadata and governed repository records only. It does not inspect or import raw Rust-port, M07, or Scilab source text and makes no source-parity, certification, safety, or operational-readiness claim.
