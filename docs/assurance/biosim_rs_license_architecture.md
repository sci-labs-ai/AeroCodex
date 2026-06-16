# BioSim-RS License-Bound Architecture

Stage 4 Chunk 4 defines the BioSim-RS license and architecture boundary before any implementation slice is promoted. It is governance and documentation only. It does not import Java BioSim code, does not import the BioSim-RS bootstrap scaffold, does not add AeroCodex public APIs, and does not change the current dual `MIT OR Apache-2.0` core license.

AeroCodex remains research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved. This document is engineering governance and is not legal advice.

## Registered source artifacts

The governing source IDs are registered in `data-governance/DATA_REGISTRY.yaml`:

| Source ID | Role | License boundary | Current use |
| --- | --- | --- | --- |
| `stage4.biosim_rs_bootstrap.2026_06_15` | Rust-native BioSim-RS scaffold and migration docs | GPL-3.0-or-later boundary in the registered scaffold | Reference and rewrite planning material only |
| `stage4.biosim_java_reference.2026_06_15` | Original Java BioSim source reference | GPL-3.0 boundary in the registered Java source | GPL-boundaried reference and clean-rewrite planning material only |

The archives stay outside the repository under logical `external://stage4/...` paths. Their presence in the registry is not permission to copy implementation code into the AeroCodex crates.

## License path decision tree

BioSim-RS work must choose one explicit path before implementation promotion:

1. **GPL-compatible path**
   - Use GPL-bound source or derivative implementation detail only in a GPL-compatible distribution lane.
   - Keep the work outside the current dual `MIT OR Apache-2.0` core until the project deliberately changes or extends the licensing plan.
   - Include a GPL-compatible license file, source-offer expectations, third-party notices, and distribution notes before code is bundled.

2. **Permissioned or relicensed path**
   - Obtain written permission from the relevant rights holders for the exact use and distribution terms.
   - Record the permission artifact as private evidence or a governed public record before implementation promotion.
   - Do not infer permission from local possession of the source archive.

3. **Clean-room path**
   - Separate specification and implementation responsibilities.
   - The specification role may inspect GPL-bound source and public documents to write behavior specifications, fixture descriptions, and golden-master expectations.
   - The implementation role must not inspect GPL-bound implementation code and implements from the reviewed specifications, public domain facts, allowed public documents, and explicitly licensed fixtures.
   - Keep reviewer logs, independence attestations, fixture manifests, and traceability records before code promotion.

If no path is selected, BioSim-RS remains a source-intake and planning workstream only.

## Repository and workspace boundary

The current AeroCodex crates remain dual `MIT OR Apache-2.0` and must not receive GPL-bound or translated Java implementation material.

Allowed in the dual-core repository before a licensing decision:

- governance documents;
- source inventory summaries;
- high-level architecture descriptions;
- validation plans;
- placeholder directories with README-only warnings;
- source IDs, validation statuses, and non-promotional registry entries.

Blocked before a licensing decision:

- copied Java code, comments, class bodies, package structure, translated tests, or derivative implementation detail;
- bulk import of the BioSim-RS scaffold into the Cargo workspace;
- making `biosim-*` crates members of the current AeroCodex workspace;
- public APIs that imply validated habitat, medical, operational, or regulated-use capability;
- distributing GPL-bound material under the dual `MIT OR Apache-2.0` core license.

The placeholder `biosim-rs/` directory is a Stage 4 boundary marker only. It must remain README-only until a later chunk deliberately adds a BioSim-RS workspace. Chunk 6A, Chunk 6B, Chunk 6C, Chunk 6D, and Chunk 6E clean-room primitives live in `crates/aero-codex-life-support` and do not import the external BioSim-RS scaffold crates.

## Architecture boundary for future implementation slices

Future BioSim-RS implementation slices should remain Rust-native and deterministic rather than Java class-for-class translations. The permitted architecture target is a typed simulation kernel where modules read an immutable snapshot, emit intents, a resource allocator commits changes, invariants are checked, and telemetry/replay records are produced.

Future slices should be sequenced as:

1. resource identity and tick validation — completed in Chunk 6A as clean-room generic identities, positive-duration tick validation, and consecutive transition checks;
2. atomic transaction commit — completed in Chunk 6B as a clean-room caller-state/caller-delta helper that rejects invalid commits without exposing a committed output;
3. deterministic ordering, digest, and replay proof — completed in Chunk 6C as clean-room canonical ordering, fnv-1a digest evidence, and one-tick replay proof;
4. resource ledger and minimal oxygen-loop conservation — completed in Chunk 6D as grouped residual checks plus a bounded two-store oxygen transfer proof;
5. CLI/API smoke tests and friend-test report — completed in Chunk 6E as a static clean-room API/example-output smoke report over the earlier kernel slices.

Each slice must carry its own source IDs, license-path reference, validation status, unit/domain assumptions, mass/energy ledger expectations, deterministic replay evidence, and conservative research caveat. Chunk 6A intentionally carries no transaction, ledger, replay, or conservation evidence. Chunk 6B carries only atomic resource-delta commit evidence. Chunk 6C carries deterministic ordering, fnv-1a digest, and one-tick replay proof evidence only. Chunk 6D carries grouped ledger residual and bounded minimal oxygen-loop conservation evidence only. Chunk 6E carries static API/example-output smoke evidence only; persistent command surfaces, external fixture replay, scenario execution, and habitat-control behavior remain future work.

## Clean-room evidence package for future chunks

A future clean-room implementation slice should produce at least:

- a source-intake record naming which GPL-bound artifacts were inspected by the specification role;
- a specification document that avoids copied Java implementation expression;
- a fixture manifest with allowed origin, license, and hash for every scenario or golden-master output;
- an implementation independence note naming the implementation inputs;
- tests that prove deterministic replay and ledger behavior without importing GPL source;
- a validation card that uses the status vocabulary and avoids readiness or certification claims.

## Promotion gate

BioSim-RS may not be promoted into the current AeroCodex public API until all of the following are true:

1. The license path is selected and documented.
2. Source IDs and registry entries identify every source artifact used.
3. The implementation boundary is reviewed for GPL mixing risk.
4. Validation cards and source-registry entries exist for the slice.
5. Rust checks, deterministic replay checks, and ledger checks pass.
6. User-facing docs repeat the research/preliminary-design caveat.
7. The maintainer deliberately accepts the distribution and licensing impact.

Until then, BioSim-RS is first-class Stage 4 planning material, not a promoted dual-core implementation.
