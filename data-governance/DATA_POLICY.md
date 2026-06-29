# Data and Source Governance Policy

AeroCodex Stage 4 treats external archives, source snapshots, generated manifests, validation cards, and source-material directories as governed source material. Registration does not import external code into public crates and does not upgrade any research kernel to operational or certified status.

AeroCodex is research and preliminary-design software. It is not certified, not operational, not flight-ready, not habitat-safe, not medical-use software, and not approved for regulated use.

## Registry contract

The canonical Stage 4 registry is `data-governance/DATA_REGISTRY.yaml`. Its `validation_status` and `hash_status` fields use the vocabulary in `validation/status_vocabulary.yaml`. Each artifact entry records:

- `id`: stable artifact identifier.
- `title`: human-readable artifact name.
- `local_path`: repo-relative path for bundled artifacts, or a logical `external://stage4/...` path for external, not-bundled Stage 4 materials.
- `artifact_kind`: file, directory, archive, source bundle, generated inventory, validation-card set, or similar category.
- `origin`: source family or provenance lane.
- `license`: license or license-boundary status.
- `sha256`: file SHA256 or deterministic directory aggregate SHA256 when available.
- `hash_status`: how the hash was observed or why it is pending.
- `allowed_use`: explicit use boundary.
- `bundling_decision`: whether the artifact is bundled, quarantined, or excluded from public API import.
- `validation_status`: current validation or assurance status.
- `owner`: accountable maintainer group.
- `update_cadence`: expected refresh trigger.
- `notes`: extra boundary, certification, or evidence notes.

For directory entries, `sha256` is a deterministic aggregate over the sorted file list in that directory. The aggregate input is `repo-relative-path`, a null-byte separator, each file SHA256, and a newline per file. This is a governance fingerprint, not a replacement for per-file manifests.

## Source-boundary rules

- Do not import M07 code into public AeroCodex crates.
- Do not overwrite `crates/aero-codex-astrodynamics`.
- Do not import BioSim GPL code into the dual MIT/Apache AeroCodex core.
- Do not copy the Orekit class hierarchy or architecture class-for-class.
- Register external bundles as source material only until a future chunk explicitly changes their status.
- Preserve non-certification caveats in registry, docs, validation cards, and release notes.

## Schema validation status

JSON Schema validation is deferred for Chunk 1. The registry intentionally uses a dependency-free Rust verifier in `xtask` instead of adding third-party Rust or non-Rust scripting dependencies for this governance skeleton.

Run:

```text
cargo run -p xtask -- verify data-registry
cargo run -p xtask -- verify status-vocabulary
cargo run -p xtask -- verify --all
```

The verifier rejects malformed or empty entries, missing IDs, duplicate IDs, missing `local_path`, missing `license`, missing `allowed_use`, missing `bundling_decision`, missing `validation_status`, missing SHA256 values without `hash_status: pending_with_reason`, and unsafe direct external-archive import decisions into public crates.

## Promotion rule

A registered artifact may move toward public API implementation only after a later chunk supplies the required license decision, equation/source contract, tests, tolerances, validation evidence, source IDs, documentation, and release-gate evidence. Registry inclusion alone is not implementation approval.
