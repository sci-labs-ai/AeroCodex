# Historical RR-055 Fixture and Reserved-Output Path Contract

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md`, task `LOCK-003`.

This document records the historical RR-055 path reservation that preceded the implemented parser, registry, verifier, and CLI. The reserved outputs have been moved under `tests/fixtures/historical/rr055_reserved_outputs/` so they cannot be mistaken for current golden contracts.

No current test consumes these historical outputs. Current generated state is governed under `generated/`, and current CLI behavior is verified by executable integration tests. Future live golden contracts require a separate reviewed contract and must not reuse these placeholders as approval evidence.

## Path ownership rule

The input fixtures below remain active test inputs. The reserved-output paths are retained only as historical fixtures and must not be used as current approval or execution oracles.

## Equation-batch parser fixtures

```text
tests/fixtures/equation_batch/manifest_valid_minimal.tsv
tests/fixtures/equation_batch/manifest_invalid_missing_formula_id.tsv
tests/fixtures/equation_batch/manifest_invalid_status.tsv
tests/fixtures/equation_batch/manifest_m00_slice_a.tsv
tests/fixtures/historical/rr055_reserved_outputs/equation_batch/manifest_valid_minimal_plan.json
tests/fixtures/historical/rr055_reserved_outputs/equation_batch/manifest_m00_slice_a_report.json
```

Parser tests use the active `tests/fixtures/equation_batch/` inputs. The two reserved outputs are historical RR-055 snapshots and are not current expected-output contracts.

## Formula registry fixtures

```text
tests/fixtures/formula_registry/m00/deg_to_rad_sidecar.yaml
tests/fixtures/formula_registry/m00/rad_to_deg_sidecar.yaml
tests/fixtures/formula_registry/m00/angle_normalization_sidecar.yaml
tests/fixtures/formula_registry/m07/blocked_candidate_sidecar.yaml
tests/fixtures/historical/rr055_reserved_outputs/formula_registry/formula_registry_m00_slice_a.json
tests/fixtures/historical/rr055_reserved_outputs/formula_registry/formula_registry_m00_slice_a.sha256
tests/fixtures/historical/rr055_reserved_outputs/formula_registry/formula_registry_with_m07_blocked_candidate.json
```

Registry tests use the active sidecar fixtures. The three reserved outputs are historical RR-055 snapshots and are not the generated registry authority.

## Historical CLI reserved outputs

```text
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_list_human.txt
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_list_json.json
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_describe_deg_to_rad_human.txt
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_describe_deg_to_rad_json.json
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_run_deg_to_rad_human.txt
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_run_deg_to_rad_json.json
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_run_blocked_m07_json.json
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_status_report_json.json
```

The historical RR-055 JSON run reservation was:

```text
tests/fixtures/historical/rr055_reserved_outputs/cli/formula_run_deg_to_rad_json.json
```

These files remain deterministic and free of machine-local data, but they are not current CLI output promises and contain no execution evidence.

## Future implementation guard

A downstream task that needs a live golden output must create a new, separately reviewed contract tied to executable tests. It must not silently promote a file from the historical RR-055 directory.
