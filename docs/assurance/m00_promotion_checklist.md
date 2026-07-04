# M00 promotion checklist for `implementation_verified`

RR-027 defines this M00 checklist for promotion packets that recommend `implementation_verified`. It covers M00 unit conversions, angle conversions, vector algebra, and simple math formulas before any normal research-alpha execution status change is proposed.

This checklist does not promote a formula. A completed checklist may support a packet recommendation, but the formula status changes only in a dedicated promotion PR. The checklist is not flight, mission, habitat, life-support, operational, regulatory, or certification evidence.

## Required reviewer roles

- traceability reviewer: confirms source identity, precise locator, formula meaning, variables, units, domain constraints, assumptions, and legal/source boundary.
- implementation reviewer: confirms runtime mapping, CLI/parser mapping, generated-registry linkage, tests, negative tests, tolerance policy, and fail-closed status-gate behavior.

Both reviewers must sign off before a packet can recommend `implementation_verified`.

## Universal M00 gates

- [ ] Packet uses `docs/assurance/promotion_packet_template.md`.
- [ ] Formula ID, legacy alias, equation-batch row, validation card, source seed, sidecar, contract, and runtime-link references are all listed.
- [ ] Current status is recorded before review.
- [ ] Requested status is exactly `implementation_verified`.
- [ ] Packet states that the recommendation does not change real status fields.
- [ ] A separate dedicated promotion PR is identified for any later status-field edits.
- [ ] No unrelated formula status changes are included.
- [ ] M07 or Scilab-derived material, if used for discovery, is not treated as authoritative without independent public/citable/legal source confirmation.

## Source traceability gate

- [ ] Public, citable, legally usable source or governed internal contract is named.
- [ ] Precise source locator is present: title, edition/revision, section/page/table/equation, URL or archive locator where available.
- [ ] Formula statement is independently summarized and does not copy source text wholesale.
- [ ] Assumptions and applicability limits are documented.
- [ ] Source/licensing checklist is referenced.
- [ ] traceability reviewer confirms source identity.
- [ ] traceability reviewer confirms variables and units.
- [ ] traceability reviewer confirms domain constraints and edge cases.

## Unit conversion checklist

For M00 unit conversions, including canonical unit helpers:

- [ ] Input and output units are explicit and dimensionally compatible.
- [ ] Scale factor direction is checked with at least one nontrivial value.
- [ ] Identity or reciprocal relationships are tested where applicable.
- [ ] Zero, positive, negative, and representative finite values are considered when mathematically allowed.
- [ ] Non-finite inputs are rejected or handled according to the contract.
- [ ] Tolerance policy distinguishes exact scale factors from floating-point approximations.
- [ ] CLI flag names match variable meanings and units.

## Angle conversion checklist

For M00 angle formulas such as degrees/radians conversion and wrapping:

- [ ] Degree/radian input and output fields are unambiguous.
- [ ] Known identities are tested, such as `180 deg = pi rad` and `pi rad = 180 deg` where applicable.
- [ ] Negative angles are covered when the contract permits them.
- [ ] Endpoint and wrapping conventions are explicit, especially `[0, tau)`, signed-zero, and `tau`/`2*pi` boundaries.
- [ ] Non-finite angle inputs are rejected.
- [ ] Tolerance policy records whether values are exact, approximate, or branch-sensitive.
- [ ] Runtime behavior matches formula-vault contract names and generated registry aliases.

## Vector algebra checklist

For finite 3-vector M00 helpers:

- [ ] Vector shape is explicit, including dimensionality and ordering.
- [ ] Units or dimensionless quantities are documented for each component.
- [ ] Dot/cross/norm/normalization conventions are stated.
- [ ] Zero-vector and near-zero-vector behavior is explicit.
- [ ] Non-finite components are rejected.
- [ ] Tolerance policy covers scalar results, vector component comparisons, and norm-based checks.
- [ ] Sign/orientation conventions are tested for cross products or handedness-sensitive formulas.
- [ ] At least one orthogonal, parallel, and general finite vector case is included where applicable.

## Simple math formula checklist

For simple scalar formulas and helper identities:

- [ ] Algebraic expression is stated independently and linked to a source or governed contract.
- [ ] Input domain is explicit.
- [ ] Singularities, division-by-zero cases, square-root/log domains, and branch choices are covered when applicable.
- [ ] Exact identities and representative finite values are tested.
- [ ] Invalid input cases fail closed.
- [ ] Tolerance policy states exact, absolute, relative, or ULP expectations.

## Implementation and CLI checklist

- [ ] implementation reviewer confirms runtime symbol and code path.
- [ ] implementation reviewer confirms parser flags and CLI dispatch mapping.
- [ ] implementation reviewer confirms canonical ID and legacy alias route through the same registry/status-gate path.
- [ ] implementation reviewer confirms generated registry linkage or records why regeneration is outside the current task.
- [ ] Normal execution remains blocked until the dedicated promotion PR changes status.
- [ ] Below-threshold status attempts fail with the expected error and no normal result payload.
- [ ] `--preliminary` behavior, if relevant, matches the status gate policy.

## Test evidence checklist

- [ ] Positive test vectors are listed with inputs, expected outputs, tolerance, and source of expected values.
- [ ] Negative tests cover missing inputs, duplicate inputs, unknown flags, invalid numeric values, non-finite values, and domain violations where applicable.
- [ ] Commands are recorded with exact invocation and pass/fail result.
- [ ] `cargo run -p xtask -- verify --all` result is recorded.
- [ ] `cargo test --all` result is recorded.
- [ ] Any expected non-required diagnostic failure is explicitly classified and does not replace required test evidence.

## Status-diff checklist for the later dedicated promotion PR

- [ ] Packet recommendation is accepted before status fields are edited.
- [ ] Only the intended formula status fields change.
- [ ] Validation cards, equation batches, generated registry inputs, and generated artifacts are handled according to the authorized promotion PR scope.
- [ ] The diff links back to the accepted packet.
- [ ] No status is promoted merely because a sidecar, README, checklist, or template exists.

## Non-claims checklist

- [ ] Packet states that `implementation_verified` is not flight readiness.
- [ ] Packet states that `implementation_verified` is not mission readiness.
- [ ] Packet states that `implementation_verified` is not habitat-safety approval.
- [ ] Packet states that `implementation_verified` is not life-support certification.
- [ ] Packet states that `implementation_verified` is not operational or regulatory approval.
- [ ] Public wording remains limited to research and preliminary-design software.

## Signoff block

| Role | Reviewer | Date | Decision | Notes |
|---|---|---|---|---|
| traceability reviewer | | | pending | |
| implementation reviewer | | | pending | |
