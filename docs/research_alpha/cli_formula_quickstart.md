# Research Alpha formula CLI quickstart

<!-- aerocodex-current-identity:start -->
Release version: `0.1.0-alpha.1`
Release tier: `research_software_alpha` (`Research Software Alpha`)
Workspace packages: `14`
Registry formulas: `152`
Blocked formulas: `152`
Publicly executable formulas: `0`
<!-- aerocodex-current-identity:end -->

The RR-019 CLI namespace is:

```bash
aerocodex formula list [--json]
aerocodex formula describe <formula-id> [--json]
aerocodex formula status-report [--json]
aerocodex formula run <formula-id> [--preliminary] --input-name value ... [--json]
```

Use this namespace for new scripts. The older Beta 1 aliases remain available for compatibility:

```bash
aerocodex formulas [--json]
aerocodex describe <formula-id> [--json]
aerocodex run <formula-id> [--preliminary] name=value ... [--json]
```

When a legacy alias emits successful JSON, it includes `deprecated_alias=true` and a `migration_command` field. The alias behavior is intentionally migration-only; it does not promote formula status and does not certify or approve formula execution.

## List formulas

```bash
cargo run -p aero-codex-cli -- formula list
cargo run -p aero-codex-cli -- formula list --json
```

The list command reports the registry-backed formula inventory, including the bounded M00 concept entries, and includes `validation_status=research_required` plus the safety notice. JSON output uses canonical Formula Registry IDs while preserving the legacy ID in `legacy_formula_id`; list output is inventory metadata and does not make a row executable.

Legacy compatibility smoke:

```bash
cargo run -p aero-codex-cli -- formulas --json
```

## Formula status report

RR-026 adds a read-only registry-backed status report for humans and agents:

```bash
cargo run -p aero-codex-cli -- formula status-report
cargo run -p aero-codex-cli -- formula status-report --json > /tmp/status_report.json
python3 -m json.tool /tmp/status_report.json >/dev/null
```

The report summarizes the checked-in Formula Registry inventory without executing formulas or changing statuses. Human and JSON output include the total registry formula count, counts by status, counts by execution policy, by-family counts, normal executable count, preliminary-only count, blocked count, M07 candidate count, promotion candidate count, registry schema version, source hash, validation status, warnings, and the safety notice.

Blocked formulas are honest inventory entries rather than command errors. If the current registry has zero M07 candidates, zero promotion candidates, or zero default executable formulas, the report prints zero counts instead of fabricating examples or promoting rows.

## Describe formulas

Canonical registry ID:

```bash
cargo run -p aero-codex-cli -- formula describe \
  m00.canonical.distance_to_canonical --json
```

Legacy alias ID:

```bash
cargo run -p aero-codex-cli -- describe \
  formula_vault.m00.canonical.distance_to_canonical --json
```

Both forms resolve to the same canonical formula ID. Legacy JSON output records `alias_used`, `canonical_formula_id`, and `migration_command` for traceability.

RR-021 backs `formula describe` from the generated Formula Registry. Human output includes `formula_id`, `legacy_formula_id`, `family`, `status`, `execution_policy`, `quarantine_state`, `inputs`, `outputs`, `units`, `domain_constraints`, `implementation_path`, `contract_path`, `validation_card_path`, `source_seed_path`, and `warnings`. JSON output keeps the registry fields and adds `ok` plus `command`; legacy ID requests also include `alias_used` and `canonical_formula_id`.

Missing formula IDs fail closed with `formula_not_found`; JSON mode returns `ok=false` and exits nonzero.

The namespace can also describe checked-in registry rows that are not executable through the Beta 1 CLI surface:

```bash
cargo run -p aero-codex-cli -- formula describe \
  aerodynamics.coefficients.drag_coefficient --json
```

Such descriptions are inventory/status metadata only. Registry inclusion is not validation, status promotion, certification, execution approval, readiness approval, or regulatory approval.

## Run formulas

`formula run` accepts RR-022 scalar flag-style inputs whose names come from the generated Formula Registry, for example `--degrees 180`. Negative scalar values such as `--degrees -180` are values, not flags. Duplicate, missing, unexpected, invalid-number, mixed-syntax, and vector/array-shaped inputs fail closed with stable error handling before any runtime dispatch. The legacy `name=value` assignment syntax remains available for migration compatibility; successful legacy JSON runs include `"input_syntax":"legacy_assignment"`.

`formula run` is guarded by the RR-025 public-alpha execution status gate. The gate runs after formula ID resolution and scalar input parsing but before runtime dispatch.

Default public-alpha execution requires a registry status of `implementation_verified` or `reference_validated`. The current checked-in registry snapshot remains lower-status inventory: formulas are listable and describable, but `research_required` rows fail closed with `execution_blocked_by_status` and do not run through the public-alpha CLI.

A representative blocked run:

```bash
cargo run -p aero-codex-cli -- formula run \
  aerodynamics.coefficients.drag_coefficient --json
```

The task-card smoke command remains blocked by the status gate after the RR-022 parser accepts the scalar flag syntax. RR-023 wires the bounded M00 angle dispatch path for future promoted rows, but current `research_required` registry rows still do not execute through the public-alpha CLI:

```bash
cargo run -p aero-codex-cli -- formula run \
  m00.angle.deg_to_rad --degrees 180 --preliminary --json
```

When M00 angle rows are later promoted through governed status work, the wired dispatch uses `m00_degrees_to_radians` and `m00_radians_to_degrees` and reports `angle_radians` / `angle_degrees` with registry traceability. `equation_traceable` rows, when present, require `--preliminary` to pass the RR-025 status gate. `research_required` rows remain blocked even with `--preliminary`, and M07 candidates remain blocked until a later governed promotion task.

The legacy run alias remains accepted during migration, but it routes through the same gate and cannot bypass status policy:

```bash
cargo run -p aero-codex-cli -- run \
  formula_vault.m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
```

Rows that are present in the Formula Registry but below the execution threshold fail closed for `formula run` with a stable JSON error envelope rather than dispatching placeholder or future work.

## Safety and status posture

- `validation_status` remains `research_required`.
- The CLI is research/preliminary-design software only.
- RR-019 introduces command namespace and alias routing only.
- RR-023 wires only bounded M00 angle dispatch specs and keeps them behind RR-025 status gates while the registry rows remain `research_required`.
- RR-019/RR-023 do not change formula statuses, generated registry artifacts, runtime formula implementations, GitHub workflows, validation cards, equation batches, M07 quarantine/status data, or regulatory/safety claims.
