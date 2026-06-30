# Formula Registry Fixtures and Golden Outputs v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md`, task `LOCK-003`.

This document fixes fixture and golden-output paths before parser, registry, verifier, and CLI implementation tasks rely on them. It documents paths only; it does not create fixtures, generated registries, runtime code, or formula execution behavior.

## Path ownership rule

Parser, registry, verifier, and CLI tasks must use the paths below unless a later explicit contract task updates this document. Agents must not add duplicate fixture trees, alternate golden-output roots, or task-local paths that bypass these contracts.

## Equation-batch parser fixtures

```text
tests/fixtures/equation_batch/manifest_valid_minimal.tsv
tests/fixtures/equation_batch/manifest_invalid_missing_formula_id.tsv
tests/fixtures/equation_batch/manifest_invalid_status.tsv
tests/fixtures/equation_batch/manifest_m00_slice_a.tsv
tests/golden/equation_batch/manifest_valid_minimal_plan.json
tests/golden/equation_batch/manifest_m00_slice_a_report.json
```

Parser tests must use these repository-relative paths when checking manifest loading, required-field validation, status vocabulary checks, and deterministic plan/report output.

## Formula registry fixtures

```text
tests/fixtures/formula_registry/m00/deg_to_rad_sidecar.yaml
tests/fixtures/formula_registry/m00/rad_to_deg_sidecar.yaml
tests/fixtures/formula_registry/m00/angle_normalization_sidecar.yaml
tests/fixtures/formula_registry/m07/blocked_candidate_sidecar.yaml
tests/golden/formula_registry/formula_registry_m00_slice_a.json
tests/golden/formula_registry/formula_registry_m00_slice_a.sha256
tests/golden/formula_registry/formula_registry_with_m07_blocked_candidate.json
```

Registry tests must preserve sorted formula IDs, sorted aliases, declared input/output order, sorted test vectors, and deterministic `.sha256` output.

## CLI golden outputs

```text
tests/golden/cli/formula_list_human.txt
tests/golden/cli/formula_list_json.json
tests/golden/cli/formula_describe_deg_to_rad_human.txt
tests/golden/cli/formula_describe_deg_to_rad_json.json
tests/golden/cli/formula_run_deg_to_rad_human.txt
tests/golden/cli/formula_run_deg_to_rad_json.json
tests/golden/cli/formula_run_blocked_m07_json.json
tests/golden/cli/formula_status_report_json.json
```

The canonical CLI JSON run fixture ID is `formula_run_deg_to_rad_json` and its golden path is:

```text
tests/golden/cli/formula_run_deg_to_rad_json.json
```

CLI JSON golden outputs must be stable, repository-relative, and free of wall-clock timestamps. Human-output golden files may preserve fixed line order but must not include machine-local paths or environment-specific values.

## Future implementation guard

A downstream parser, verifier, registry, or CLI task that needs a new fixture or golden output must update this contract first, or cite the later task that updates it. Until then, these paths are the only approved fixture/golden roots for research-readiness formula infrastructure.
