# BioSim-plus Deterministic Replay Policy

Status: `research_required` docs/contracts-only policy for Stage 5 prep.

This policy describes how a future BioSim-plus clean-room scenario layer should produce deterministic multi-tick replay evidence without importing GPL BioSim source, fixtures, or runtime outputs. It does not add a CLI, API, parser, fixture loader, or scenario executor in this chunk.

AeroCodex remains research and preliminary-design software. Passing replay checks must not be presented as physical validation, BioSim parity, biological validation, habitat safety, medical suitability, operational readiness, certification, or regulated-use approval.

## Replay input contract

A replay input is a single scenario contract document after schema validation. A future runner must treat the contract as immutable for one replay.

Required deterministic inputs:

- `scenario_id`;
- `schema_version`;
- tick count;
- tick duration;
- ordered module list;
- resource key table;
- initial stores;
- module-stub deltas or deterministic schedules;
- ledger tolerance policy;
- optional sensitivity case identifier, if explicitly selected.

Blocked implicit inputs:

- wall-clock time;
- host locale;
- filesystem iteration order;
- network access;
- environment-variable behavior not recorded in the report;
- hidden random state;
- external BioSim fixtures or golden-master outputs without a reviewed fixture manifest.

## Canonical ordering

A future replay report must canonicalize these fields before digest generation:

1. scenario metadata by field name;
2. resource keys by lowercase key;
3. stores by resource key, then store ID;
4. modules by explicit `module_order` integer, then module ID as a tie-breaker if needed;
5. tick records by tick index;
6. per-tick deltas by module order, resource key, and store ID;
7. ledger rows by tick index, resource key, canonical unit, and store ID.

The ordering rule is part of the contract. A report that depends on hash-map iteration order or source-document mapping order is out of domain.

## Numeric serialization

A future digest input must use a stable numeric serialization policy. The recommended contract is:

- reject NaN and infinity;
- serialize integers in base 10 without separators;
- serialize finite floats using an explicitly documented round-trip representation;
- include canonical units next to every numeric resource amount;
- include the tolerance value and tolerance unit in the report.

This docs-only chunk does not choose a Rust formatting implementation. The deployment agent should keep numeric serialization as a future implementation decision.

## Digest policy

Digest fields are smoke-test evidence for deterministic replay only. They are not cryptographic signatures, not anti-tamper mechanisms, not persistent ledger keys, and not validation against external BioSim outputs.

A future implementation may reuse the existing dependency-free digest style if the implementation chunk explicitly scopes that choice. The report must include an algorithm label so the digest can be regenerated later.

## Multi-tick replay semantics

For each tick:

1. Validate the previous state.
2. Evaluate module-stub deltas in explicit module order.
3. Stage all deltas for the tick.
4. Reject the tick if any staged result is out of domain.
5. Commit the full tick atomically.
6. Emit before/after digest fields.
7. Emit ledger rows and residuals.
8. Continue to the next tick only if the contract's failure policy allows continuation.

The default failure policy should stop at the first invalid tick and emit a failed report. A permissive reporting mode may be designed later, but it must not hide or repair invalid states.

## Sensitivity and uncertainty hooks

Sensitivity hooks are deterministic when enabled. A future runner may support:

- named deterministic cases;
- bounded sweeps with explicit start, stop, and step values;
- seeded pseudo-random draws with a declared algorithm, seed, and sample index.

Default replay must report hooks as `declared_but_disabled` unless a case is selected. A disabled hook must not affect digests, ledger rows, or pass/fail status.

## Friend-test report v2 reproducibility

Friend-test report v2 should be reproducible from the same scenario contract and replay policy. It should include:

- scenario ID;
- schema version;
- report version;
- status;
- tick count and duration;
- resource keys;
- module count and order;
- ledger pass/fail summary;
- digest algorithm labels and digest values if implemented;
- sensitivity hook status;
- clean-room boundary statement;
- non-claims.

A future friend-test command may be added only in a separate deployment chunk. This handoff does not add one.

## Required negative cases for a future implementation

A future implementation chunk should include tests that reject or fail reports for:

- duplicate resource keys;
- unsupported schema version;
- tick duration equal to zero;
- negative tick duration;
- NaN or infinity in amounts or tolerances;
- duplicate store IDs within a resource;
- unit mismatch;
- hidden random hook without selected seed;
- negative after-store balance;
- residual outside tolerance;
- missing non-claim text in report output.

## Promotion gate

The deployment agent should merge this policy only as Stage 5 prep documentation and metadata. Do not add scenario execution, fixture import, parser crates, dependencies, or public APIs in the same chunk unless a later prompt explicitly scopes them.
