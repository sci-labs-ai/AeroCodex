# BioSim-plus Clean-Room Scenario Schema

Status: `research_required` docs/contracts-only handoff for Stage 5 prep.

This document defines a small BioSim-plus scenario layer that can sit above the existing clean-room BioSim-RS resource/tick, transaction, deterministic replay, resource-ledger, and friend-test smoke slices. It is intentionally not a public API, not a command-line surface, not a biological dynamics model, and not a habitat controller.

AeroCodex remains research and preliminary-design software. This document is not certification evidence, not operational approval, not habitat-safety evidence, not medical-use evidence, and not regulated-use approval.

## Clean-room boundary

The schema is written from AeroCodex clean-room requirements and the current dual `MIT OR Apache-2.0` repository boundary. It must not import, translate, or paraphrase GPL Java BioSim implementation code, GPL-bound BioSim-RS scaffold code, scenario fixtures, generated outputs, class names, method structure, comments, control flow, or package architecture.

The deployment agent must keep the GPL Java archive, GPL-bound bootstrap scaffold, generated fixtures, external runtimes, `target/`, evidence logs, and root `Cargo.lock` out of any patch. The current Stage 5 prep handoff is docs/contracts only.

## Relationship to existing clean-room slices

The proposed layer builds conceptually on the existing clean-room primitives without adding new Rust code in this chunk:

| Existing capability | Current role in this schema |
| --- | --- |
| Resource identity and tick validation | Provides the initial resource vocabulary, positive-duration ticks, and consecutive tick boundaries. |
| Atomic resource transaction | Provides the idea of all-or-nothing resource deltas at a tick boundary. |
| Deterministic ordering and digest replay | Provides the replay principle for canonical ordering and deterministic report generation. |
| Resource ledger and minimal oxygen-loop proof | Provides ledger residual language and bounded resource-balance checks. |
| Static friend-test smoke report | Provides the precedent for a non-operational friend-test output format. |

No existing helper is promoted to scenario execution by this document. Scenario execution, persistent ledger storage, fixture replay, external BioSim parity, and habitat-control behavior remain future work.

## Scope of the v0.1 schema subset

The v0.1 scenario subset is deliberately small. It describes a deterministic, finite, multi-tick resource-accounting replay over named stores and declared module stubs. It does not model physiological response, plant growth, microbial dynamics, thermal control, cabin control, fault management, or crew health.

A scenario record has these top-level sections:

| Field | Required | Purpose |
| --- | --- | --- |
| `schema_version` | yes | Exact contract version, initially `biosim_plus.scenario_contract.v0_1`. |
| `scenario_id` | yes | Stable lowercase identifier for the clean-room example. |
| `status` | yes | Must remain `research_required`. |
| `time` | yes | Tick duration and tick-count metadata. |
| `resources` | yes | Canonical resource keys, units, and nonnegative initial store amounts. |
| `modules` | yes | Deterministic clean-room module stubs that emit resource deltas; no imported BioSim implementation. |
| `ledger` | yes | Per-resource invariant configuration and residual tolerances. |
| `replay` | yes | Canonical ordering, deterministic digest, and output report policy. |
| `uncertainty` | yes | Optional sensitivity hooks; disabled unless explicitly selected by a future runner. |
| `non_claims` | yes | Machine-readable non-claim flags. |

## Resource vocabulary

The scenario subset must include the resource concepts requested for BioSim-plus while preserving the existing clean-room identity language where possible.

| Scenario key | Canonical unit | Current mapping stance | Notes |
| --- | --- | --- | --- |
| `o2_gas` | `kg` | maps to current clean-room oxygen gas identity | Resource accounting only; not respiratory safety. |
| `co2_gas` | `kg` | maps to current clean-room carbon-dioxide gas identity | Resource accounting only; not atmospheric control. |
| `h2o_potable` | `kg` | maps to current clean-room potable-water identity | Resource accounting only. |
| `h2o_waste` | `kg` | maps to current clean-room waste-water identity | Resource accounting only. |
| `biomass_edible` | `kg` | maps to current clean-room edible-biomass identity | Resource accounting only; not nutrition suitability. |
| `food_stored` | `kg` | contract-only token pending implementation review | Included so food can be tracked separately from biomass in scenario examples. |
| `power_electrical` | `kWh` | maps to current clean-room electrical-energy identity | Resource accounting only; not a power-system model. |

Future deployment chunks may decide whether `food_stored` becomes a new clean-room resource identity or remains an alias over an existing edible-biomass store. That decision must be made in a separate source/contract review before implementation.

## Minimal scenario shape

A minimal habitat scenario example is a synthetic closed accounting box with one or more named stores, fixed module stubs, and a finite tick count. It is called a habitat scenario only because its resource vocabulary resembles a habitat resource ledger; it is not a habitat-safety model.

Example module-stub classes are allowed only as neutral functional roles:

- `fixed_load_stub`: deterministic resource deltas representing a declared load profile.
- `fixed_producer_stub`: deterministic resource deltas representing a declared production profile.
- `storage_transfer_stub`: deterministic transfer between stores of the same resource key.
- `report_only_stub`: records telemetry without changing balances.

Module-stub records must not use external BioSim class names, Java terms, GPL-bound scaffold crate names, or copied control-flow structure.

## Deterministic multi-tick replay contract

A future replay engine may execute a scenario only if all of these contract checks pass:

1. `schema_version` is recognized exactly.
2. `status` is `research_required`.
3. Tick count is a positive integer and tick duration is finite and strictly positive.
4. Resource keys are unique within the scenario.
5. Resource units are canonical for each key.
6. All initial amounts and module deltas are finite numbers.
7. Initial amounts are nonnegative.
8. Module order is explicit and stable.
9. Per-tick resource deltas are applied atomically.
10. Ledger rows are sorted by tick index, resource key, and store identifier.
11. Report digests are deterministic smoke-test digests, not cryptographic evidence.
12. Any nonzero residual beyond declared tolerance produces a failed research report, not a corrected output.

Replay output must include enough metadata to reproduce the same canonical report from the same input contract: scenario ID, schema version, tick count, tick duration, module order, resource key order, ledger tolerance policy, digest algorithm label, report status, and non-claim text.

## Ledger invariants expected by the schema

The resource ledger is a conservation-style accounting ledger, not a biological or physical proof. At each tick, for each resource key, the reported residual is:

```text
residual = (after_total - before_total) - accounted_sources + accounted_sinks
```

The residual must be compared to a declared absolute tolerance in the resource's canonical unit. A report may pass only when every row is finite and every absolute residual is less than or equal to the tolerance. The report remains `research_required` even when all rows pass.

Cross-resource conversions, such as CO2-to-biomass, water recovery, food production, or power use, are allowed in a scenario contract only as declared module-stub deltas. They are not validated biological reactions or hardware performance models in this docs-only chunk.

## Uncertainty and sensitivity hooks

The v0.1 schema may carry optional sensitivity hooks for future analysis, but a default replay must ignore them unless a future implementation explicitly selects a sensitivity case.

Allowed hook types:

- named scalar parameter values;
- bounded deterministic parameter sweeps;
- seeded pseudo-random case selection declared by algorithm name;
- disabled-by-default uncertainty distributions with units and bounds.

Blocked hook behavior:

- hidden random global state;
- system clock dependence;
- network or external file dependence;
- automatic upgrade of validation status;
- biological, medical, crew-safety, or habitat-safety conclusions.

## Friend-test report v2 expectations

Friend-test report v2 is a static/reporting contract. It should be readable by an outside tester and must include:

- scenario contract ID and schema version;
- status: `research_required`;
- resource count and tick count;
- deterministic replay digest fields if a future runner creates them;
- ledger pass/fail summary;
- disabled uncertainty/sensitivity summary;
- exact non-claim wording.

This handoff does not add a persistent CLI or API command. Any future command surface must be scoped in a separate deployment chunk and must carry its own tests and validation card.

## Required future promotion gates

Before scenario execution is promoted beyond docs/contracts:

1. Decide whether the scenario schema files are committed as root contracts, `docs/architecture` examples, or a future `data/scenarios` fixture lane.
2. Add a clean-room implementation design that does not inspect GPL implementation code.
3. Add fixture license and hash records if any external scenario file or golden output is used.
4. Add unit tests for schema parsing, canonical ordering, multi-tick replay, ledger residuals, and report text.
5. Add a source-boundary review proving that no GPL Java or scaffold source entered the dual core.
6. Keep all validation status at `research_required` until source review and independent validation evidence justify a later status change.

## Explicit non-scope

This schema does not provide or imply:

- medical suitability;
- crew health assessment;
- closed-loop habitat control;
- environmental control approval;
- real biological dynamics;
- BioSim parity;
- fixture replay;
- operational readiness;
- certification;
- regulated-use approval.
