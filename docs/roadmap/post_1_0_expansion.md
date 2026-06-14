# Post-1.0 Expansion Concepts

Post-1.0 work is intentionally separated from Phase 0.001. The concepts below are future directions after the verified core API has a stability policy, stronger validation evidence, and repeatable release checks.

## Post-1.0 candidates

| Area | Future direction | Phase 0.001 boundary |
| --- | --- | --- |
| Advanced thermodynamics | Real-gas models, mixtures, transport properties, equilibrium chemistry, and verified tabular data. | Phase 0.001 keeps only first perfect-gas primitives and research targets. |
| Astrodynamics | High-fidelity perturbations, element/state conversions, numerical propagation, and mission-design utilities. | Phase 0.001 keeps simple two-body and transfer helpers. |
| Aeroelasticity and structures | Coupled aero-structural helpers, flutter scaffolds, and material/allowable libraries. | Phase 0.001 keeps basic stress, beam, and buckling equations. |
| Controls and guidance | Linear models, controller helpers, trajectory tools, and verification examples. | Phase 0.001 does not claim controls capability. |
| Optimization | Design-space exploration, constrained solvers, sensitivity, and agentic optimization workflows. | Phase 0.001 only reserves the roadmap category. |
| Uncertainty and validation | Uncertainty propagation, benchmark datasets, report generators, and reviewed validation suites. | Phase 0.001 uses conservative validation-card and source-registry scaffolding. |
| Generated engineering reports | Traceable reports that cite formulas, assumptions, source status, and tests. | Phase 0.001 may prepare metadata, but does not claim report completeness. |

## Beyond-1.0 concepts

Beyond 1.0 work may include high-fidelity modules, uncertainty-aware workflows, agentic optimization loops, generated reports, expanded reference datasets, and more formal assurance tooling. These are roadmap targets only. They are not claims of current capability, validation, certification, or readiness.

Any post-1.0 expansion must preserve the pure Rust policy, source-traceability model, no-wrapper guardrail, dual license, validation honesty, and safety/certification caveat.

AeroCodex remains not certified and not flight-ready until a separate assurance, qualification, and certification process proves otherwise.
