# External M07 orbital-geometry and conic-branch Wave 2 resolution

A17 assigns terminal metadata dispositions to the next 40 source-ordered rows in classifier group `9A_classical_elements_and_9E_mission_design_contracts`, after the 40 rows already covered by A16. It consumes classifier metadata only and does not inspect raw Rust-port, M07, or Scilab source text.

## Results

- 3 exact aliases reuse governed A7 specific-angular-momentum, eccentricity-vector, and node-vector runtimes;
- 15 generic math, state/element conversion, orbit-determination, or composite summary helpers are excluded from formula scope;
- 22 rows remain contract-blocked for hyperbolic geometry, semilatus/energy/conic algebra, residual weighting, groundtrack/frame/time policy, angular-rate units, or circular/synchronous body/time semantics;
- 38 selected rows retain `medium_risk_requires_contract_review` and 2 retain `high_risk_requires_numerical_policy`;
- 297 rows remain in the classifier group;
- no Rust kernel, formula node, validation card, or source seed is added;
- validation remains `research_required`.

The alias records reuse `equation-batches/a7-astrodynamics-orekit-foundation.tsv` and its existing contract, validation-card, and source-seed paths. A17 makes no M07/Scilab parity, certification, or operational-readiness claim.

## Verification

```text
cargo run -p xtask -- verify formula-vault --self-test
cargo run -p xtask -- verify formula-vault
```
