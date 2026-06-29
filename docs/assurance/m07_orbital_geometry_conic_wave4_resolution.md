# External M07 Orbital Geometry and Conic Wave 4 Resolution

Status: `research_required`

A19 assigns terminal metadata dispositions to the fourth 40 source-ordered rows in classifier group `9A_classical_elements_and_9E_mission_design_contracts`. It uses only the registered classifier metadata and governed A7 equation-batch records; raw Rust-port, M07, and Scilab sources are not opened or imported.

## Outcome

- selected classifier rows: 40;
- exact aliases to existing governed runtimes: 3;
- internal/composite helper exclusions: 8;
- contract or policy blocks: 29;
- selected risk tiers: 37 `medium_risk_requires_contract_review`, 3 `high_risk_requires_numerical_policy`;
- A16-A19 group coverage: 160 rows;
- remaining rows in the classifier group: 217;
- cumulative external rows with terminal dispositions: 321;
- remaining external backlog: 1,002;
- new Rust kernels or formula nodes: 0.

## Exact aliases

- `ch4::semimajor_axis_from_rv` reuses `formula_vault.astrodynamics.elements.semimajor_axis_from_state`;
- `ch4::elliptic_e_to_true` reuses `formula_vault.astrodynamics.kepler.true_anomaly_from_eccentric_anomaly`;
- `ch5::eccentricity_vector` reuses `formula_vault.astrodynamics.elements.eccentricity_vector`.

The remaining rows are excluded as internal/composite solver support or remain blocked pending explicit conic branch, anomaly, time-of-flight, Lagrange f/g, Stumpff numerical, state-unit, degeneracy, and validation-oracle contracts. No parity, certification, or operational-readiness claim is made.

Verifier: `cargo run -p xtask -- verify formula-vault`
