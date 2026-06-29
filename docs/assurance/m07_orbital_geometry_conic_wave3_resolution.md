# External M07 orbital-geometry and conic-branch Wave 3 resolution

A18 assigns terminal metadata dispositions to the next 40 source-ordered rows in classifier group `9A_classical_elements_and_9E_mission_design_contracts`, after the 80 rows already covered by A16-A17. It consumes classifier metadata only and does not inspect raw Rust-port, M07, or Scilab source text.

## Results

- 1 exact alias reuses the governed A7 eccentricity-vector runtime;
- 7 generic math or composite maneuver helpers are excluded from formula scope;
- 32 rows remain contract-blocked for groundtrack/frame/time policy, maneuver geometry and linearization, conic reachability and classification, perturbation/body-rate semantics, anomaly prediction/solver policy, or state-derived eccentricity contracts;
- 33 selected rows retain `medium_risk_requires_contract_review` and 7 retain `high_risk_requires_numerical_policy`;
- 257 rows remain in the classifier group;
- no Rust kernel, formula node, validation card, or source seed is added;
- validation remains `research_required`.

The alias record reuses `equation-batches/a7-astrodynamics-orekit-foundation.tsv` and its existing contract, validation-card, and source-seed paths. A18 makes no M07/Scilab parity, certification, or operational-readiness claim.

## Verification

```text
cargo run -p xtask -- verify formula-vault --self-test
cargo run -p xtask -- verify formula-vault
```
