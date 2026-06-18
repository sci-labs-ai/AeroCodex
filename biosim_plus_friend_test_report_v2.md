# BioSim-plus Friend-Test Report v2 Contract

Status: `research_required` draft report contract for Stage 5 prep.

Friend-test report v2 is intended to help outside testers read a deterministic, non-operational BioSim-plus scenario report after a future runner exists. This handoff does not add a runner, persistent CLI, parser, fixture loader, public API, or external BioSim parity command.

AeroCodex remains research and preliminary-design software. The report is not evidence of medical suitability, crew safety, habitat safety, operational readiness, certification, or regulated-use approval.

## Required report sections

A future report should include these sections in this order:

1. `Report identity`
2. `Clean-room boundary`
3. `Scenario contract`
4. `Replay summary`
5. `Resource ledger summary`
6. `Uncertainty and sensitivity hooks`
7. `Failure details`, if any
8. `Non-claims`
9. `Reproduction commands`, if a future command surface exists

## Required fields

| Field | Required | Notes |
| --- | --- | --- |
| `report_version` | yes | Must be `biosim_plus.friend_test_report.v2` for this contract. |
| `scenario_id` | yes | Stable clean-room scenario identifier. |
| `schema_version` | yes | Exact scenario schema version. |
| `status` | yes | Must be `research_required`. |
| `tick_count` | yes | Positive integer. |
| `tick_duration_seconds` | yes | Finite positive value. |
| `resource_keys` | yes | Canonical keys included in the replay. |
| `module_count` | yes | Count of deterministic module stubs. |
| `module_order_digest` | optional until runtime | Smoke-test digest of canonical module order. |
| `before_digest` | optional until runtime | Smoke-test digest of canonical initial state. |
| `after_digest` | optional until runtime | Smoke-test digest of canonical final state. |
| `ledger_passed` | yes when runtime exists | Boolean ledger summary, not a validation status. |
| `max_abs_residual` | yes when runtime exists | Numeric residual summary with unit context. |
| `sensitivity_state` | yes | Default is `declared_but_disabled`. |
| `non_claims` | yes | Must repeat explicit non-claim wording. |

## Example text skeleton

```text
BioSim-plus friend-test report v2
status: research_required
scenario_id: biosim_plus.example.fixed_resource_box.v0_1
schema_version: biosim_plus.scenario_contract.v0_1
report_version: biosim_plus.friend_test_report.v2

clean_room_boundary: no GPL Java BioSim code, GPL-bound BioSim-RS scaffold code,
external BioSim fixtures, or golden-master outputs were imported for this report.

tick_count: 3
tick_duration_seconds: 3600.0
resources: o2_gas, co2_gas, h2o_potable, h2o_waste, biomass_edible, food_stored, power_electrical
module_count: 2
sensitivity_state: declared_but_disabled

ledger_passed: not_run_docs_contract_only
max_abs_residual: not_run_docs_contract_only
replay_digest_algorithm: not_run_docs_contract_only

non_claims: not medical use; not habitat safety; not operational use; not
flight readiness; not mission readiness; not certification; not regulated-use
approval; not external BioSim parity.
```

## Report pass/fail vocabulary

A future report should use small stable labels:

| Label | Meaning |
| --- | --- |
| `passed_research_accounting_checks` | All implemented deterministic ledger checks passed under declared assumptions. |
| `failed_research_accounting_checks` | One or more deterministic ledger checks failed. |
| `not_run_docs_contract_only` | This handoff defines the report contract but no runner exists yet. |
| `blocked_by_source_boundary` | A fixture/source boundary issue blocks report generation. |
| `blocked_by_schema_error` | Scenario contract does not satisfy the schema subset. |

The labels are report states only. They do not upgrade validation status above `research_required`.

## Required non-claim text

Every future v2 report must contain wording equivalent to:

```text
This BioSim-plus report is research/preliminary-design evidence only. It is not
medical advice, crew-safety evidence, habitat-safety evidence, operational
approval, flight-readiness evidence, mission-readiness evidence, certification,
regulated-use approval, or external BioSim parity evidence.
```

## Future test expectations

A future implementation chunk should add tests that verify:

- report version and schema version are printed;
- status remains `research_required`;
- resource keys include O2, CO2, H2O, biomass, food, and power concepts;
- disabled uncertainty hooks are reported as disabled;
- ledger pass/fail fields are deterministic;
- missing non-claim text fails the report test;
- no external fixture, GPL source, or scaffold crate is required to print the report.
