# Stage 4 Source Inventory

This inventory records Stage 4 local source materials inspected for planning. It does not import the archives into the repository and does not promote any external source into a public application programming interface.

AeroCodex remains research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Chunk 0 intake summary

| Source ID | Local bundle filename | SHA256 | Observed role | Stage 4 state |
| --- | --- | --- | --- | --- |
| `stage4:aerocodex-main-reference:2026-06-15` | `AeroCodex-main (3).zip` | `48e3a8bea47a9093e43538e7c2a47637f15278bffdadff63e2a603e2efa7d12c` | Current AeroCodex repo snapshot/reference overlay. | Compared for docs; current GitHub `main` matched the inspected reference docs, so no bulk copy was needed. |
| `stage4:master-plan:2026-06-15` | `AeroCodex_stage4_refined_master_plan.md` | `8f8beea3ee54465405d44e24ed750cc28f5afc38ea0c9540a47260a0c1805e46` | Consolidated Stage 4 planning source. | Used to author Stage 4 governance docs. |
| `stage4:first-prompt:2026-06-15` | `AeroCodex_stage4_first_prompt.txt` | `d2861842ae55bbb48ad6cafa1b911a22fef13a3335c2d68068b65f25870855a6` | Initial operating prompt for Stage 4. | Used to freeze Chunk 0 rules. |
| `stage4:m07-rust-port-v14:2026-06-15` | `aerocodex_rust_port_v14_m07_final_bundle.zip` | `15b1ca3a39267187167c43ea1228f28fd4736c4456f65d072dc42a32a7b19190` | Scilab-to-Rust astrodynamics release-candidate workspace and validation docs. | Quarantined formula-vault candidate; not public API. |
| `stage4:biosim-rs-bootstrap:2026-06-15` | `aerocodex-biosim-rs-bootstrap.zip` | `4e8912dabe1e0a41a0ad397e4b4b57a81749a4c7bbfbebedcfd944d44288183e` | Rust-native BioSim-RS scaffold and migration docs. | First-class but license-boundaried planning source. |
| `stage4:biosim-java-reference:2026-06-15` | `biosim-main (1).zip` | `4168c9147ccdc14890b497a944354f615119f2908996805559d61c11991414c5` | Original Java BioSim source/reference. | GPL-3.0-or-later boundary; do not mix into dual MIT/Apache core. |
| `stage4:orekit-reference:2026-06-15` | `Orekit-develop.zip` | `f4b36d8a8df7d293724fb3ffbd07264d4bc984ab36506bc223a6545cc27bd574` | Orekit source reference. | Reference oracle and architecture guide only; no class-for-class clone. |
| `stage4:talk-plan:2026-06-15` | `talk.zip` | `fef17836f1074602b40850ad1fe4a821ae9fa589b2fae14b9bc160df11fc5c18` | Prior consolidated planning notes. | Planning context only. |

## Chunk 1 governance registry

Chunk 1 registers the in-repo manifests/directories and external Stage 4 archives in `data-governance/DATA_REGISTRY.yaml`. The companion data/source governance policy defines repo-relative versus `external://stage4/...` paths, directory aggregate SHA256 handling, license/status fields, quarantine decisions, and the dependency-free `cargo run -p xtask -- verify data-registry` gate.

## Chunk 2 status vocabulary normalization

Chunk 2 adds the canonical status vocabulary in `validation/status_vocabulary.yaml` and the human guidance in `validation/status_vocabulary.md`. The `cargo run -p xtask -- verify status-vocabulary` gate checks that validation cards, source-registry seeds, and data-governance `validation_status` / `hash_status` fields use documented status values and do not add readiness/certification claims as allowed statuses.

No source bundle, validation card, source-registry seed, or data-governance artifact is promoted by this chunk.

## Chunk 3 formula-vault staging

Chunk 3 defines the quarantined M07 formula-vault staging shape in `docs/assurance/formula_vault_staging.md`, records the M07 intake boundary in `docs/source_intake/m07_formula_vault_intake.md`, and adds the empty `formula-vault/` skeleton for future reviewed metadata. It does not import M07 source code, does not promote public application programming interfaces, and does not overwrite `crates/aero-codex-astrodynamics`.

The M07 source artifact remains the registered external archive `stage4.m07_rust_port_v14.2026_06_15`; the formula-vault states are quarantine lifecycle labels only and do not upgrade validation or certification status.

## Chunk 4 BioSim-RS license-bound architecture

Chunk 4 defines the BioSim-RS license and source boundary in `docs/assurance/biosim_rs_license_architecture.md`, records the source-intake boundary in `docs/source_intake/biosim_rs_source_boundary.md`, and adds the README-only `biosim-rs/` placeholder. It does not import Java BioSim code, does not import the BioSim-RS bootstrap scaffold, and does not make any `biosim-*` crates members of the current AeroCodex workspace.

The BioSim-RS source artifacts remain the registered external archives `stage4.biosim_rs_bootstrap.2026_06_15` and `stage4.biosim_java_reference.2026_06_15`. Later BioSim-RS slices must choose a GPL-compatible, permissioned, or clean-room path before implementation promotion.

## Chunk 5 Orekit reference-oracle mapping

Chunk 5 defines the Orekit reference-oracle mapping in `docs/assurance/orekit_reference_oracle_mapping.md` and records the source boundary in `docs/source_intake/orekit_reference_oracle_boundary.md`. It does not import Orekit source code, does not run Orekit, does not create fixtures, does not add public application programming interfaces, and does not copy the Java class hierarchy.

The Orekit source artifact remains the registered external archive `stage4.orekit_reference.2026_06_15`. Future Orekit-backed checks must create slice-specific oracle evidence records with source IDs, units, frames, epochs, valid domains, expected outputs, tolerances, fixture hashes where applicable, validation/status vocabulary values, and the research/preliminary-design caveat.

## Chunk 6A BioSim-RS resource identity and tick validation

Chunk 6A adds the first clean-room implementation slice for BioSim-RS under `crates/aero-codex-life-support/src/biosim_resource_tick.rs`, plus validation card `life_support.biosim_rs.resource_tick` and source seed `source.life_support.biosim_rs.resource_tick_clean_room.research_required`. It validates unique generic resource identity catalogs, finite positive tick durations, and consecutive tick transitions only.

The slice does not import Java BioSim code, does not import the BioSim-RS scaffold crates, does not execute scenarios, does not add fixtures, and does not implement transaction commit, deterministic replay, resource ledgers, or O2-loop conservation. The validation status remains `research_required`.

## Chunk 6B BioSim-RS atomic transaction commit

Chunk 6B adds a clean-room atomic transaction commit primitive under `crates/aero-codex-life-support/src/biosim_resource_tick.rs`, plus validation card `life_support.biosim_rs.atomic_transaction_commit` and source seed `source.life_support.biosim_rs.transaction_commit_clean_room.research_required`. It applies finite resource deltas over a caller-supplied state at one validated consecutive tick boundary and rejects invalid commits before exposing committed output.

The slice does not import Java BioSim code, does not import the BioSim-RS scaffold crates, does not execute scenarios, does not add fixtures, and does not implement deterministic replay, persistent resource ledgers, O2-loop conservation, or habitat-control behavior. The validation status remains `research_required`.

## Chunk 6C BioSim-RS deterministic ordering, digest, and replay proof

Chunk 6C adds clean-room deterministic ordering and digest helpers under `crates/aero-codex-life-support/src/biosim_resource_tick.rs`, plus validation card `life_support.biosim_rs.deterministic_ordering_digest_replay` and source seed `source.life_support.biosim_rs.deterministic_replay_clean_room.research_required`. It canonicalizes caller-supplied resource states and deltas by static resource ID, emits dependency-free fnv-1a before/after digests, and returns one-tick replay proof evidence after the atomic commit succeeds.

The slice does not import Java BioSim code, does not import the BioSim-RS scaffold crates, does not execute scenarios, does not add fixtures, and does not implement persistent resource ledgers, O2-loop conservation, biological dynamics, or habitat-control behavior. The validation status remains `research_required`.

## Chunk 6D BioSim-RS resource ledger and minimal oxygen-loop conservation

Chunk 6D adds clean-room grouped resource-ledger residual checks under `crates/aero-codex-life-support/src/biosim_resource_tick.rs`, plus validation card `life_support.biosim_rs.resource_ledger_minimal_o2_loop_conservation` and source seed `source.life_support.biosim_rs.resource_ledger_clean_room.research_required`. It groups caller-supplied before/after store totals by static resource kind and canonical unit, computes residuals against caller-accounted source/sink terms, and proves a bounded two-store oxygen transfer loop within a declared absolute tolerance.

The slice does not import Java BioSim code, does not import the BioSim-RS scaffold crates, does not execute scenarios, does not add fixtures, and does not implement persistent ledger storage, biological dynamics, habitat-control behavior, or external BioSim parity. The validation status remains `research_required`.

## Chunk 6E BioSim-RS CLI/API smoke and friend-test report

Chunk 6E adds a static clean-room smoke/reporting slice under `crates/aero-codex-life-support/src/biosim_resource_tick.rs`, plus package example `crates/aero-codex-life-support/examples/biosim_friend_test_smoke.rs`, validation card `life_support.biosim_rs.cli_api_smoke_friend_test_report`, and source seed `source.life_support.biosim_rs.cli_api_smoke_clean_room.research_required`. It composes the Chunk 6A through Chunk 6D primitives with deterministic built-in inputs and prints a friend-test report that repeats the research boundary.

The slice does not import Java BioSim code, does not import the BioSim-RS scaffold crates, does not execute scenarios, does not add external fixtures, and does not implement persistent command surfaces, biological dynamics, habitat-control behavior, or external BioSim parity. The validation status remains `research_required`.

## Chunk 7A formula-vault implementation-candidate gate

Chunk 7A adds a metadata-only gate for future formula-vault implementation candidates: `docs/assurance/formula_vault_candidate_gate.md`, non-operative template `formula-vault/templates/implementation_candidate_slice.yaml`, validation card `validation.formula_vault.candidate_gate`, and source seed `source.validation.formula_vault_candidate_gate.research_required`.

The gate requires future per-slice source locators, variable/unit/frame/time metadata, domain/singularity/branch/tolerance records, evidence plans, and blocked-by-default promotion status before any formula implementation is proposed. It does not import M07 code, does not add archives or fixtures, does not run Scilab or SGP4 checks, does not create public application programming interfaces, and does not make certification, readiness, operational, medical, or regulated-use claims. The validation status remains `research_required`.

## Chunk 7B formula-vault M00 angle/unit metadata slice

Chunk 7B adds the first bounded formula-vault candidate metadata slice: `formula-vault/candidates/m00_angle_unit_conversions.yaml`, `docs/assurance/formula_vault_m00_angle_unit_candidate.md`, validation card `validation.formula_vault.m00_angle_unit_conversions`, and source seed `source.formula_vault.m00_angle_unit_conversions.research_required`.

The slice is limited to M07 release-gate rows 3 through 5 for `app_deg2rad`, `app_rad2deg`, and `app_wrap2pi`, plus Scilab equivalence job locator `equivalence job 002`. It does not import M07 code, does not add archives or fixtures, does not execute Scilab, does not implement formulas, does not create public application programming interfaces, and does not make certification, readiness, operational, medical, or regulated-use claims. The validation status remains `research_required`.

## Chunk 7C formula-vault candidate verifier

Chunk 7C adds a dependency-free verifier for formula-vault candidate metadata: `cargo run -p xtask -- verify formula-vault`, assurance note `docs/assurance/formula_vault_candidate_verifier.md`, validation card `validation.formula_vault.candidate_verifier`, and source seed `source.validation.formula_vault_candidate_verifier.research_required`.

The verifier checks required candidate sections, existing validation/source cross-links, duplicate slice/formula identifiers, blocked promotion state, required non-claim booleans, and absence of local evidence paths. It does not import M07 code, does not add archives or fixtures, does not execute Scilab, does not implement formulas, does not create public application programming interfaces, and does not make certification, readiness, operational, medical, or regulated-use claims. The validation status remains `research_required`.

## Chunk 7D formula-vault per-candidate manifest/reference-link depth

Chunk 7D adds metadata-only reference depth for the existing M00 candidate: `formula-vault/manifests/m00_angle_unit_conversions_manifest.yaml`, assurance note `docs/assurance/formula_vault_m00_reference_manifest.md`, validation card `validation.formula_vault.m00_reference_manifest`, and source seed `source.formula_vault.m00_reference_manifest.research_required`.

The manifest links each selected formula identifier to its candidate function alias, source-function alias, release-gate row alias, equivalence-job alias, source-file locator, candidate/assurance/validation/source records, and pending review statuses. It does not import M07 code, does not copy source expressions, does not add archives or fixtures, does not execute Scilab, does not implement formulas, does not create public application programming interfaces, and does not make certification, readiness, operational, medical, or regulated-use claims. The validation status remains `research_required`.


## Chunk 7F M00 source-expression and test-vector contract

Chunk 7F adds the machine-readable contract `formula-vault/contracts/m00_angle_unit_conversions_contract.yaml`, assurance note `docs/assurance/formula_vault_m00_source_expression_test_vectors.md`, validation card `validation.formula_vault.m00_source_expression_test_vectors`, and source seed `source.formula_vault.m00_source_expression_test_vectors.research_required`.

The contract covers only `formula_vault.m00.angle.deg2rad`, `formula_vault.m00.angle.rad2deg`, and `formula_vault.m00.angle.wrap2pi`. It records independently written mathematical summaries, finite-input domains, invalid non-finite inputs, tolerance metadata, reference-oracle metadata, and endpoint-sensitive `wrap2pi` behavior. It does not import M07 source code, does not copy source expressions into implementation code, does not add archives or fixtures, does not execute Scilab, does not implement formulas, does not create public application programming interfaces, and does not make certification, readiness, operational, medical, habitat-safety, or regulated-use claims. The validation status remains `research_required`.

## Chunk 7E equation inventory/readiness dashboard

Chunk 7E adds the machine-readable inventory `validation/equation_inventory.tsv`, assurance note `docs/assurance/equation_inventory_readiness_dashboard.md`, validation card `validation.equation_inventory.readiness_dashboard`, source seed `source.validation.equation_inventory.readiness_dashboard.research_required`, and dependency-free verifier `cargo run -p xtask -- verify equation-inventory`.

The inventory distinguishes executable research equations, metadata-only formula-vault candidates, external M07 backlog rows, validation-card-only records, source-registry seeds, and helper algorithms. Every inventory row remains blocked with an explicit block reason. The chunk does not implement formulas, does not import M07 code, does not add generated formula code or fixtures, does not create public application programming interfaces, and does not make certification, readiness, operational, medical, or regulated-use claims. The validation status remains `research_required`.

## M07 observations

- The M07 artifact manifest reports 1,350 represented function rows.
- The generated final port summary reports 188 Scilab equivalence source jobs.
- The M07 package state is release-candidate material pending AeroCodex Rust checks, Scilab equivalence execution, and SGP4 certification.
- It must not overwrite the existing `crates/aero-codex-astrodynamics` crate.
- It must not be bulk-merged into public application programming interfaces. Treat it as quarantined formula-vault candidate material until intake, correctness, and release gates pass.

## BioSim and BioSim-RS observations

- The original BioSim Java reference archive was observed with 296 Java files, 93 configuration files, 29 schema files, 9 `.biosim` scenario files, and 442 total files.
- The BioSim-RS bootstrap archive was observed with 47 files and the scaffold crates `biosim-api`, `biosim-cli`, `biosim-config`, `biosim-core`, `biosim-models`, `biosim-telemetry`, and `biosim-verify`.
- BioSim-RS is first-class Stage 4 source material, but the original Java source is GPL-3.0-or-later. Future work must choose and document a GPL-compatible, permissioned, or clean-room path before implementation code is promoted into the current dual MIT/Apache AeroCodex core.

## Orekit observations

- The Orekit archive was observed with 4,865 total files, including 3,584 Java source files, 83 markup files, and 64 PlantUML files.
- Orekit is Apache-2.0 source material and can serve as a reference oracle and architecture guide.
- Stage 4 must not copy Orekit design class-for-class or preserve Java class hierarchy structure in Rust.
- Chunk 5 maps permitted oracle families at a capability/test-family level only; future slices still need fixture manifests, source IDs, units, domains, tolerances, validation cards, and promotion gates before any Orekit-backed comparison can support public behavior.

## Intake rules for future chunks

1. Assign a durable source ID before using any source bundle.
2. Record license, provenance, and source-boundary status before implementation.
3. Keep raw archives out of the repository unless a future chunk explicitly approves a source-material import.
4. Do not promote external source code into public AeroCodex API without equation contracts, tests, tolerances, validation status, docs, and release-gate evidence.
5. Preserve the one-canonical-`main` rule and the non-certification caveat in all source-intake outputs.
