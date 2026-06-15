# Orekit Reference-Oracle Mapping

Stage 4 Chunk 5 defines how AeroCodex may use Orekit as an external reference oracle and architecture comparison source. It is governance and documentation only. It does not import Orekit source code, does not add AeroCodex public APIs, does not port Java classes, and does not change the current dual `MIT OR Apache-2.0` AeroCodex core.

AeroCodex remains research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Registered source artifact

The governing source ID is registered in `data-governance/DATA_REGISTRY.yaml`:

| Source ID | Registered path | License | Current use |
| --- | --- | --- | --- |
| `stage4.orekit_reference.2026_06_15` | `external://stage4/Orekit-develop.zip` | Apache-2.0 | Reference oracle and architecture comparison only |

The archive stays outside the repository. Its registry entry records hash and provenance; it is not permission to mirror Orekit's Java package tree, class hierarchy, comments, test bodies, or implementation expression in AeroCodex.

## Observed source shape

The Stage 4 source inventory records the Orekit archive as an Apache-2.0 reference with 4,865 file entries, including 3,584 Java source files and 64 PlantUML files. The inspected archive exposes broad space-flight-dynamics areas such as propagation, orbit representations, frames, time, forces, files, estimation, bodies, data loading, and utilities.

These observations are used only to choose oracle families and architecture comparison questions. They are not implementation inputs.

## Permitted use modes

| Use mode | Meaning | Required boundary |
| --- | --- | --- |
| Reference-oracle planning | Use Orekit to identify independent scenario families, expected quantities, tolerances, and edge cases for future AeroCodex checks. | Record an AeroCodex-native oracle evidence record before using any generated expected values. |
| Architecture comparison | Compare high-level responsibilities and data-flow questions such as time handling, frame transforms, propagator state, force-model boundaries, and file ingestion. | Convert comparisons into AeroCodex-native contracts; do not preserve Java inheritance or package structure. |
| Test-family selection | Select families of future tests that should exist before astrodynamics promotion. | Tests must cite AeroCodex source IDs, units, domains, and tolerance policies; Orekit-derived fixtures require a fixture manifest and hash. |
| Documentation vocabulary | Use Orekit-facing terminology to notice concepts that AeroCodex docs should define. | Add only terms that are needed for AeroCodex contracts and pass nomenclature checks. |

## Reference-oracle family map

| Orekit-facing area | AeroCodex target | Future oracle evidence to require | Current Chunk 5 effect |
| --- | --- | --- | --- |
| Orbit representation and element conversion | `crates/aero-codex-astrodynamics` and future formula-vault contracts | Source ID, input units, output units, singularities, round-trip checks, tolerance rationale | Planning only; no new equations or APIs |
| Two-body and analytical propagation | Existing astrodynamics research helpers plus future formula-vault candidates | Independent scenario set, epoch, central-body parameter, expected position and velocity, absolute and relative tolerances | Planning only |
| Frame and body transforms | Future astrodynamics contracts | Frame names, epoch, body model, transform direction, valid domain, edge cases near singularities | Planning only |
| Time scale and epoch conversion | Future core or astrodynamics utility contracts | Time scale names, leap-second source, epoch representation, precision and discontinuity expectations | Planning only |
| Force and perturbation model families | Future formula-vault candidates | Source equation references, assumptions, valid domains, disabled/enabled model list, tolerance budget | Planning only |
| Event or boundary-condition handling | Future test-harness design | Predicate definition, root-finding expectations, bracketing behavior, non-convergence handling | Planning only |
| Ephemeris and tracking-data file families | Future fixture-ingestion plans | File license, file hash, parser boundary, expected parsed quantities, no bundled external archive unless approved | Planning only |
| Estimation and measurement model families | Future roadmap only | Measurement definition, noise assumptions, residual tolerance, source-provenance record | Planning only |

This map is intentionally a family map rather than a class map. AeroCodex should not create modules whose purpose is to be a Rust copy of Orekit packages.

## Non-copying architecture rules

1. Do not mirror `org.orekit` package names, class names, inheritance patterns, abstract class boundaries, or interface hierarchy as the AeroCodex architecture.
2. Do not copy Java source, comments, test code, data-loader logic, examples, or generated documentation into AeroCodex.
3. Do not translate method bodies or preserve one-to-one class responsibilities.
4. Do not promote an AeroCodex behavior merely because Orekit has an analogous class or test.
5. Convert each future use into an AeroCodex-native contract with source IDs, units, domains, singularities, tolerance policy, validation status, and documentation caveat.
6. Keep generated oracle values, fixture manifests, and execution logs outside the repository unless a future chunk explicitly approves a bounded, license-reviewed fixture import.

## Future oracle evidence record

Before a future implementation slice can claim an Orekit-backed reference check, it should record:

1. AeroCodex capability or formula ID.
2. AeroCodex source artifact IDs and Orekit source ID.
3. Orekit archive hash and version/source snapshot.
4. The inspected Orekit area at a family level, not copied implementation text.
5. Scenario inputs, units, frames, epoch, assumptions, and valid domain.
6. Expected output quantities and units.
7. Absolute and relative tolerances with rationale.
8. Fixture or generated-output license and SHA256 where an external value file is kept.
9. Command or procedure used to generate the comparison value.
10. Validation/status vocabulary value.
11. Promotion gate and explicit research/preliminary-design caveat.

## Promotion gate

An AeroCodex astrodynamics capability may not be promoted because of Orekit comparison alone. It still needs all normal AeroCodex gates: source IDs, equation contracts, reviewed Rust implementation, unit tests, reference-oracle tests, tolerance records, validation cards, dependency-policy compliance, documentation, and final maintainer acceptance.

Until those slice-specific gates exist, Orekit remains registered source material for oracle planning and architecture comparison only.
