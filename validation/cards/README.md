# Validation cards

Validation cards are governed traceability and evidence-planning records for AeroCodex formulas, source seeds, and validation families. They participate in the status ladder described in `validation/README.md` and in the fail-closed formula status gate policy.

RR-027 adds only promotion-packet guidance. It does not edit any validation-card status field.

## Relationship to promotion packets

A formula promotion packet may cite validation cards as evidence inputs, but a packet does not change a card by itself. A packet can recommend `implementation_verified` only after the required traceability reviewer and implementation reviewer checks are complete.

Any later status change must happen in a dedicated promotion PR that:

- cites the accepted packet;
- shows the exact status-field diff;
- changes only the intended formula records;
- preserves the fail-closed research-readiness posture;
- keeps sidecars, READMEs, checklists, and templates as metadata rather than execution authority.

## Authoring boundary

Do not change `validation/cards/*.yaml` status fields from README, checklist, or packet-template tasks. Status changes require a separate controller-authorized promotion PR with source trace, variables/units review, domain constraints, implementation mapping, test vectors, negative tests, tolerance policy, and reviewer signoff.

These records are not flight, mission, habitat, life-support, operational, regulatory, or certification evidence. AeroCodex remains professional research and preliminary-design software.