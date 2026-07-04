# M00 `implementation_verified` promotion packet template

RR-027 defines the human-readable packet shape used before an M00 formula can be recommended for `implementation_verified`. This template is evidence-planning and review scaffolding only. A completed packet may recommend `implementation_verified`, but status changes happen only in a dedicated promotion PR that explicitly edits the governed status fields and cites the accepted packet.

A promotion packet is not flight, mission, habitat, life-support, operational, regulatory, or certification evidence. `implementation_verified` means a bounded research implementation has been reviewed against traceable formula evidence and tests; it is not flight/mission/habitat/life-support certification.

## 1. Packet identity

- Packet ID:
- Formula ID:
- Formula title:
- Formula family / M00 slice:
- Current validation status:
- Requested validation status: `implementation_verified`
- Requested execution policy, if any:
- Packet author:
- Packet date:
- Related issue / task / PR:
- Dedicated status-change PR, if separate:

## 2. Formula identity

Record every identifier that must resolve to the same formula record.

- Canonical AeroCodex ID:
- Legacy `formula_vault.*` ID, if any:
- Equation-batch row ID(s):
- Formula-vault contract path(s):
- Formula sidecar path(s), if any:
- Validation card path(s):
- Source registry seed path(s):
- Generated registry row / runtime link path(s):
- Public CLI name(s) and aliases:

## 3. Source trace

The traceability reviewer completes this section before any implementation review can approve the packet.

| Evidence item | Required content | Packet entry |
|---|---|---|
| Source identity | Public, citable, legally usable source or project-governed internal contract | |
| Precise locator | Title, report/standard/book identifier, edition/revision, page/table/equation/section, URL/archive locator when available | |
| Formula statement | Independent summary of the formula, not wholesale copied source text | |
| Assumptions | Coordinate/time frame, unit system, simplifications, sign conventions, branch conventions | |
| Licensing review | Confirmation against `docs/assurance/source_licensing_checklist.md` | |
| M07 boundary | If M07 was used for discovery, confirm it is not authoritative by itself | |

## 4. Variables and units

| Symbol / field | Direction | Type | Unit | Required? | Meaning | Source locator |
|---|---|---|---|---|---|---|
| | input | | | | | |
| | output | | | | | |

Checks to record:

- All input and output units are explicit.
- Unit conversions are dimensionally consistent.
- CLI input flags map unambiguously to variables.
- Registry metadata, sidecar metadata, formula-vault contract metadata, and runtime names use the same variable meanings.

## 5. Domain constraints

Record the finite-input and mathematical-domain policy used by implementation and tests.

- Finite input requirement:
- Accepted numeric interval(s):
- Excluded value(s):
- Singularities / undefined cases:
- Endpoint convention(s):
- Branch cut / wrapping convention(s):
- Coordinate or vector shape constraints:
- Error behavior for out-of-domain inputs:

## 6. Implementation mapping

The implementation reviewer completes this section after the traceability evidence is complete.

- Runtime crate:
- Runtime module / function:
- CLI dispatch path:
- Parser input flags:
- Output field name:
- Registry status gate behavior:
- Generated registry linkage:
- Formula-vault runtime-link record:
- Implementation PR(s):
- Code paths reviewed:

Review questions:

- Does the runtime compute the traceable formula without changing the mathematical meaning?
- Does the implementation reject non-finite or out-of-domain inputs consistently with the contract?
- Does CLI execution remain behind the status gate until the dedicated promotion PR changes status?
- Are aliases routed through the same registry and status-gate path as the canonical ID?

## 7. Positive test vectors

| Case ID | Inputs | Expected output | Tolerance | Source of expected value | Command / test path | Result |
|---|---|---|---|---|---|---|
| | | | | | | |

Required notes:

- Include exact identities where applicable.
- Include representative finite values.
- Include endpoint or branch cases for wrapping/angle formulas.
- Cite whether expected values come from source tables, analytic identities, independent calculations, or checked fixtures.

## 8. Negative tests and fail-closed behavior

| Case ID | Invalid / blocked condition | Expected error or status gate | Command / test path | Result |
|---|---|---|---|---|
| | non-finite input | | | |
| | missing required input | | | |
| | duplicate or unknown CLI input | | | |
| | below-threshold status gate | | | |
| | domain violation | | | |

Confirm that blocked or invalid runs do not emit normal result values.

## 9. Tolerance policy

- Absolute tolerance:
- Relative tolerance:
- ULP or exact-equality rule, if applicable:
- Rounding/normalization convention:
- Endpoint tolerance convention:
- Rationale for tolerance choice:
- Evidence that tolerance is tighter than the documented applicability requires:

## 10. Reviewer signoff

Both reviewer roles are required for an `implementation_verified` recommendation.

| Role | Required responsibility | Reviewer | Date | Decision | Notes |
|---|---|---|---|---|---|
| traceability reviewer | Confirms source identity, formula meaning, variables/units, assumptions, legal/source boundary, and precise locator | | | pending | |
| implementation reviewer | Confirms runtime mapping, parser/CLI mapping, tests, negative tests, tolerance policy, and status-gate behavior | | | pending | |

Optional additional reviewers:

- Status-gate reviewer:
- Documentation reviewer:
- Release/handoff reviewer:

## 11. Status change diff preview

A packet can recommend `implementation_verified`; it does not change status by itself. The dedicated promotion PR must include the actual diff and reviewers must confirm it touches only the intended status records.

Expected status-change files, if later authorized:

- Validation card(s):
- Equation-batch manifest row(s):
- Generated registry input(s):
- Generated registry artifact(s), if regenerated by an authorized generator task:

Status-diff checklist:

- Previous status:
- Requested status:
- Execution policy after promotion:
- Files changing status:
- Files explicitly not changing:
- Link to accepted promotion packet:
- Confirmation that no unrelated formula status changes are included:

## 12. Final packet decision

- Recommendation: pending / recommend `implementation_verified` / do not promote
- Required follow-up before status PR:
- Dedicated promotion PR required: yes
- Packet artifacts:
- Command evidence:
- Reviewer notes:

## 13. Non-claims

This packet, any accepted checklist, and any future `implementation_verified` status are not flight, not mission, not habitat, not life-support, not operational approval, not regulatory approval, and not certification evidence. AeroCodex remains professional research and preliminary-design software.