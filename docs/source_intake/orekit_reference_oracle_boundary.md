# Orekit Reference-Oracle Source Boundary

This source-intake note records the Stage 4 Chunk 5 boundary for Orekit reference-oracle use. It does not import the Orekit archive, does not promote implementation code, and does not authorize a class-for-class Java-to-Rust port.

AeroCodex remains research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Source artifact

| Source ID | Registered path | SHA256 | Boundary status |
| --- | --- | --- | --- |
| `stage4.orekit_reference.2026_06_15` | `external://stage4/Orekit-develop.zip` | `f4b36d8a8df7d293724fb3ffbd07264d4bc984ab36506bc223a6545cc27bd574` | Apache-2.0 reference source; oracle planning and architecture comparison only |

The source archive remains external to the repository. Chunk 5 inspected archive metadata only: 4,865 file entries, 3,584 Java source files, 64 PlantUML files, and high-level source families including propagation, orbits, frames, time, forces, files, estimation, bodies, data loading, and utilities.

These inventory facts are not an import, not a crate membership decision, and not implementation authorization.

## Boundary labels

The following labels are source-intake lifecycle labels for Orekit-backed work. They do not replace data-governance validation statuses.

| Label | Meaning | Allowed repository effect |
| --- | --- | --- |
| `registered_source_material` | Orekit is recorded by source ID, hash, license, and allowed use. | Registry and inventory notes only |
| `reference_oracle_candidate` | A future AeroCodex formula or capability proposes Orekit comparison. | Planning note and oracle-evidence checklist only |
| `architecture_comparison_note` | A future design review compares responsibilities or data flow against Orekit at a family level. | AeroCodex-native design notes only |
| `fixture_manifest_drafted` | A bounded external expected-value file or scenario is proposed. | Manifest with license, source, and hash; no archive import by default |
| `oracle_job_planned` | A comparison job is defined but has not run. | Test-plan record only |
| `oracle_job_executed` | A bounded comparison job has run and recorded tolerances/results. | Evidence record; promotion still requires all other gates |
| `blocked_clone_or_derivative` | A proposal copies class hierarchy, source expression, or one-to-one package design. | Block promotion and keep only a redacted provenance note |
| `rejected_or_superseded` | The source use or oracle plan is replaced or intentionally retired. | Retain provenance note and block promotion |

## Allowed planning use now

Current Chunk 5 use is limited to:

- recording source ID, license, hash, and non-bundling status;
- defining how Orekit can help select future reference-oracle families;
- naming the evidence fields required before any Orekit-generated expected values are used;
- documenting non-copying architecture rules;
- connecting future orbit, time, frame, force-model, file-ingestion, event, and estimation checks to AeroCodex-native contracts.

## Blocked use now

The following remain blocked:

- copying Orekit Java source, comments, examples, test bodies, data-loader logic, package structure, or class hierarchy into AeroCodex;
- translating Orekit classes or methods one-for-one into Rust;
- replacing `crates/aero-codex-astrodynamics` with an Orekit-shaped module tree;
- adding public APIs that imply validated flight dynamics, operational readiness, or regulated-use approval;
- committing the raw `Orekit-develop.zip` archive, extracted source tree, generated comparison logs, or unreviewed fixture files;
- claiming certification or safety-critical suitability from an Orekit comparison.

## Future intake records

Before a future AeroCodex slice uses Orekit as a reference oracle, create a slice-specific intake record that includes:

1. AeroCodex capability or formula ID.
2. AeroCodex source IDs plus `stage4.orekit_reference.2026_06_15` as the oracle reference.
3. Orekit archive hash and date/source snapshot.
4. Family-level Orekit area used for comparison, with no copied implementation expression.
5. Scenario inputs, units, frames, time scale, epoch, assumptions, and valid domain.
6. Expected output quantities, units, and tolerance policy.
7. Fixture or generated-output license and SHA256, if any expected-value artifact is kept.
8. Command or procedure used to generate comparison values.
9. Validation/status vocabulary value and validation card path.
10. Promotion gate, unresolved-risk list, and explicit research/preliminary-design caveat.

Chunk 5 creates the boundary and mapping only. It does not run Orekit, generate oracle fixtures, or promote any Orekit-derived implementation.
