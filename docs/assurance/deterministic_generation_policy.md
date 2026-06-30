# Deterministic Generation Policy v0.7.2

Source of truth: `AeroCodex_Research_Readiness_Master_Execution_Plan_v0_7_2.md`, task `LOCK-003`.

This policy fixes deterministic generation rules for future registry and inventory artifacts. It is docs-only; it does not generate files or change formula/runtime behavior.

## Covered paths

The policy applies to checked-in generated artifacts and their source contracts, including:

```text
schemas/*.schema.json
formula-schemas/**
promotion-packets/**
generated/*.json
generated/*.sha256
generated/formula_registry.json
generated/formula_registry.sha256
generated/runtime_equation_inventory.json
generated/runtime_equation_inventory.sha256
generated/m07_candidate_registry.json
generated/m07_candidate_registry.sha256
crates/aero-codex-registry/src/generated.rs
```

## Required deterministic rules

Generators must be reproducible from the declared repository inputs:

- No wall-clock timestamps in checked-in generated files.
- No `generated_at`, local timezone, hostname, username, temp directory, cache path, or absolute machine-local path in checked-in generated files.
- Sort formulas by canonical `formula_id`.
- Sort aliases by alias ID.
- Preserve declared input order.
- Preserve declared output order.
- Sort test vectors by test vector ID.
- Sort map/object keys where the serializer supports stable ordering.
- Include `schema_version`.
- Include `generator_version`.
- Include `source_hash` for source-content digesting.
- Include `build_input_hash` for the complete declared generator input set.
- Emit `.sha256` files for checked-in generated JSON artifacts.
- Treat changed generated output without changed source inputs as a generator or ordering defect.

## Required hash behavior

- Hashes must use SHA-256.
- Hash inputs must be declared and stable.
- Hash input path ordering must be lexicographic and repository-relative.
- Hashes must not include generated file modification time.
- Hashes must not include Git checkout location, user home paths, credentials, auth state, or platform cache directories.
- Sidecar `.sha256` files must reference repository-relative artifact paths.

## Registry stale-check expectation

After generator tasks exist, a stale-check command must fail if checked-in generated artifacts differ from their declared inputs. The expected generated registry paths are:

```text
generated/formula_registry.json
generated/formula_registry.sha256
crates/aero-codex-registry/src/generated.rs
```

The future check may be wired through `cargo run -p xtask -- formula-registry generate --check`, but this LOCK-003 task only documents the path and determinism contract.

## Review rules

Reviewers should reject generated artifacts when they contain:

- wall-clock timestamps;
- nondeterministic ordering;
- host-local paths;
- credentials or auth-state material;
- a registry path outside the approved `generated/*.json`, `generated/*.sha256`, or `crates/aero-codex-registry/src/generated.rs` locations;
- formula status promotion not explicitly assigned to the current task.
