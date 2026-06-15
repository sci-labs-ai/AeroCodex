# Stage 4 Source Inventory

This inventory records Stage 4 local source materials inspected for planning. It does not import the archives into the repository and does not promote any external source into public API.

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

## M07 astrodynamics release-candidate observations

- The M07 artifact manifest reports 1,350 represented function rows.
- The generated final port summary reports 188 Scilab equivalence source jobs.
- The M07 package state is release-candidate material pending AeroCodex Rust checks, Scilab equivalence execution, and SGP4 certification.
- It must not overwrite the existing `crates/aero-codex-astrodynamics` crate.
- It must not be bulk-merged into public APIs. Treat it as quarantined formula-vault candidate material until intake, correctness, and release gates pass.

## BioSim and BioSim-RS observations

- The original BioSim Java reference archive was observed with 296 Java files, 93 configuration files, 29 schema files, 9 `.biosim` scenario files, and 442 total files.
- The BioSim-RS bootstrap archive was observed with 47 files and the scaffold crates `biosim-api`, `biosim-cli`, `biosim-config`, `biosim-core`, `biosim-models`, `biosim-telemetry`, and `biosim-verify`.
- BioSim-RS is first-class Stage 4 source material, but the original Java source is GPL-3.0-or-later. Future work must choose and document a GPL-compatible, permissioned, or clean-room path before implementation code is promoted into the current dual MIT/Apache AeroCodex core.

## Orekit observations

- The Orekit archive was observed with 4,865 total files, including 3,584 Java source files, 83 markup files, and 64 PlantUML files.
- Orekit is Apache-2.0 source material and can serve as a reference oracle and architecture guide.
- Stage 4 must not copy Orekit design class-for-class or preserve Java class hierarchy structure in Rust.

## Intake rules for future chunks

1. Assign a durable source ID before using any source bundle.
2. Record license, provenance, and source-boundary status before implementation.
3. Keep raw archives out of the repository unless a future chunk explicitly approves a source-material import.
4. Do not promote external source code into public AeroCodex API without equation contracts, tests, tolerances, validation status, docs, and release-gate evidence.
5. Preserve the one-canonical-`main` rule and the non-certification caveat in all source-intake outputs.
