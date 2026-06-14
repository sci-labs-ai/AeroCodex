# AeroCodex Roadmap Milestones

This roadmap locks the Phase 0.001 scope language without promising premature production readiness. It is a planning guide for repository organization, validation metadata, and future development sequencing.

## Roadmap levels

| Level | Target description | Expected evidence before moving beyond the level |
| --- | --- | --- |
| **Phase 0.001** | Planning, first equations, source registry, testing scaffold. | Workspace foundation, conservative source statuses, first tests, and validation-card schema. |
| **Phase 0.01** | Coherent multi-category equation set. | Each required category has a usable API surface and tests for representative valid and invalid inputs. |
| **Phase 0.1** | Early public alpha. | Documentation examples, stronger error handling, more validation cards, and initial user-facing API cleanup. |
| **Phase 0.5** | Broad validation beta. | Reference validation expands materially across categories; source provenance is reviewed and tracked. |
| **Phase 1.0** | Stable verified core API. | Stability policy, reviewed public API, passing full Rust verification, and clearly bounded validation claims. |
| **Post-1.0** | Advanced thermo, astrodynamics, aeroelasticity, controls, optimization. | Stable core exists first; advanced modules remain explicitly scoped and validated. |
| **Beyond 1.0** | High-fidelity modules, uncertainty, agentic optimization, generated reports. | Long-range research and tooling capabilities only, with separate assurance plans. |

## Required scope categories

The following categories are locked as required roadmap categories for AeroCodex. Phase 0.001 does not require each category to be complete, but it should preserve a clear place for each category in code, documentation, validation metadata, or future backlog.

| Category | Phase 0.001 expectation |
| --- | --- |
| atmosphere | First standard-atmosphere primitives and source registry seed. |
| thermodynamics | Perfect-gas primitives and future advanced-thermo path. |
| gas dynamics | Isentropic, normal-shock, expansion, and oblique-shock primitives. |
| aerodynamics | Basic force and coefficient equations. |
| propulsion | Rocket and nozzle basics. |
| heat transfer | Radiation, convection, and conduction primitives. |
| structures | Basic stress, beam, and buckling equations. |
| flight dynamics | Coordinated-turn and simple performance helpers. |
| celestial mechanics / astrodynamics | Two-body, transfer, and celestial helper equations. |
| bio-regenerative life support systems | Simple mass-balance primitives with conservative source status. |
| validation | Codex Card schema, examples, source registry seeds, and xtask checks. |
| agentic optimization | Roadmap category only in Phase 0.001; no operational optimization claim. |

## Milestone interpretation rules

- Roadmap levels are not deployment certifications and are not certification gates.
- `Phase 0.001` is a human planning phase; Cargo package versions remain `0.0.1` during this phase.
- Validation statuses must remain conservative unless source and test evidence justify an upgrade.
- Any future claim of stable, validated, certified, flight-ready, mission-ready, or operationally suitable behavior must be backed by a separate assurance process outside the Phase 0.001 foundation.
