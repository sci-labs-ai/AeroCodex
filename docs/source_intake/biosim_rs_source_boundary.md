# BioSim-RS Source Boundary

This source-intake note records the Stage 4 Chunk 4 boundary for BioSim-RS, the Chunk 6A clean-room resource/tick slice, the Chunk 6B clean-room atomic transaction commit slice, the Chunk 6C clean-room deterministic ordering/digest slice, and the Chunk 6D clean-room resource-ledger slice. It does not import any source archive, does not promote GPL-bound implementation code, and does not change AeroCodex's current dual `MIT OR Apache-2.0` core.

AeroCodex remains research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Source artifacts

| Source ID | Registered path | SHA256 | Boundary status |
| --- | --- | --- | --- |
| `stage4.biosim_rs_bootstrap.2026_06_15` | `external://stage4/aerocodex-biosim-rs-bootstrap.zip` | `4e8912dabe1e0a41a0ad397e4b4b57a81749a4c7bbfbebedcfd944d44288183e` | GPL-boundaried Rust scaffold; reference and rewrite planning only |
| `stage4.biosim_java_reference.2026_06_15` | `external://stage4/biosim-main (1).zip` | `4168c9147ccdc14890b497a944354f615119f2908996805559d61c11991414c5` | GPL-3.0 Java source reference; clean-room planning only |

The Stage 4 source inventory observed the original Java reference archive with 296 Java files, 93 configuration files, 29 schema files, 9 `.biosim` scenario files, and 442 total files. It observed the BioSim-RS bootstrap archive with the scaffold crates `biosim-api`, `biosim-cli`, `biosim-config`, `biosim-core`, `biosim-models`, `biosim-telemetry`, and `biosim-verify`.

These facts are inventory facts only. They are not implementation authorization.

## Boundary labels

The following labels are source-intake lifecycle labels for BioSim-RS. They do not replace data-governance validation statuses.

| Label | Meaning | Allowed repository effect |
| --- | --- | --- |
| `registered_source_material` | The artifact is recorded by source ID, hash, and license boundary. | Registry and inventory notes only |
| `license_path_pending` | GPL-compatible, permissioned, or clean-room path has not been selected. | No implementation promotion |
| `gpl_compatible_lane` | The project deliberately chooses a GPL-compatible distribution path for a bounded workspace. | Separate licensed lane only after approval |
| `permissioned_lane` | The project records rights-holder permission for the intended use. | Bounded implementation after permission evidence |
| `clean_room_spec_lane` | Specification role may inspect GPL-bound source and write non-copying behavior specs. | Specs and fixture manifests only |
| `clean_room_implementation_lane` | Implementation role builds from approved specs without inspecting GPL-bound implementation code. | Bounded code only after independence evidence |
| `rejected_or_superseded` | The artifact or lane is no longer used. | Retain provenance note and block promotion |

## Allowed planning and Chunk 6A/6B/6C/6D use

Chunk 4 use remains limited to:

- recording source IDs and license boundaries;
- defining repository and workspace placement rules;
- planning clean-room, permissioned, and GPL-compatible options;
- naming future validation gates for deterministic replay, resource ledgers, and minimal O2-loop conservation;
- preserving the BioSim-RS workstream as first-class but license-boundaried.

Chunk 6A adds only clean-room generic resource identities and local tick validation in `crates/aero-codex-life-support`, with source seed `source.life_support.biosim_rs.resource_tick_clean_room.research_required` and validation card `life_support.biosim_rs.resource_tick`. It uses no external BioSim archive contents, no GPL-bound scaffold crates, no fixtures, no scenarios, and no transaction or ledger behavior.

Chunk 6B adds only clean-room atomic resource-delta commit behavior in `crates/aero-codex-life-support`, with source seed `source.life_support.biosim_rs.transaction_commit_clean_room.research_required` and validation card `life_support.biosim_rs.atomic_transaction_commit`. It uses no external BioSim archive contents, no GPL-bound scaffold crates, no fixtures, no scenarios, and no deterministic replay, persistent ledger, conservation, or habitat-control behavior.

Chunk 6C adds only clean-room deterministic resource ordering and fnv-1a before/after digest evidence in `crates/aero-codex-life-support`, with source seed `source.life_support.biosim_rs.deterministic_replay_clean_room.research_required` and validation card `life_support.biosim_rs.deterministic_ordering_digest_replay`. It uses no external BioSim archive contents, no GPL-bound scaffold crates, no fixtures, no scenarios, and no persistent ledger, conservation, biological-dynamics, or habitat-control behavior.

Chunk 6D adds only clean-room grouped resource-ledger residual checks and bounded minimal oxygen-loop conservation proof evidence in `crates/aero-codex-life-support`, with source seed `source.life_support.biosim_rs.resource_ledger_clean_room.research_required` and validation card `life_support.biosim_rs.resource_ledger_minimal_o2_loop_conservation`. It uses no external BioSim archive contents, no GPL-bound scaffold crates, no fixtures, no scenarios, and no persistent ledger storage, biological dynamics, habitat-control behavior, or external BioSim parity claim.

## Blocked use now

The following remain blocked:

- copying Java BioSim implementation code into this repository;
- translating Java classes, methods, comments, or package structure into Rust;
- importing the BioSim-RS bootstrap scaffold as workspace crates;
- using GPL-bound fixtures without a fixture license and hash record;
- claiming habitat safety, medical suitability, operational readiness, certification, or regulated-use approval;
- adding persistent resource-ledger storage, broader conservation models, scenario-execution, or habitat-control behavior without new slice-specific evidence;
- merging any GPL-derived BioSim-RS implementation into the dual AeroCodex core before the license path and validation gates are accepted.

## Future intake records

Before any later BioSim-RS implementation slice lands, create or update a slice-specific intake record that includes:

1. source artifact IDs used;
2. selected license path;
3. implementation boundary statement;
4. fixture or golden-master manifest with hashes and licenses;
5. resource and unit assumptions;
6. tick and transaction semantics;
7. mass/energy ledger expectations;
8. broader deterministic replay proof plan;
9. validation status vocabulary value;
10. promotion gate and explicit non-certification caveat.

The next BioSim-RS implementation-oriented chunks must remain blocked until this source boundary is referenced by their slice records.
