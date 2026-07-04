# Formula-vault contracts

Formula-vault contracts record governed formula metadata: source-facing identifiers, variables, units, domains, assumptions, runtime-link expectations, test-vector expectations, and promotion gates. They are contract and evidence-planning artifacts; executable mathematics remains in the Rust crates.

RR-027 adds only promotion-packet guidance. It does not edit formula-vault resolutions, runtime code, generated registry artifacts, or real formula statuses.

## Relationship to promotion packets

A promotion packet may cite one or more formula-vault contracts to support an `implementation_verified` recommendation. The packet must still include source trace, variables/units review, domain constraints, implementation mapping, positive test vectors, negative tests, tolerance policy, and reviewer signoff.

The required reviewer roles are:

- traceability reviewer: confirms source identity, precise locator, formula meaning, variables, units, assumptions, and legal/source boundary;
- implementation reviewer: confirms runtime mapping, CLI/parser mapping, status-gate behavior, tests, negative tests, and tolerance policy.

A contract, README, sidecar, or checklist cannot promote a formula by itself. Status changes happen only in a dedicated promotion PR that cites an accepted packet and shows the exact status-field diff.

## Contract boundary

Formula-vault contracts must not import raw M07 or Scilab source, assert M07 parity, bypass validation cards, bypass status gates, or grant public execution authority. M07 candidates remain visible-but-blocked unless a later governed family-specific task promotes them through the required traceability, implementation, and validation evidence.

These contracts are not flight, mission, habitat, life-support, operational, regulatory, or certification evidence. AeroCodex remains professional research and preliminary-design software.