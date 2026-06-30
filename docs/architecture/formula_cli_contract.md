# Formula CLI Contract v0.7.2

Source of truth: AeroCodex Research Readiness Master Plan v0.7.2, task LOCK-004.

AeroCodex formula commands are professional research/preliminary-design interfaces. They do not certify formulas for operational aerospace, flight, habitat-safety, or life-support use. Human-readable output is the default. Stable JSON is available with `--json` on every canonical command below.

## Canonical command namespace

The public alpha formula namespace is `aerocodex formula`. Implementation tasks must not introduce an alternate public namespace for formula inventory or execution.

```bash
aerocodex formula list
aerocodex formula list --json
aerocodex formula describe m00.angle.deg_to_rad
aerocodex formula describe m00.angle.deg_to_rad --json
aerocodex formula run m00.angle.deg_to_rad --degrees 180
aerocodex formula run m00.angle.deg_to_rad --degrees 180 --json
aerocodex formula status-report
aerocodex formula status-report --json
```

Legacy Beta 1 commands such as `formulas`, `describe`, and `run` are deprecated aliases only. Once registry/status-gate routing exists, they must route through the same generated registry lookup, alias handling, and status-gate path as `aerocodex formula ...`, or fail with a clear migration message. Legacy aliases must never bypass status gates or imply normal execution for `research_required`, M07-candidate, or otherwise blocked formulas.

## Status gate summary

- Inventory commands may show `research_required` formulas.
- Normal execution requires `implementation_verified` or higher.
- `equation_traceable` formulas require explicit `--preliminary` and must not run in normal mode.
- M07 candidates are visible as blocked candidates and must not execute through the public alpha CLI until promoted family by family.
- Ambiguous registry, status, quarantine, alias, runtime-symbol, or schema states fail closed.

## Human output examples

### List

```text
AeroCodex formula inventory

ID                         Status                   Execution policy              Summary
m00.angle.deg_to_rad        implementation_verified  normal_research              Degrees to radians
m00.angle.rad_to_deg        implementation_verified  normal_research              Radians to degrees
m07.candidate.some_formula  research_required        blocked                      Visible M07 candidate; execution blocked

Notice: Research/preliminary-design software. Not certified for operational use.
```

### Describe

```text
Formula: m00.angle.deg_to_rad
Name: Degrees to radians
Status: implementation_verified
Execution policy: normal_research
Quarantine state: none

Inputs:
  --degrees <number>    angle in degrees

Outputs:
  angle_radians         angle in radians

Traceability:
  Source: M00 canonical angle identity
  Validation: implementation_verified

Notice: Research/preliminary-design software. Not certified for operational use.
```

### Successful run

```text
Formula: m00.angle.deg_to_rad
Status: implementation_verified
Execution policy: normal_research

Inputs:
  degrees = 180 deg

Outputs:
  angle_radians = 3.141592653589793 rad

Traceability:
  Source: M00 canonical angle identity
  Validation: implementation_verified

Notice: Research/preliminary-design software. Not certified for operational use.
```

### Blocked run

```text
Formula: m07.candidate.some_formula
Status: research_required
Execution policy: blocked
Quarantine state: m07_candidate_blocked

Error: m07_candidate_blocked
Formula is an M07 candidate and is visible but non-executable until promoted.

Notice: Research/preliminary-design software. Not certified for operational use.
```

### Unknown formula

```text
Error: formula_not_found
Formula ID "m99.unknown.formula" does not exist in the generated registry.
```

## JSON success envelope

JSON envelopes must be stable enough for golden tests. A successful formula run uses this shape:

```json
{
  "ok": true,
  "command": "formula_run",
  "formula_id": "m00.angle.deg_to_rad",
  "canonical_formula_id": "m00.angle.deg_to_rad",
  "alias_used": null,
  "status": "implementation_verified",
  "execution_policy": "normal_research",
  "quarantine_state": "none",
  "inputs": {
    "degrees": {"value": 180.0, "unit": "deg"}
  },
  "outputs": {
    "angle_radians": {"value": 3.141592653589793, "unit": "rad"}
  },
  "traceability": {
    "source_ref": "M00 canonical angle identity",
    "validation_status": "implementation_verified",
    "registry_schema_version": "aerocodex.formula_registry.v1"
  },
  "warnings": [
    "Research/preliminary-design software. Not certified for operational use."
  ],
  "error": null
}
```

## JSON failure envelopes

Failures must set `ok` to `false`, include the same formula/status context when known, and include `error.code` plus `error.message`. Unknown formulas use `formula_not_found`.

### Blocked by status

```json
{
  "ok": false,
  "command": "formula_run",
  "formula_id": "a4.example.research_required_formula",
  "canonical_formula_id": null,
  "alias_used": null,
  "status": "research_required",
  "execution_policy": "blocked",
  "quarantine_state": "below_execution_threshold",
  "inputs": {},
  "outputs": {},
  "traceability": null,
  "warnings": [
    "Inventory visibility is not execution permission."
  ],
  "error": {
    "code": "execution_blocked_by_status",
    "message": "Formula is research_required and cannot run in normal research mode."
  }
}
```

### Preliminary flag required

```json
{
  "ok": false,
  "command": "formula_run",
  "formula_id": "a4.example.equation_traceable_formula",
  "canonical_formula_id": "a4.example.equation_traceable_formula",
  "alias_used": null,
  "status": "equation_traceable",
  "execution_policy": "preliminary_flag_required",
  "quarantine_state": "below_execution_threshold",
  "inputs": {},
  "outputs": {},
  "traceability": {"validation_status": "equation_traceable"},
  "warnings": [],
  "error": {
    "code": "preliminary_flag_required",
    "message": "Formula is equation_traceable and requires explicit --preliminary execution mode."
  }
}
```

### M07 candidate blocked

```json
{
  "ok": false,
  "command": "formula_run",
  "formula_id": "m07.candidate.some_formula",
  "canonical_formula_id": "m07.candidate.some_formula",
  "alias_used": null,
  "status": "research_required",
  "execution_policy": "blocked",
  "quarantine_state": "m07_candidate_blocked",
  "inputs": {},
  "outputs": {},
  "traceability": null,
  "warnings": [
    "M07 remains visible but blocked until promoted family by family."
  ],
  "error": {
    "code": "m07_candidate_blocked",
    "message": "Formula is an M07 candidate and is visible but non-executable until promoted."
  }
}
```

### Unknown formula

```json
{
  "ok": false,
  "command": "formula_run",
  "formula_id": "m99.unknown.formula",
  "canonical_formula_id": null,
  "alias_used": null,
  "status": null,
  "execution_policy": null,
  "quarantine_state": null,
  "inputs": {},
  "outputs": {},
  "traceability": null,
  "warnings": [],
  "error": {
    "code": "formula_not_found",
    "message": "Formula ID does not exist in the generated registry."
  }
}
```

## Public error-code rule

The canonical public error-code catalog is `docs/assurance/formula_error_code_catalog.csv`. New public error codes require updating that catalog, CLI golden tests, and the relevant docs in the same task or an explicitly approved contract update.
