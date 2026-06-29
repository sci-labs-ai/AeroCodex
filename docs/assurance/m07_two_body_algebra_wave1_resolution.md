# External M07 classical two-body algebra Wave 1 resolution

A14 assigns terminal metadata dispositions to the first 40 rows, in source-row order, from classifier group `8E_or_9A_classical_two_body_algebra_contracts`. The classifier continues to label these rows `medium_risk_requires_contract_review`; A14 does not downgrade that risk tier.

## Results

- 16 rows are provenance aliases of five existing governed A7 astrodynamics runtimes: circular speed, circular period, escape speed, vis-viva speed, and mean motion;
- 24 rows remain contract-blocked because their aliases do not establish a safe exact mapping to an existing governed runtime;
- the blocked rows cover ambiguous specific-energy input forms, semilatus rectum, conic radius, apsis geometry, reference-radius altitude conversion, general semimajor-axis period semantics, and inverse energy-to-semimajor-axis semantics;
- 9 rows remain in the 49-row classifier group for a later bounded wave;
- no Rust kernel, formula node, validation card, or source seed is added;
- validation remains `research_required`.

The alias target counts are four circular-speed records, four circular-period records, four escape-speed records, two vis-viva records, and two mean-motion records. Existing A7 batch and family-contract metadata are reused directly. `target_resolution_id` remains empty because these targets are governed equation-batch rows rather than A10 metadata-candidate resolution records.

## Verification

```text
cargo run -p xtask -- verify formula-vault --self-test
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify --all
```

A14 raises cumulative terminally processed external rows from 112 to 152 and reduces the remaining backlog from 1,211 to 1,171. It uses classifier metadata and governed repository records only. It does not inspect or import raw Rust-port, M07, or Scilab source text and makes no source-parity, certification, safety, or operational-readiness claim.

## Later aggregate accounting

A15 adds a separate 9-row Wave 2 manifest for the remainder of the same classifier group. A14 itself remains a 40-row wave; A14-A15 together cover all 49 classical two-body algebra rows with 22 aliases and 27 contract blocks. Aggregate terminally processed external rows are now 161 and the remaining backlog is 1,162.
