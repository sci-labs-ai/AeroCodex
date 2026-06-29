# External M07 classical two-body algebra Wave 2 resolution

A15 assigns terminal metadata dispositions to the remaining 9 rows, in source-row order, from classifier group `8E_or_9A_classical_two_body_algebra_contracts`. Together A14 and A15 cover all 49 rows in that group. The classifier risk tier remains `medium_risk_requires_contract_review`; A15 does not downgrade it.

## Results

- 6 rows are provenance aliases of existing governed A7 astrodynamics runtimes: two circular-speed records, two mean-motion records, one escape-speed record, and one vis-viva-speed record;
- 2 specific-energy aliases remain contract-blocked because the alias does not distinguish semimajor-axis form from state or speed/radius form;
- `ch8_planet_mean_motion_AU_TU` remains contract-blocked because reuse of the SI-oriented mean-motion runtime requires an explicit astronomical-unit/time-unit input, output, and scaling contract;
- A14-A15 now cover all 49 rows in the two-body algebra classifier group with 22 aliases and 27 contract blocks;
- no Rust kernel, formula node, validation card, or source seed is added;
- validation remains `research_required`.

Existing A7 batch and family-contract metadata are reused directly for aliases. `target_resolution_id` remains empty because the targets are governed equation-batch rows rather than A10 metadata-candidate resolution records.

## Verification

```text
cargo run -p xtask -- verify formula-vault --self-test
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify --all
```

A15 raises cumulative terminally processed external rows from 152 to 161 and reduces the remaining backlog from 1,171 to 1,162. It uses classifier metadata and governed repository records only. It does not inspect or import raw Rust-port, M07, or Scilab source text and makes no source-parity, certification, safety, or operational-readiness claim.
