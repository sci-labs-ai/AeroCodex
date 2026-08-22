# Research Alpha formula CLI quickstart

<!-- aerocodex-current-identity:start -->
Release version: `0.1.0-alpha.1`
Release tier: `research_software_alpha` (`Research Software Alpha`)
Workspace packages: `14`
Registry formulas: `152`
Blocked formulas: `140`
Publicly executable formulas: `12`
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

The list command reports the registry-backed formula inventory and includes the release-slice validation status plus the safety notice. JSON output uses canonical Formula Registry IDs while preserving the legacy ID in `legacy_formula_id`. Add `--executable` to return exactly the twelve manifest-selected `implementation_verified` / `normal_research` M00 formulas; unfiltered list output remains inventory metadata.

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

The report summarizes the checked-in Formula Registry inventory without executing formulas or changing statuses. Human and JSON output compute registry, dispatchable, and executable counts separately, alongside counts by status, execution policy, and family; preliminary-only, blocked, M07-candidate, and promotion-candidate counts; registry schema version; source hash; release-slice validation status; warnings; and the safety notice.

Blocked formulas are honest inventory entries rather than command errors. The current report derives 152 registry rows, 12 dispatchable formulas, 12 publicly executable formulas, and 140 blocked formulas; empty categories remain explicit zero counts.

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

RR-021 backs `formula describe` from the generated Formula Registry. Human and JSON output explicitly distinguish `implemented`, `dispatchable`, `executable`, `validated`, and `blocked`, alongside `formula_id`, `legacy_formula_id`, `family`, `status`, `execution_policy`, `quarantine_state`, inputs, outputs, units, constraints, implementation and evidence paths, and warnings. `validated=true` is reserved for scientific reference/experiment validation; `implementation_verified` release rows report implemented and executable without overstating scientific validation.

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

Default public-alpha execution requires a registry status of `implementation_verified` or `reference_validated` plus an actual CLI dispatch specification. The current release slice contains twelve `implementation_verified` / `normal_research` formulas. The other 140 `research_required` rows fail closed with `execution_blocked_by_status` and do not dispatch.

A representative blocked run:

```bash
cargo run -p aero-codex-cli -- formula run \
  aerodynamics.coefficients.drag_coefficient --json
```

The release-slice smoke command executes the promoted angle-conversion path and returns radians, traceability, `implementation_verified`, and `normal_research` in the versioned JSON success envelope:

```bash
cargo run -p aero-codex-cli -- formula run \
  m00.angle.deg_to_rad --degrees 180 --json
```

The angle dispatch uses `m00_degrees_to_radians` and `m00_radians_to_degrees` and reports `angle_radians` / `angle_degrees` with registry traceability. `equation_traceable` rows, when present, require `--preliminary`; `research_required` rows remain blocked even with that flag, and M07 candidates remain blocked until a later governed promotion task.

The legacy run alias remains accepted during migration, but it routes through the same gate and cannot bypass status policy:

```bash
cargo run -p aero-codex-cli -- run \
  formula_vault.m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
```

Rows that are present in the Formula Registry but below the execution threshold fail closed for `formula run` with a stable JSON error envelope rather than dispatching placeholder or future work.

## Safety and status posture

- The release-slice `validation_status` is `implementation_verified`; the other 140 registry rows remain `research_required`.
- The CLI is research/preliminary-design software only.
- Only the twelve manifest-selected M00 formulas are promoted; broader families and M07 quarantine remain unchanged.
- `implementation_verified` does not mean scientifically reference-validated, certified, operational, flight-ready, mission-ready, or suitable for regulated use.
