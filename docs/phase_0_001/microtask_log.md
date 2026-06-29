# Phase 0.001 Microtask Log

This log tracks the current interactive, one-microtask-at-a-time review and development session.

The uploaded repository foundation already contains generated files for microtasks 001-020. For this session, those files are treated as the baseline to inspect and improve incrementally. A task is marked complete here only after it has been reviewed or updated during this interactive session.

## Current session baseline

- Outer uploaded bundle: `AeroCodex_Phase_0_001_Microtasks_001_020_All_In_One_Delivery.zip`
- Outer checksum verified: `ff4cbbad4c7d8340db074908012b1fcc2f329d5a7d5621a844bfd678617e5ee5`
- Working repository baseline: `AeroCodex_repository_foundation_v0_001_microtasks_001_020.zip`
- Repository baseline checksum verified: `da5105ebed1395c056870da11ca702ffeeedd0305ce0d7d2d33a9343c426283b`
- Nested phase bundle checksum verified: `74773f922a434fc8e60b9bc5ac0c4eb2f8c94fdecc68291b8205204904b59d41`

## Session status table

| Microtask | Title | Current session status |
| --- | --- | --- |
| 1 | Repository Intake and Baseline Inventory | complete |
| 2 | Versioning and Roadmap Lock | complete |
| 3 | Core Result, Error, and Verification Types | complete |
| 4 | Minimal Unit-Safe Scalar Types | complete |
| 5 | Constants and Source Registry Seeds | complete |
| 6 | Codex Card Schema and Validation Scaffold | complete |
| 7 | Atmosphere v0.001 Equations | complete |
| 8 | Thermodynamics v0.001 Perfect Gas Equations | complete |
| 9 | Gas Dynamics v0.001 Isentropic Flow | complete |
| 10 | Gas Dynamics v0.001 Normal Shock | complete |
| 11 | Gas Dynamics v0.001 Mach Angle and Prandtl-Meyer | complete |
| 12 | Gas Dynamics v0.001 Oblique Shock Solver | complete |
| 13 | Aerodynamics v0.001 Basic Coefficients | complete |
| 14 | Propulsion v0.001 Rocket and Nozzle Basics | complete |
| 15 | Heat Transfer v0.001 | complete |
| 16 | Structures v0.001 Beam and Buckling Basics | complete |
| 17 | Flight Dynamics v0.001 Basic Performance | complete |
| 18 | Astrodynamics v0.001 Two-Body Basics | complete |
| 19 | Astrodynamics v0.001 Hohmann and Celestial Mechanics Helpers | complete |
| 20 | Bio-Regenerative Life Support v0.001 | complete |

## Microtask 1 — Repository Intake and Baseline Inventory

Status: complete in this session.

Actions completed:

- Verified the uploaded outer delivery checksum against its SHA256 sidecar.
- Extracted the outer delivery bundle into a local working area.
- Verified the nested repository foundation checksum against its SHA256 sidecar.
- Verified the nested phase bundle checksum against its SHA256 sidecar.
- Extracted `AeroCodex_repository_foundation_v0_001_microtasks_001_020.zip` as the current working baseline.
- Confirmed the repository foundation extracts with repository contents at ZIP root and no extra nested repository-root mistake.
- Inspected required root items: `Cargo.toml`, `crates/`, `docs/`, `validation/`, `xtask/`, `.github/`, `README.md`, and license files.
- Parsed all Cargo manifests with a temporary local manifest parser.
- Recorded crate inventory, missing crate status, local tool availability, and known limitations in `docs/phase_0_001/working_inventory.md`.
- Updated source-research backlog language for the current session.

Files changed:

- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`

Checks run:

- `sha256sum /mnt/data/AeroCodex_Phase_0_001_Microtasks_001_020_All_In_One_Delivery.zip`
- `sha256sum /mnt/data/aerocodex_work/input/AeroCodex_Phase_0_001_Microtasks_001_020_Bundle.zip`
- `sha256sum /mnt/data/aerocodex_work/input/AeroCodex_repository_foundation_v0_001_microtasks_001_020.zip`
- `unzip -l` on uploaded and nested ZIP files
- Required file and directory presence checks
- Cargo manifest parse of all workspace manifests
- Cargo dependency-name inventory across all manifests

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- No source-registry entry was upgraded from `research_required` during Microtask 1.
- Exact source editions, equation/table/page identifiers, and validation evidence still require later source review.

## Microtask 2 — Versioning and Roadmap Lock

Status: complete in this session.

Actions completed:

- Confirmed the root workspace uses `[workspace.package] version = "0.0.1"`.
- Confirmed all library crates and `xtask` inherit the workspace package version with `version.workspace = true`.
- Confirmed no Cargo manifest contains `version = "0.001"`.
- Expanded the roadmap docs to distinguish the human roadmap label `Phase 0.001` from Cargo semantic version `0.0.1`.
- Added a dedicated version-lock audit and Microtask 2 review note.
- Updated the README, docs index, and deployment prompt with version-lock sanity checks and reminders.
- Refreshed inventory/backlog documentation without upgrading any source-verification statuses.
- Regenerated the repository file manifest and file inventory after documentation edits.

Files changed:

- `README.md`
- `docs/index.md`
- `removed deployment prompt document`
- `docs/roadmap/versioning.md`
- `docs/roadmap/milestones.md`
- `docs/roadmap/post_1_0_expansion.md`
- `docs/phase_0_001/version_lock_audit.md`
- `docs/phase_0_001/microtask_002_versioning_review.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Workspace package version check for `0.0.1`.
- Workspace member version inheritance check for `version.workspace = true`.
- Static scan confirming no Cargo manifest contains Cargo package version `0.001`.
- Static scan confirming roadmap docs include all required roadmap levels.
- Static scan confirming roadmap docs include all required scope categories.
- Static caveat scan confirming roadmap docs do not imply premature Phase 1.0, mission, flight, operational, or certification readiness.
- Static Cargo manifest forbidden-dependency token scan.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- Microtask 2 is documentation and governance scoped; no equation source-registry entry was upgraded from `research_required`.
- Formula and validation-source details remain for later equation-specific microtasks.

Definition-of-done result:

The versioning plan is explicit, Cargo package versions remain `0.0.1`, `Phase 0.001` is documented as a human roadmap label only, and the roadmap docs preserve conservative non-readiness language.

## Microtask 3 — Core Result, Error, and Verification Types

Status: complete in this session.

Actions completed:

- Reviewed the existing `aero-codex-core` result/error/verification vocabulary against the Microtask 3 requirements.
- Confirmed and retained the shared `AeroResult<T>` alias.
- Refined `AeroError` with stable snake-case codes, optional parameter diagnostics, and a domain-error helper.
- Confirmed the required error variants are present: `NonPositiveInput`, `OutOfDomain`, `RequiresSupersonic`, `AmbiguousBranch`, `NumericalFailure`, and `UnverifiedSource`.
- Retained `NegativeInput` for nonnegative-domain checks where zero is valid.
- Added dependency-free `as_str`, `Display`, and `FromStr` support for `VerificationStatus` and `ValidityStatus`.
- Added `ParseStatusError`, helper constructors for assumptions/warnings/verification records, and convenience methods on `EngineeringResult<T>`.
- Changed the default `EngineeringResult<T>` validity to `NotAssessed` to avoid silently implying documented-domain validity.
- Updated the life-support closure-fraction result to explicitly mark `WithinDocumentedDomain` after its input checks.
- Added a Microtask 3 review note and updated the API summary.

Files changed:

- `crates/aero-codex-core/src/error.rs`
- `crates/aero-codex-core/src/result.rs`
- `crates/aero-codex-core/src/lib.rs`
- `crates/aero-codex-life-support/src/lib.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_003_core_result_error_verification.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan for all required Microtask 3 public type names.
- Static scan for all required `AeroError` variants.
- Static scan for all required `VerificationStatus` variants and snake-case text forms.
- Static scan confirming no new dependency was added to `aero-codex-core`.
- Static Cargo manifest forbidden-dependency token scan.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- Microtask 3 changed shared metadata/error types only; no equation source was reviewed.
- No source-registry entry or validation-card status was upgraded.
- The required Rust compile/test/doc checks remain mandatory in a Rust-equipped deployment environment.

Definition-of-done result:

The shared core result/error/verification vocabulary is implemented/refined, dependency-free, and covered by expanded in-crate tests pending execution under a Rust toolchain.

## Microtask 4 — Minimal Unit-Safe Scalar Types

Status: complete in this session.

Actions completed:

- Reviewed the existing `aero-codex-core` scalar wrappers against the Microtask 4 requirements.
- Confirmed the required exported types are present: `Angle`, `Mach`, `Gamma`, `Pressure`, `Temperature`, `Density`, `Length`, `Area`, `Mass`, `Time`, `Velocity`, `Acceleration`, `Force`, and `HeatFlux`.
- Documented canonical SI storage units and construction rules in the Rust source and the Microtask 4 review note.
- Retained `Angle::from_degrees`, `Angle::from_radians`, `Angle::as_degrees`, `Angle::as_radians`, `Angle::sin`, `Angle::cos`, and `Angle::tan`.
- Added `Angle::ZERO` for simple zero-angle construction without changing existing angle APIs.
- Confirmed `Mach::new` requires finite `M >= 0`.
- Confirmed `Gamma::new` requires finite `gamma > 1`.
- Confirmed `Pressure`, `Temperature`, `Density`, `Length`, `Area`, `Mass`, `Time`, `Velocity`, and `Acceleration` use checked finite nonnegative constructors.
- Preserved signed `Force` and `HeatFlux` semantics and added checked finite constructors for untrusted input.
- Expanded unit tests for trigonometry, finite-domain rejection, zero-boundary behavior, canonical SI getters, and signed scalar checked construction.
- Updated API and source-research documentation without upgrading source-verification statuses.

Files changed:

- `crates/aero-codex-core/src/units.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_004_unit_scalars.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan confirming `aero-codex-core` still has no external dependencies.
- Static scan confirming all required Microtask 4 scalar type names are present.
- Static scan confirming required angle, Mach, Gamma, pressure, temperature, and length method names are present.
- Static scan confirming nonnegative scalar constructors delegate to shared validation.
- Static scan confirming expanded Microtask 4 unit test markers are present.
- Static Cargo manifest forbidden-dependency token scan.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- Microtask 4 changed shared scalar wrapper APIs/tests only; no equation source was reviewed.
- No source-registry entry or validation-card status was upgraded.
- The required Rust compile/test/doc checks remain mandatory in a Rust-equipped deployment environment.

Definition-of-done result:

The first unit-safe scalar wrappers exist with tests and explicit canonical-unit documentation, pending execution under a Rust toolchain.

## Microtask 5 — Constants and Source Registry Seeds

Status: complete in this session.

Actions completed:

- Reviewed the existing `aero-codex-constants` crate against the Microtask 5 constant list.
- Retained the required seed constants for standard gravity, universal gas constant, standard sea-level pressure, temperature, density, standard dry-air gas constant, dry-air gamma, Stefan-Boltzmann constant, Earth gravitational parameter, and Earth mean radius.
- Retained the solar gravitational-parameter item only as an explicit unverified placeholder and added a boolean verification flag set to `false`.
- Added dependency-free `ConstantSeed` metadata and `PHASE_0_001_CONSTANT_SEEDS` so constants can point to conservative source-registry research targets without adding serialization or native dependencies.
- Expanded the required source-registry seed files for U.S. Standard Atmosphere 1976, NACA Report 1135/equivalent compressible-flow references, NASA Glenn/CEA thermodynamics, NASA/JPL astrodynamics parameters, and NASA BVAD/ECLSS life-support sources.
- Added a conservative NIST/CODATA physical-constants research-target seed for the universal gas constant and Stefan-Boltzmann constant.
- Confirmed all source-registry seeds remain `research_required` and do not claim exact page, table, equation, uncertainty, validation, certification, flight-readiness, operational-readiness, or mission-readiness status.
- Added the Microtask 5 review note and updated API, source-research, inventory, and index documentation.

Files changed:

- `crates/aero-codex-constants/src/lib.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_005_constants_source_registry.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`
- `validation/source_registry/naca_report_1135.yaml`
- `validation/source_registry/nasa_glenn_thermo_cea.yaml`
- `validation/source_registry/nasa_jpl_astrodynamics_parameters.yaml`
- `validation/source_registry/nasa_life_support_bvad_eclss.yaml`
- `validation/source_registry/nist_codata_physical_constants.yaml`
- `validation/source_registry/us_standard_atmosphere_1976.yaml`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan confirming `aero-codex-constants` still has no external dependencies.
- Static scan confirming all required Microtask 5 constants are present.
- Static scan confirming `ConstantSeed`, `PHASE_0_001_CONSTANT_SEEDS`, and `constant_seed` are present.
- Static scan confirming each public constant has seed metadata.
- Static scan confirming all constant metadata uses `research_required`.
- Static scan confirming all required source-registry seed files are present.
- Static scan confirming all source-registry YAML files remain `status: research_required`.
- Static scan confirming the solar gravitational-parameter placeholder is explicitly not verified.
- Static Cargo manifest forbidden-dependency token scan.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- Microtask 5 created/refined constants and source-registry seeds only; it did not verify exact source editions, equation numbers, table numbers, uncertainty metadata, or validation datasets.
- No source-registry entry or validation-card status was upgraded from `research_required`.
- The required Rust compile/test/doc checks remain mandatory in a Rust-equipped deployment environment.

Definition-of-done result:

Shared constants exist, source-registry seeds exist, and all source status remains conservative without overclaiming.


## Microtask 6 — Codex Card Schema and Validation Scaffold

Status: complete in this session.

Actions completed:

- Reviewed the existing Codex Card schema and validation scaffold.
- Tightened `validation/schema/codex_card.schema.json` with Draft 2020-12 metadata, required fields, nonempty list definitions, status enums, category enums, dotted-ID patterns, a structured source object, and `additionalProperties: false`.
- Added `validation/README.md` documenting validation directory layout, status ladder, scaffold commands, and card-authoring rules.
- Expanded `xtask` to support `verify --all`, `verify cards`, `verify source-registry`, and `dependency-policy`.
- Added dependency-free checks for schema markers, card top-level fields, nonempty card list sections, status/category values, source-registry ID linkage, source-registry required fields, duplicate source IDs, selected forbidden readiness markers, and forbidden native dependency tokens.
- Added unit-test scaffolding in `xtask/src/main.rs` for helper behavior pending Rust-toolchain execution.
- Updated README, docs index, API summary, deployment prompt, source-research backlog, and working inventory.
- Regenerated the repository file manifest and file inventory after edits.

Files changed:

- `README.md`
- `docs/index.md`
- `removed deployment prompt document`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_006_codex_card_schema_validation.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`
- `validation/README.md`
- `validation/schema/codex_card.schema.json`
- `xtask/src/main.rs`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan confirming `xtask` still has no external dependencies.
- JSON parse of `validation/schema/codex_card.schema.json`.
- Static scan confirming required schema markers, status strings, category strings, and `additionalProperties: false` are present.
- Static scan confirming every validation card has required top-level fields and nonempty required list sections.
- Static scan confirming every validation card references a source-registry ID that exists.
- Static scan confirming every source-registry file has required top-level fields and nonempty required list sections.
- Static scan confirming every validation card and source-registry file remains `status: research_required`.
- Static Cargo manifest forbidden-dependency token scan.
- Rough brace/parenthesis balance checks on changed Rust source.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- Microtask 6 changed validation-governance scaffolding only; no equation source was reviewed.
- No source-registry entry or validation-card status was upgraded from `research_required`.
- The required Rust compile/test/doc and `xtask` execution checks remain mandatory in a Rust-equipped deployment environment.

Definition-of-done result:

The Codex Card schema and validation scaffold are stricter, dependency-free, and documented, with static checks confirming current cards and source-registry seeds remain conservative.

## Microtask 7 — Atmosphere v0.001 Equations

Status: complete in this session.

Actions completed:

- Reviewed the existing `aero-codex-atmosphere` implementation against the Microtask 7 requirements.
- Hardened and documented the Phase 0.001 troposphere domain as `0 m <= altitude_m <= 11000 m`.
- Documented that geometric altitude is used directly as the standard-atmosphere altitude variable and that geometric/geopotential conversion is deferred.
- Added public troposphere domain constants and a `troposphere_state(altitude_m)` convenience helper.
- Added `verification_record(codex_id)` to expose conservative, dependency-free atmosphere trace metadata.
- Tightened `speed_of_sound(gamma, gas_constant, temperature)` so temperature must be strictly positive.
- Expanded atmosphere tests for sea-level constants, troposphere monotonic behavior, upper-bound acceptance, invalid altitude handling, invalid speed-of-sound inputs, state aggregation, and `ResearchRequired` verification metadata.
- Added `validation/cards/atmosphere_standard_troposphere.yaml` as a conservative validation-planning card.
- Updated README, docs index, API summary, validation README, source-research backlog, and working inventory.
- Regenerated the repository file manifest and file inventory after edits.

Files changed:

- `README.md`
- `crates/aero-codex-atmosphere/src/lib.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_007_atmosphere_equations.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`
- `validation/README.md`
- `validation/cards/atmosphere_standard_troposphere.yaml`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan confirming `aero-codex-atmosphere` depends only on `aero-codex-core` and `aero-codex-constants`.
- Static scan confirming all required Microtask 7 public function names are present.
- Static scan confirming public troposphere altitude domain constants are present.
- Static scan confirming invalid altitude handling covers negative, nonfinite, and above-domain values.
- Static scan confirming `speed_of_sound` uses shared validation helpers for gamma, gas constant, and temperature.
- Static scan confirming atmosphere `VerificationRecord` metadata remains `ResearchRequired`.
- Static scan confirming the new atmosphere validation card has required fields and an existing source-registry ID.
- Static scan confirming every validation card and source-registry file remains `status: research_required`.
- Static Cargo manifest forbidden-dependency token scan.
- Rough brace/parenthesis balance checks on changed Rust source.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- Microtask 7 implemented and hardened atmosphere equations only; it did not review exact source editions, equation numbers, table numbers, tolerances, uncertainty metadata, or validation datasets.
- The geometric/geopotential altitude convention remains a documented Phase 0.001 simplification.
- No source-registry entry or validation-card status was upgraded from `research_required`.
- The required Rust compile/test/doc and `xtask` execution checks remain mandatory in a Rust-equipped deployment environment.

Definition-of-done result:

The atmosphere crate has first Phase 0.001 atmosphere equations, documented model limits, conservative trace metadata, a validation-planning card, and expanded tests, with all source and validation statuses kept conservative.

## Microtask 8 — Thermodynamics v0.001 Perfect Gas Equations

Status: complete in this session.

Actions completed:

- Reviewed and refined the `aero-codex-thermo` crate for checked Phase 0.001 perfect-gas helpers.
- Added conservative `verification_record` metadata for the thermodynamics Codex IDs.
- Tightened temperature domain handling so both density and speed of sound require positive absolute temperature.
- Added finite-output guards that return `AeroError::NumericalFailure` for nonfinite derived values.
- Added `validation/cards/thermo_perfect_gas.yaml` and kept it at `research_required`.
- Refined the NASA Glenn/CEA source-registry seed as a future validation target without adding CEA/native/runtime dependencies.
- Updated README, docs index, API summary, validation README, source backlog, working inventory, file manifest, and file inventory.

Files changed:

- `README.md`
- `crates/aero-codex-thermo/src/lib.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_008_thermodynamics_perfect_gas.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`
- `validation/README.md`
- `validation/cards/thermo_perfect_gas.yaml`
- `validation/source_registry/nasa_glenn_thermo_cea.yaml`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan for all required Microtask 8 public function names.
- Static scan confirming positive temperature checks for density and speed of sound.
- Static scan confirming `cp > cv` handling.
- Static scan confirming positive molar-mass handling.
- Static scan confirming conservative verification metadata.
- Static validation-card and source-registry linkage checks.
- Static validation status check confirming all cards and source-registry files remain `research_required`.
- Static Cargo manifest forbidden-dependency token scan.
- Rough delimiter-balance check on changed Rust source.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- No thermodynamics source-registry entry or validation card was upgraded from `research_required`.
- NASA Glenn/CEA source editions, equation/page identifiers, property definitions, example values, and tolerances remain pending review.

Definition-of-done result:

The thermodynamics crate has checked basic perfect-gas equations and tests represented in source, with conservative validation metadata and no native/foreign dependency additions.

## Microtask 9 — Gas Dynamics v0.001 Isentropic Flow

Status: complete in this session.

Actions completed:

- Reviewed and refined direct isentropic perfect-gas helpers in `aero-codex-gas-dynamics`.
- Added a pure-Rust workspace dependency on `aero-codex-constants` to reuse the NACA/source-registry ID constant.
- Added conservative `verification_record(codex_id)` metadata for the Microtask 9 isentropic gas-dynamics Codex IDs.
- Added `CODEX_ID_ISENTROPIC_MASS_FLOW_PARAMETER`.
- Added finite-output guards that return `AeroError::NumericalFailure` for nonfinite derived isentropic values.
- Documented domain rules: `Mach >= 0` for ratios and mass-flow parameter, `Mach > 0` for area-Mach ratio, and `gamma > 1` for all isentropic helpers.
- Documented that inverse area-Mach branch solving is deferred to a later microtask.
- Added `validation/cards/gasdyn_isentropic_flow.yaml` and kept it at `research_required`.
- Refined the NACA Report 1135/equivalent source-registry seed as a future validation target without upgrading source status.
- Updated README, docs index, API summary, validation README, source backlog, working inventory, file manifest, and file inventory.

Files changed:

- `README.md`
- `crates/aero-codex-gas-dynamics/Cargo.toml`
- `crates/aero-codex-gas-dynamics/src/lib.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_009_gas_dynamics_isentropic.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`
- `validation/README.md`
- `validation/cards/gasdyn_isentropic_flow.yaml`
- `validation/source_registry/naca_report_1135.yaml`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan confirming `aero-codex-gas-dynamics` depends only on workspace Rust crates.
- Static scan for all required Microtask 9 isentropic public function names.
- Static scan confirming isentropic Codex ID constants and conservative trace metadata.
- Static scan confirming gamma and Mach domain checks.
- Static scan confirming finite-output guards.
- Static validation-card and source-registry linkage checks.
- Static validation status check confirming all cards and source-registry files remain `research_required`.
- Static Cargo manifest forbidden-dependency token scan.
- Rough delimiter-balance check on changed Rust source.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- No gas-dynamics source-registry entry or validation card was upgraded from `research_required`.
- NACA Report 1135/equivalent source editions, equation/page/table identifiers, area-Mach branch conventions, reference values, and tolerances remain pending review.
- At that point, later gas-dynamics features already present in the imported baseline, including Mach angle, Prandtl-Meyer, and oblique-shock helpers, remained pending their dedicated Microtask 11-12 reviews.

Definition-of-done result:

The gas-dynamics crate has checked, documented, and source-test-scaffolded direct isentropic functions, with conservative validation metadata and no source-status upgrade.

## Microtask 10 — Gas Dynamics v0.001 Normal Shock

Status: complete in this session.

Actions completed:

- Reviewed and refined direct normal-shock perfect-gas helpers in `aero-codex-gas-dynamics`.
- Added Codex ID constants for downstream Mach, static pressure ratio, static density ratio, static temperature ratio, and total-pressure ratio.
- Added conservative `verification_record(codex_id)` metadata for the Microtask 10 normal-shock Codex IDs.
- Documented domain rules: `mach1 > 1` and `gamma > 1`.
- Preserved `AeroError::RequiresSupersonic` behavior for sonic and subsonic upstream Mach numbers.
- Added finite-output guards and upstream Mach-square overflow checks that report `AeroError::NumericalFailure` for nonfinite derived values.
- Added `validation/cards/gasdyn_normal_shock.yaml` and kept it at `research_required`.
- Refined the NACA Report 1135/equivalent source-registry seed as a future normal-shock validation target without upgrading source status.
- Updated README, docs index, API summary, validation README, source backlog, working inventory, file manifest, and file inventory.

Files changed:

- `README.md`
- `crates/aero-codex-gas-dynamics/src/lib.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_010_gas_dynamics_normal_shock.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`
- `validation/README.md`
- `validation/cards/gasdyn_normal_shock.yaml`
- `validation/source_registry/naca_report_1135.yaml`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan confirming `aero-codex-gas-dynamics` depends only on workspace Rust crates.
- Static scan for all required Microtask 10 normal-shock public function names.
- Static scan confirming normal-shock Codex ID constants and conservative trace metadata.
- Static scan confirming `mach1 > 1` and `gamma > 1` domain checks.
- Static scan confirming finite-output guards and upstream Mach-square overflow checks.
- Static validation-card and source-registry linkage checks.
- Static validation status check confirming all cards and source-registry files remain `research_required`.
- Static Cargo manifest forbidden-dependency token scan.
- Rough delimiter-balance check on changed Rust source.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- No gas-dynamics source-registry entry or validation card was upgraded from `research_required`.
- NACA Report 1135/equivalent source editions, normal-shock equation/page/table identifiers, reference values, total-pressure convention, and tolerances remain pending review.
- At that point, later gas-dynamics features already present in the imported baseline, including Mach angle, Prandtl-Meyer, and oblique-shock helpers, remained pending their dedicated Microtask 11-12 reviews.

Definition-of-done result:

The gas-dynamics crate has checked, documented, and source-test-scaffolded direct normal-shock functions, with conservative validation metadata and no source-status upgrade.

## Microtask 11 — Gas Dynamics v0.001 Mach Angle and Prandtl-Meyer

Status: complete in this session.

Actions completed:

- Reviewed and refined Mach-angle and Prandtl-Meyer expansion-flow helpers in `aero-codex-gas-dynamics`.
- Added Codex ID constants for Mach angle, Prandtl-Meyer forward, and Prandtl-Meyer inverse helpers.
- Added conservative `verification_record(codex_id)` metadata for the Microtask 11 expansion-flow Codex IDs.
- Documented domain rules: `Mach >= 1` for `mach_angle` and `prandtl_meyer_nu`; `gamma > 1` for Prandtl-Meyer helpers; finite `0 <= nu < nu_max(gamma)` and positive finite tolerance for the inverse solve.
- Preserved `AeroError::RequiresSupersonic` behavior for subsonic inputs.
- Added finite intermediate/output guards and explicit `AeroError::NumericalFailure` reporting for inverse bracketing and bisection failures.
- Added `validation/cards/gasdyn_mach_angle_prandtl_meyer.yaml` and kept it at `research_required`.
- Refined the NACA Report 1135/equivalent source-registry seed as a future expansion-flow validation target without upgrading source status.
- Updated README, docs index, API summary, validation README, source backlog, working inventory, file manifest, and file inventory.

Files changed:

- `README.md`
- `crates/aero-codex-gas-dynamics/src/lib.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`
- `validation/README.md`
- `validation/cards/gasdyn_mach_angle_prandtl_meyer.yaml`
- `validation/source_registry/naca_report_1135.yaml`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan confirming `aero-codex-gas-dynamics` depends only on `aero-codex-core` and `aero-codex-constants`.
- Static scan for all required Microtask 11 public function names.
- Static scan confirming Mach-angle and Prandtl-Meyer Codex/source constants and conservative trace metadata.
- Static scan confirming `Mach >= 1`, `gamma > 1`, `nu >= 0`, and positive tolerance validation markers.
- Static scan confirming inverse solve bracketing and bisection numerical-failure guards.
- Static validation-card and source-registry linkage checks.
- Static validation status check confirming all cards and source-registry files remain `research_required`.
- Static Cargo manifest forbidden-dependency token scan.
- Rough delimiter-balance check on changed Rust source.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- No gas-dynamics source-registry entry or validation card was upgraded from `research_required`.
- NACA Report 1135/equivalent source editions, Mach-angle and Prandtl-Meyer equation/page/table identifiers, reference values, inverse-solver tolerance targets, and comparison tables remain pending review.
- Oblique-shock helpers already present in the imported baseline were left to the dedicated Microtask 12 review, now recorded below.

Definition-of-done result:

The gas-dynamics crate has checked, documented, and source-test-scaffolded Mach-angle and Prandtl-Meyer forward/inverse functions, with conservative validation metadata and no source-status upgrade.

## Microtask 12 — Gas Dynamics v0.001 Oblique Shock Solver

Status: complete in this session.

Actions completed:

- Reviewed and refined branch-explicit oblique-shock helpers in `aero-codex-gas-dynamics`.
- Confirmed oblique-shock Codex IDs for the theta-beta-Mach residual, beta solve, normal-Mach component, and downstream-Mach projection.
- Confirmed `oblique_shock_beta` requires explicit `ShockBranch::Weak` or `ShockBranch::Strong` selection.
- Confirmed `mach > 1`, `gamma > 1`, finite deflection-angle, beta-range, normal-Mach, and downstream-projection checks.
- Confirmed no-attached-solution cases return `AeroError::NumericalFailure` rather than `NaN` or a silent weak/strong fallback.
- Added `validation/cards/gasdyn_oblique_shock.yaml` and kept it at `research_required`.
- Refined the NACA Report 1135/equivalent source-registry seed as a future oblique-shock validation target without upgrading source status.
- Updated README, docs index, API summary, validation README, source backlog, working inventory, file manifest, and file inventory.

Files changed:

- `README.md`
- `crates/aero-codex-gas-dynamics/src/lib.rs`
- `docs/index.md`
- `docs/phase_0_001/api_summary.md`
- `docs/phase_0_001/microtask_012_gas_dynamics_oblique_shock.md`
- `docs/phase_0_001/working_inventory.md`
- `docs/phase_0_001/source_research_backlog.md`
- `docs/phase_0_001/microtask_log.md`
- `docs/phase_0_001/file_manifest.md`
- `docs/phase_0_001/file_inventory.csv`
- `validation/README.md`
- `validation/cards/gasdyn_oblique_shock.yaml`
- `validation/source_registry/naca_report_1135.yaml`

Checks run:

- Cargo manifest parse of all workspace manifests.
- Static scan confirming `aero-codex-gas-dynamics` depends only on `aero-codex-core` and `aero-codex-constants`.
- Static scan for all required Microtask 12 public function names and `ShockBranch` variants.
- Static scan confirming oblique-shock Codex/source constants and conservative trace metadata.
- Static scan confirming `mach > 1`, `gamma > 1`, explicit branch, no-attached-solution numerical-failure, and downstream projection checks.
- Static validation-card and source-registry linkage checks.
- Static validation status check confirming all cards and source-registry files remain `research_required`.
- Static Cargo manifest forbidden-dependency token scan.
- Rough delimiter-balance check on changed Rust source.

Could not run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Reason: Rust tooling is unavailable in this environment (`rustc`, `cargo`, `rustfmt`, and `clippy-driver` were not found).

Source verification gaps:

- No gas-dynamics source-registry entry or validation card was upgraded from `research_required`.
- NACA Report 1135/equivalent source editions, theta-beta-Mach equation/page/table identifiers, detachment-limit values, weak/strong branch references, downstream-Mach examples, and tolerances remain pending review.
- Detached-shock modeling and shock-polar construction remain out of scope for Phase 0.001.

Definition-of-done result:

The gas-dynamics crate has checked, branch-explicit, and source-test-scaffolded oblique-shock helpers, with conservative validation metadata and no source-status upgrade.

## Microtask 13 — Aerodynamics v0.001 Basic Coefficients

Status: complete in this session.

Summary:

- Reviewed `aero-codex-aerodynamics` scalar dynamic-pressure, lift, drag, inverse coefficient, and induced-drag helpers.
- Added conservative `verification_record(codex_id)` metadata for the reviewed aerodynamic Codex IDs.
- Added finite-output guards so overflow/nonfinite arithmetic returns `AeroError::NumericalFailure`.
- Added `validation/cards/aerodynamics_basic_coefficients.yaml` and `validation/source_registry/aerodynamics_basic_coefficients.yaml`; both remain `research_required`.
- Preserved pure-Rust dependency policy: the crate depends only on `aero-codex-core`.

Checks completed in this environment:

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-aerodynamics` depends only on `aero-codex-core`.
- Confirmed required Microtask 13 public function names and Codex/source markers are present.
- Confirmed domain-check markers for nonnegative density/velocity/q/area, positive inverse q/area, positive aspect ratio, and positive Oswald efficiency.
- Confirmed finite-output numerical-failure guards are present.
- Confirmed aerodynamics validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native dependency token scan across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

Not run because this environment does not have Rust tooling:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Source verification gaps:

- Exact source edition, equation IDs, reference-area conventions, sign conventions, aspect-ratio conventions, Oswald-efficiency convention, representative examples, and tolerances remain pending source review.
- No aerodynamics source-registry entry or validation card was upgraded from `research_required`.

Definition-of-done result:

The aerodynamics crate has checked basic force/coefficient equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.

## Microtask 14 — Propulsion v0.001 Rocket and Nozzle Basics

Status: complete in this session.

Summary:

- Reviewed `aero-codex-propulsion` scalar rocket/nozzle helpers.
- Added conservative `verification_record(codex_id)` metadata for Tsiolkovsky delta-v, inverse mass ratio, ideal thrust, specific impulse from effective exhaust velocity, and choked mass flux per area.
- Added finite-output guards so overflow/nonfinite derived arithmetic returns `AeroError::NumericalFailure`.
- Added `validation/cards/propulsion_rocket_nozzle_basics.yaml` and `validation/source_registry/propulsion_rocket_nozzle_basics.yaml`; both remain `research_required`.
- Preserved pure-Rust dependency policy: the crate depends only on `aero-codex-core`.

Checks completed in this environment:

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-propulsion` depends only on `aero-codex-core`.
- Confirmed required Microtask 14 public function names and Codex/source markers are present.
- Confirmed domain-check markers for positive Isp, positive `g0`, `initial_mass > final_mass > 0`, nonnegative delta-v, nonnegative thrust inputs, `gamma > 1`, positive gas constant, nonnegative stagnation pressure, and positive stagnation temperature.
- Confirmed finite-output numerical-failure guards are present.
- Confirmed propulsion validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency token scan across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

Not run because this environment does not have Rust tooling:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Source verification gaps:

- Exact source edition, equation IDs, pressure-thrust sign convention, standard-gravity convention, choked-flow assumptions, representative examples, and tolerances remain pending source review.
- No propulsion source-registry entry or validation card was upgraded from `research_required`.

Definition-of-done result:

The propulsion crate has checked first rocket/nozzle equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.

## Microtask 15 — Heat Transfer v0.001 Radiation, Convection, and Conduction

Status: complete in this session.

Summary:

- Reviewed `aero-codex-heat-transfer` scalar radiation, convection, and conduction helpers.
- Added conservative `verification_record(codex_id)` metadata for radiative flux, convective flux, conduction resistance, and conduction heat rate.
- Added finite-output guards so overflow/nonfinite derived arithmetic returns `AeroError::NumericalFailure`.
- Added `validation/cards/heat_transfer_basic_primitives.yaml` and `validation/source_registry/heat_transfer_basic_primitives.yaml`; both remain `research_required`.
- Preserved pure-Rust dependency policy: the crate depends only on `aero-codex-core` and `aero-codex-constants`.

Checks completed in this environment:

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-heat-transfer` depends only on `aero-codex-core` and `aero-codex-constants`.
- Confirmed required Microtask 15 public function names and Codex/source markers are present.
- Confirmed domain-check markers for emissivity range, nonnegative absolute temperatures, nonnegative heat-transfer coefficient, nonnegative thickness, positive conductivity, positive area, and positive resistance.
- Confirmed finite-output numerical-failure guards are present.
- Confirmed heat-transfer validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency token scan across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

Not run because this environment does not have Rust tooling:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Source verification gaps:

- Exact source edition, equation IDs, radiative sign convention, view-factor convention, convective temperature convention, conduction geometry convention, representative examples, and tolerances remain pending source review.
- No heat-transfer source-registry entry or validation card was upgraded from `research_required`.

Definition-of-done result:

The heat-transfer crate has checked first radiation, convection, and conduction equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.

## Microtask 16 — Structures v0.001 Beam and Buckling Basics

Status: complete in this session.

Summary:

- Reviewed `aero-codex-structures` scalar axial stress, bending stress, cantilever end-load tip deflection, and Euler column buckling helpers.
- Added conservative `verification_record(codex_id)` metadata for the reviewed structures helpers.
- Added finite-output guards so overflow/nonfinite derived arithmetic returns `AeroError::NumericalFailure`.
- Added `validation/cards/structures_beam_buckling_basics.yaml` and `validation/source_registry/structures_basic_mechanics.yaml`; both remain `research_required`.
- Preserved pure-Rust dependency policy: the crate depends only on `aero-codex-core`.

Checks completed in this environment:

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-structures` depends only on `aero-codex-core`.
- Confirmed required Microtask 16 public function names and Codex/source markers are present.
- Confirmed domain-check markers for finite signed force/moment coordinates, positive area, positive length, positive elastic modulus, positive second moment of area, and positive effective length factor.
- Confirmed finite-output numerical-failure guards are present.
- Confirmed structures validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency token scan across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

Not run because this environment does not have Rust tooling:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Source verification gaps:

- Exact source edition, equation IDs, end-condition convention, sign conventions, representative examples, material assumptions, section-axis definitions, material allowables, finite-element comparisons, and tolerances remain pending source review.
- No structures source-registry entry or validation card was upgraded from `research_required`.

Definition-of-done result:

The structures crate has checked first stress, beam-deflection, and Euler-buckling equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.

## Microtask 17 — Flight Dynamics v0.001 Basic Performance

Status: complete in this session.

Summary:

- Reviewed `aero-codex-flight-dynamics` scalar level-turn, stall-speed, and specific-excess-power helpers.
- Added conservative `verification_record(codex_id)` metadata for the reviewed flight-dynamics helpers.
- Tightened bank-angle domain checking to finite `|bank_angle| < 90 degrees` for level-turn helpers.
- Added finite-output guards so overflow/nonfinite derived arithmetic returns `AeroError::NumericalFailure`.
- Added `validation/cards/flight_dynamics_basic_performance.yaml` and `validation/source_registry/flight_dynamics_basic_performance.yaml`; both remain `research_required`.
- Preserved pure-Rust dependency policy: the crate depends only on `aero-codex-core`.

Checks completed in this environment:

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-flight-dynamics` depends only on `aero-codex-core`.
- Confirmed required Microtask 17 public function names and Codex/source markers are present.
- Confirmed domain-check markers for bank-angle, positive gravity/velocity, positive stall-speed denominators, nonnegative specific-excess-power velocity, and finite signed thrust/drag.
- Confirmed finite-output numerical-failure guards are present.
- Confirmed flight-dynamics validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency token scan across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

Not run because this environment does not have Rust tooling:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Source verification gaps:

- Exact flight-dynamics/performance source edition, equation IDs, coordinated-turn convention, zero-bank/infinite-radius convention, stall-speed convention, specific-excess-power convention, representative examples, and tolerances remain pending source review.
- No flight-dynamics source-registry entry or validation card was upgraded from `research_required`.

Definition-of-done result:

The flight-dynamics crate has checked first coordinated-turn, stall-speed, and specific-excess-power equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.

## Microtask 18 — Astrodynamics v0.001 Two-Body Basics

Status: complete in this session.

Summary:

- Reviewed `aero-codex-astrodynamics` scalar two-body helpers: circular orbit speed, circular orbital period, escape velocity, vis-viva speed, and specific orbital energy.
- Added conservative `verification_record(codex_id)` metadata for the reviewed two-body helpers.
- Added finite-output guards so overflow/nonfinite derived arithmetic returns `AeroError::NumericalFailure`.
- Added `validation/cards/astrodynamics_two_body_basics.yaml` and `validation/source_registry/astrodynamics_two_body_basics.yaml`, and added `validation/source_registry/astrodynamics_two_body_basics.yaml` and updated the existing `validation/source_registry/nasa_jpl_astrodynamics_parameters.yaml`; all remain `research_required`.
- Preserved pure-Rust dependency policy: the crate depends only on `aero-codex-core` and `aero-codex-constants`.
- Left Hohmann-transfer and sphere-of-influence helpers present but reserved for Microtask 19 review.

Checks completed in this environment:

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-astrodynamics` depends only on `aero-codex-core` and `aero-codex-constants`.
- Confirmed required Microtask 18 public function names and Codex/source markers are present.
- Confirmed positive-domain validation markers for `mu`, `radius`, and `semi_major_axis`.
- Confirmed finite-output numerical-failure guards are present.
- Confirmed the astrodynamics validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency token scan across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

Not run because this environment does not have Rust tooling:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Source verification gaps:

- Exact astrodynamics source edition or dataset, equation IDs, gravitational-parameter epoch/uncertainty, radius convention, sign convention for specific energy, representative examples, and tolerances remain pending source review.
- No astrodynamics source-registry entry or validation card was upgraded from `research_required`.

Definition-of-done result:

The astrodynamics crate has checked first two-body equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.

## Microtask 19 — Astrodynamics v0.001 Hohmann and Celestial Mechanics Helpers

Status: complete in this session.

Summary:

- Reviewed `aero-codex-astrodynamics` scalar Hohmann-transfer and sphere-of-influence helpers.
- Added Codex IDs and conservative `verification_record(codex_id)` metadata for `hohmann_transfer_delta_v1`, `hohmann_transfer_delta_v2`, `hohmann_transfer_total_delta_v`, `hohmann_transfer_time`, and `sphere_of_influence_radius`.
- Added finite-output and intermediate overflow guards so nonfinite derived arithmetic returns `AeroError::NumericalFailure`.
- Added `validation/cards/astrodynamics_transfer_celestial_basics.yaml` and `validation/source_registry/astrodynamics_transfer_celestial_basics.yaml`; all validation/source statuses remain `research_required`.
- Preserved pure-Rust dependency policy: the crate depends only on `aero-codex-core` and `aero-codex-constants`.

Checks completed in this environment:

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-astrodynamics` depends only on `aero-codex-core` and `aero-codex-constants`.
- Confirmed required Microtask 19 public function names and Codex/source markers are present.
- Confirmed positive-domain validation markers for `mu`, `r1`, `r2`, `primary_distance`, `secondary_mass`, and `primary_mass`.
- Confirmed finite-output numerical-failure guards are present.
- Confirmed the transfer/celestial astrodynamics validation card links to an existing source-registry seed.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency token scan across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

Not run because this environment does not have Rust tooling:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Source verification gaps:

- Exact Hohmann-transfer source edition, equation IDs, burn sign/magnitude convention, circular-radius convention, same-radius boundary handling, sphere-of-influence source form, mass/distance conventions, representative examples, and tolerances remain pending source review.
- No astrodynamics source-registry entry or validation card was upgraded from `research_required`.

Definition-of-done result:

The astrodynamics crate has checked first transfer and celestial helper equations, source-test scaffolding, conservative validation metadata, and no source-status upgrade.

## Microtask 20 — Bio-Regenerative Life Support v0.001

Status: complete in this session.

Summary:

- Reviewed `aero-codex-life-support` scalar bio-regenerative mass-balance helpers: `closure_fraction`, `required_production_area`, `buffer_residence_time`, `crew_daily_requirement`, `net_daily_balance`, `oxygen_balance`, `carbon_dioxide_balance`, and `water_recovery_balance`.
- Added/confirmed conservative `verification_record(codex_id)` metadata for all reviewed life-support Codex IDs.
- Added finite-output guards so overflow/nonfinite derived arithmetic returns `AeroError::NumericalFailure`.
- Preserved warning-bearing `EngineeringResult<f64>` behavior for closure-style fractions and water-recovery balance; values above one are returned with warnings and `ValidityStatus::OutsideDocumentedDomain` rather than being clipped or silently treated as ordinary closure.
- Reviewed the required life-support validation-card seeds for closure fraction, required production area, and buffer residence time; retained group coverage for crew/daily balance and optional O2/CO2/water wrappers.
- Preserved pure-Rust dependency policy: the crate depends only on `aero-codex-core`.
- Completed the Phase 0.001 interactive review sequence through Microtask 20.

Checks completed in this environment:

- Parsed all Cargo manifests with a temporary local manifest parser.
- Confirmed `aero-codex-life-support` depends only on `aero-codex-core`.
- Confirmed required Microtask 20 public function names and Codex/source markers are present.
- Confirmed nonnegative/positive domain validation markers for recycled/required rates, production-area inputs, buffer/flow inputs, crew demand, net balance, gas balance, and water recovery inputs.
- Confirmed finite-output numerical-failure guards are present.
- Confirmed life-support validation cards link to existing source-registry seeds.
- Confirmed validation-card IDs and source-registry IDs are unique after duplicate baseline card cleanup.
- Confirmed all validation cards and source-registry files remain `status: research_required`.
- Ran static forbidden native/wrapper dependency token scan across Cargo manifests.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

Not run because this environment does not have Rust tooling:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p xtask -- verify --all`
- `cargo run -p xtask -- verify cards`
- `cargo run -p xtask -- verify source-registry`
- `cargo run -p xtask -- dependency-policy`
- `cargo doc --workspace --all-features --no-deps`

Source verification gaps:

- Exact NASA BVAD/ECLSS/ALS source editions, crop-productivity references, equation/table/page locators, units, accounting boundaries, storage and quality assumptions, representative examples, and tolerances remain pending source review.
- No life-support source-registry entry or validation card was upgraded from `research_required`.

Definition-of-done result:

The life-support crate has checked first bio-regenerative scalar mass-balance primitives, source-test scaffolding, conservative validation metadata, and no source-status upgrade.
