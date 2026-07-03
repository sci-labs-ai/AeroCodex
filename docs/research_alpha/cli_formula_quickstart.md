# Research Alpha formula CLI quickstart

The RR-019 CLI namespace is:

```bash
aerocodex formula list [--json]
aerocodex formula describe <formula-id> [--json]
aerocodex formula run <formula-id> name=value ... [--json]
```

Use this namespace for new scripts. The older Beta 1 aliases remain available for compatibility:

```bash
aerocodex formulas [--json]
aerocodex describe <formula-id> [--json]
aerocodex run <formula-id> name=value ... [--json]
```

When a legacy alias emits successful JSON, it includes `deprecated_alias=true` and a `migration_command` field. The alias behavior is intentionally migration-only; it does not promote formula status and does not certify or approve formula execution.

## List formulas

```bash
cargo run -p aero-codex-cli -- formula list
cargo run -p aero-codex-cli -- formula list --json
```

The list command reports the ten bounded Beta 1 executable concept formulas and includes `validation_status=research_required` plus the safety notice. JSON output uses canonical Formula Registry IDs while preserving the legacy ID in `legacy_formula_id`.

Legacy compatibility smoke:

```bash
cargo run -p aero-codex-cli -- formulas --json
```

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

Both forms resolve to the same canonical formula ID. Legacy JSON output records `alias_used` and `migration_command` for traceability.

The namespace can also describe checked-in registry rows that are not executable through the Beta 1 CLI surface:

```bash
cargo run -p aero-codex-cli -- formula describe \
  aerodynamics.coefficients.drag_coefficient --json
```

Such descriptions are inventory/status metadata only. Registry inclusion is not validation, status promotion, certification, execution approval, readiness approval, or regulatory approval.

## Run formulas

Use the namespaced form for executable Beta 1 formulas:

```bash
cargo run -p aero-codex-cli -- formula run \
  m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
```

The legacy run alias remains accepted during migration:

```bash
cargo run -p aero-codex-cli -- run \
  formula_vault.m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
```

Rows that are present in the Formula Registry but do not map to the bounded executable Beta 1 surface fail closed for `formula run` with `execution_blocked_by_status` rather than dispatching placeholder or future work.

## Safety and status posture

- `validation_status` remains `research_required`.
- The CLI is research/preliminary-design software only.
- RR-019 introduces command namespace and alias routing only.
- RR-019 does not change formula statuses, generated registry artifacts, runtime formula implementations, GitHub workflows, validation cards, equation batches, M07 quarantine/status data, or regulatory/safety claims.
