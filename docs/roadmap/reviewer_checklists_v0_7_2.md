# Research Readiness Reviewer Checklists v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Package_v0_7_2.zip` and the v0.7.2 task card for the PR's task ID. These checklists are review aids only; they do not enable branch protection, repository settings, formula execution, or validation-status promotion.

Use the checklist that matches the PR type, and also apply the general PR review to every RR/LOCK task.

## general PR review

- [ ] The PR maps to exactly one task ID unless a maintainer explicitly approved adjacent non-overlapping docs-only batching.
- [ ] The base branch is fresh `main`, and the branch/task name matches the assigned task ID.
- [ ] Files touched match the v0.7.2 task card, or the PR explains a small task-local discovery/linking reason.
- [ ] Files not to touch from the task card remain untouched.
- [ ] no forbidden readiness claims: no positive NASA-ready, flight-ready, mission-ready, certified-for-flight, life-support-certified, or regulatory-approved claims are introduced.
- [ ] no M07 execution enabled: M07 remains visible only as quarantined/blocked candidates unless a later family-promotion task explicitly changes that status.
- [ ] no validation status mass-promotion: validation/status rows are not promoted outside the formulas explicitly owned by the task.
- [ ] no unintended Cargo.toml or Cargo.lock changes.
- [ ] no generated artifacts unless the task owns them.
- [ ] tests/checks match the task card, including required greps, scope checks, docs checks, and `xtask verify --all` when requested.
- [ ] Handoff evidence names the task ID, files changed, commands run, results, stop conditions, and intentionally untouched forbidden scopes.

## contract/docs PR review

- [ ] The PR cites v0.7.2 as the controlling planning package and does not use older v0.3, v0.6, v0.7.0, or v0.7.1 files as controlling sources.
- [ ] Contract terms, task IDs, execution-wave wording, dependency order, and branch suggestions match the v0.7.2 task card and queue docs.
- [ ] Docs-only language does not imply branch protection, repository setting changes, CI setting changes, formula execution, or validation promotion.
- [ ] Public wording remains conservative: professional research/preliminary-design software, not operational or certified aerospace software.
- [ ] Links added to roadmap docs are small, discoverability-focused, and do not expand task scope.
- [ ] Rollback is localized to the contract/docs files owned by the task.

## generated-artifact PR review

- [ ] The task explicitly owns generated artifacts or golden outputs before any generated file is changed.
- [ ] The generator command, inputs, schema version, and source hashes are documented.
- [ ] Generated outputs are deterministic: stable ordering, no wall-clock timestamps, no machine-local paths, and no nondeterministic IDs.
- [ ] Required `.sha256` sidecars or manifests are present and match the generated files.
- [ ] Generated registry artifacts are not refreshed opportunistically by unrelated docs, parser, CLI, or status-gate tasks.
- [ ] Review includes a diff of the generated output plus the contract/source inputs that caused it.

## formula-promotion PR review

- [ ] source locator is present and precise enough for a reviewer to find the source material without guessing.
- [ ] formula identity is stated, including formula ID, aliases, family, and promotion scope.
- [ ] inputs/outputs/units are complete and align with sidecar/schema expectations.
- [ ] domain constraints are explicit, including blocked, unsupported, singular, or preliminary-only domains.
- [ ] implementation symbol is named and maps to the runtime/API function that will execute the formula.
- [ ] test vectors include inputs, expected outputs, units, tolerances, and source references.
- [ ] tolerance justification explains numeric precision, unit conversions, and acceptable error bounds.
- [ ] reviewer names/dates are recorded for source, implementation, and validation review.
- [ ] no unrelated formula status changes are included.
- [ ] M07 candidates remain blocked unless the task card is a family-specific M07 promotion task and all SOP evidence is present.
- [ ] Promotion packet, sidecar metadata, status changes, runtime implementation, tests, and generated artifacts are reviewed as one traceable chain when the task owns them.
