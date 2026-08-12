# Research Alpha JSON contract

RR-024 defines the stable JSON envelope used by the AeroCodex research-alpha CLI. Human-readable output remains the default. JSON is emitted only when `--json` is supplied.

AeroCodex JSON is for agents, notebooks, regression tests, and later APIs. It is not a validation-status promotion, execution approval, certification, flight-readiness claim, mission-readiness claim, habitat-safety claim, life-support claim, NASA approval, or regulatory approval.

## Envelope rules

- Field names are snake_case.
- Success responses set `ok=true` and `error=null`.
- Error responses set `ok=false` and populate `error.code` plus `error.message`.
- `command` is the stable command label, for example `formula_list`, `formula describe`, `formula run`, `describe`, `run`, `version`, or `self-check`.
- `formula_id` is present when the command or error can identify a formula.
- `status`, `execution_policy`, and `quarantine_state` preserve registry/status meanings. They do not imply execution approval.
- `registry_schema_version` identifies the generated Formula Registry schema used by the CLI when registry metadata is involved.
- `warnings` is always an array on success responses.
- `safety_notice` is present on success and error responses.
- Legacy aliases preserve traceability with `deprecated_alias=true`, `migration_command`, `requested_formula_id`, `alias_used`, and `canonical_formula_id` where applicable.

## Success envelope: formula list

Command:

```bash
cargo run -p aero-codex-cli -- formula list --json
```

Shape:

```json
{
  "ok": true,
  "command": "formula_list",
  "count": 152,
  "registry_formula_count": 152,
  "registry_schema_version": "aerocodex.formula_registry.v1",
  "source_hash": "sha256:...",
  "filters": {
    "family": null,
    "status": null,
    "executable": false
  },
  "validation_status": "research_required",
  "formulas": [
    {
      "formula_id": "m00.canonical.distance_to_canonical",
      "legacy_formula_id": "formula_vault.m00.canonical.distance_to_canonical",
      "aliases": ["formula_vault.m00.canonical.distance_to_canonical"],
      "name": "Distance To Canonical",
      "status": "research_required",
      "execution_policy": "blocked",
      "quarantine_state": "below_execution_threshold",
      "family": "m00",
      "registry_family": "m00.canonical",
      "batch_id": "m00-canonical-units",
      "output": "canonical_distance",
      "output_variable": "canonical_distance",
      "outputs": ["canonical_distance"],
      "runtime_symbol": "m00_distance_to_canonical",
      "implementation_path": null,
      "runtime_label": "m00_distance_to_canonical",
      "inputs": []
    }
  ],
  "warnings": [
    "Research/preliminary-design JSON contract; status and registry fields are not certification or execution approval."
  ],
  "safety_notice": "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use",
  "error": null
}
```

## Success envelope: formula describe

Command:

```bash
cargo run -p aero-codex-cli -- formula describe m00.canonical.distance_to_canonical --json
```

Shape:

```json
{
  "ok": true,
  "command": "formula describe",
  "canonical_formula_id": "m00.canonical.distance_to_canonical",
  "requested_formula_id": "m00.canonical.distance_to_canonical",
  "alias_used": null,
  "contract_path": "formula-vault/contracts/m00_canonical_unit_conversions_contract.yaml",
  "validation_card_path": "validation/cards/validation_formula_vault_m00_canonical_unit_conversions.yaml",
  "source_seed_path": "validation/source_registry/source_formula_vault_m00_canonical_unit_conversions.yaml",
  "formula_id": "m00.canonical.distance_to_canonical",
  "legacy_formula_id": "formula_vault.m00.canonical.distance_to_canonical",
  "aliases": ["formula_vault.m00.canonical.distance_to_canonical"],
  "name": "Distance To Canonical",
  "summary": "Generated Formula Registry v1 inventory entry for research/preliminary-design traceability; execution remains controlled by status gates.",
  "family": "m00.canonical",
  "batch_id": "m00-canonical-units",
  "status": "research_required",
  "quarantine_state": "below_execution_threshold",
  "execution_policy": "blocked",
  "source_trace": {
    "contract_path": "formula-vault/contracts/m00_canonical_unit_conversions_contract.yaml",
    "manifest_line": "6",
    "manifest_path": "equation-batches/m00-canonical-units.tsv",
    "source_formula_id": "formula_vault.m00.canonical.distance_to_canonical",
    "source_seed_path": "validation/source_registry/source_formula_vault_m00_canonical_unit_conversions.yaml",
    "validation_card_path": "validation/cards/validation_formula_vault_m00_canonical_unit_conversions.yaml"
  },
  "inputs": [],
  "outputs": [
    {"name": "canonical_distance", "type": "f64"}
  ],
  "units": null,
  "domain_constraints": [],
  "implementation_path": {
    "crate_name": "aero_codex_astrodynamics",
    "output_variable": "canonical_distance",
    "package": "aero-codex-astrodynamics",
    "runtime_symbol": "m00_distance_to_canonical",
    "test_strategy": "exact"
  },
  "runtime_symbol": "m00_distance_to_canonical",
  "test_vectors": [],
  "warnings": [
    "Registry inclusion does not make formulas executable or promote validation status."
  ],
  "registry_schema_version": "aerocodex.formula_registry.v1",
  "source_hash": "sha256:...",
  "validation_status": "research_required",
  "safety_notice": "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use",
  "error": null
}
```

Legacy describe keeps the same envelope and adds alias fields:

```json
{
  "ok": true,
  "command": "describe",
  "deprecated_alias": true,
  "migration_command": "aerocodex formula describe <formula-id>",
  "canonical_formula_id": "m00.canonical.distance_to_canonical",
  "requested_formula_id": "formula_vault.m00.canonical.distance_to_canonical",
  "alias_used": "formula_vault.m00.canonical.distance_to_canonical",
  "error": null
}
```

## Success envelope: formula run

Command:

```bash
cargo run -p aero-codex-cli -- formula run m00.canonical.distance_to_canonical distance=-42 distance_unit=7 --json
```

Shape:

```json
{
  "ok": true,
  "command": "formula run",
  "formula_id": "m00.canonical.distance_to_canonical",
  "canonical_formula_id": "m00.canonical.distance_to_canonical",
  "requested_formula_id": "m00.canonical.distance_to_canonical",
  "alias_used": null,
  "legacy_formula_id": "formula_vault.m00.canonical.distance_to_canonical",
  "runtime_symbol": "m00_distance_to_canonical",
  "output_variable": "canonical_distance",
  "value": -6,
  "output": "canonical_distance",
  "units": null,
  "status": "research_required",
  "execution_policy": "blocked",
  "quarantine_state": "below_execution_threshold",
  "source_trace": {
    "contract_path": "formula-vault/contracts/m00_canonical_unit_conversions_contract.yaml",
    "manifest_line": "6",
    "manifest_path": "equation-batches/m00-canonical-units.tsv",
    "source_formula_id": "formula_vault.m00.canonical.distance_to_canonical",
    "source_seed_path": "validation/source_registry/source_formula_vault_m00_canonical_unit_conversions.yaml",
    "validation_card_path": "validation/cards/validation_formula_vault_m00_canonical_unit_conversions.yaml"
  },
  "registry_schema_version": "aerocodex.formula_registry.v1",
  "source_hash": "sha256:...",
  "validation_status": "research_required",
  "warnings": [
    "Registry inclusion does not make formulas executable or promote validation status."
  ],
  "safety_notice": "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use",
  "error": null
}
```

`value` is the scalar machine-readable result for the bounded Beta 1 canonical-unit surface. `output` and `output_variable` name that scalar. Future vector/object results must preserve the same envelope and add a stable result container rather than changing existing field meanings.

## Error envelope

Unknown formula:

```bash
cargo run -p aero-codex-cli -- formula describe no.such --json
```

Shape on stderr:

```json
{
  "ok": false,
  "command": "formula describe",
  "formula_id": "no.such",
  "status": null,
  "execution_policy": null,
  "error": {
    "code": "formula_not_found",
    "message": "unknown formula id `no.such`"
  },
  "release_channel": "beta1-concept",
  "validation_status": "research_required",
  "safety_notice": "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use"
}
```

Blocked execution:

```bash
cargo run -p aero-codex-cli -- formula run aerodynamics.coefficients.drag_coefficient --json
```

Shape on stderr:

```json
{
  "ok": false,
  "command": "formula run",
  "formula_id": "aerodynamics.coefficients.drag_coefficient",
  "status": "research_required",
  "execution_policy": "blocked",
  "error": {
    "code": "execution_blocked_by_status",
    "message": "formula `aerodynamics.coefficients.drag_coefficient` execution is blocked by status `research_required` with execution_policy `blocked`"
  },
  "release_channel": "beta1-concept",
  "validation_status": "research_required",
  "safety_notice": "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use"
}
```

Known stable error codes in the current CLI surface:

- `usage_error`
- `formula_not_found`
- `invalid_assignment`
- `duplicate_input`
- `invalid_number`
- `missing_input`
- `unexpected_input`
- `execution_blocked_by_status`
- `non_positive_input`
- `out_of_domain`
- `numerical_failure`
- `self_check_failed`

## Status/report envelope

RR-024 does not add a separate `status-report` command. The current CLI status/report JSON surface is `self-check --json`; any future status-report command must preserve the same success/error envelope fields.

Command:

```bash
cargo run -p aero-codex-cli -- self-check --json
```

Shape:

```json
{
  "ok": true,
  "command": "self-check",
  "release_channel": "beta1-concept",
  "supported_formula_count": 12,
  "passed": 14,
  "failed": 0,
  "checks": [
    {
      "name": "overflow_is_rejected",
      "formula_id": "formula_vault.m00.canonical.distance_from_canonical",
      "passed": true,
      "detail": "expected_error=numerical_failure observed_error=numerical_failure"
    }
  ],
  "registry_schema_version": "aerocodex.formula_registry.v1",
  "validation_status": "research_required",
  "warnings": [
    "Research/preliminary-design JSON contract; status and registry fields are not certification or execution approval."
  ],
  "safety_notice": "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use",
  "error": null
}
```

If any self-check row fails, the same command emits the report once with an error object instead of `error=null`:

```json
{
  "ok": false,
  "command": "self-check",
  "passed": 13,
  "failed": 1,
  "checks": [
    {
      "name": "example_failed_check",
      "formula_id": "formula_vault.m00.canonical.distance_to_canonical",
      "passed": false,
      "detail": "example failure detail"
    }
  ],
  "registry_schema_version": "aerocodex.formula_registry.v1",
  "validation_status": "research_required",
  "warnings": [
    "Research/preliminary-design JSON contract; status and registry fields are not certification or execution approval."
  ],
  "safety_notice": "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use",
  "error": {
    "code": "self_check_failed",
    "message": "Beta 1 self-check reported 1 failing checks"
  }
}
```

The failure report is the only JSON object emitted for a failing `self-check --json` invocation; the generic JSON error printer is not appended a second time.

## Semantic checks

Before handoff, validate representative success and error JSON with `python3 -m json.tool` and run the Rust tests. Representative checks are:

```bash
cargo run -p aero-codex-cli -- formula list --json > /tmp/acx_list.json
python3 -m json.tool /tmp/acx_list.json >/dev/null
cargo run -p aero-codex-cli -- formula describe no.such --json 2> /tmp/acx_err.json || true
python3 -m json.tool /tmp/acx_err.json >/dev/null
```

Also keep the required gates green:

```bash
cargo fmt --check
cargo test -p aero-codex-cli
cargo test --all
```
