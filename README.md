# AeroCodex

<p align="center">
  <img src="assets/aerocodex_patch.png" alt="AeroCodex project patch showing a terminal prompt, rocket, orbital paths, and a RESEARCH READY / MEMORY SAFE banner." width="220">
</p>

**Source-traceable aerospace, astrodynamics, and bio-regenerative life-support mathematics in pure Rust.**

AeroCodex is a Phase 0.001 Rust workspace for research, education, verification-oriented development, and preliminary design. The human roadmap phase is `Phase 0.001`; the Cargo-compatible package version for all workspace packages during this phase is `0.0.1`. Do not use `0.001` as a Cargo package version.

## Safety and certification caveat

AeroCodex is **not** certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

Use it as research/preliminary-design engineering mathematics only. Safety-critical, regulated, operational, crewed, habitat, medical, or mission use requires independent project-specific assurance, validation, qualification, and certification.

## Pure Rust policy

The core repository is intentionally pure Rust. It does not include C/C++/Fortran source, BLAS/LAPACK native linkage, CEA/REFPROP/CoolProp/Cantera wrappers, Python/Matlab/Julia runtime dependencies, `bindgen`, `cc`, `cmake`, `pkg-config`, `vcpkg`, native binary blobs, or generated binaries.

## Current governed state

This README reflects current `main` after compiler-batch coverage was completed for all 152 existing Rust equation runtimes, the 27 formula-vault intake records were linked to governed runtimes, and A11-A31 assigned terminal dispositions to eighteen bounded external-backlog waves. The live governed counters remain verifier-derived; these waves change no runtime kernel count and leave every linked formula at `research_required`:

| Inventory class | Count | Meaning |
|---|---:|---|
| Executable research equations | 152 | Public Rust research/preliminary-design equation kernels inventoried by `validation/equation_inventory.tsv`. |
| Metadata-only formula-vault candidate records | 27 | Intake/provenance records; A10 links 27/27 to existing governed runtimes, leaving 0 unresolved candidate formula IDs. The metadata files are not implementations by themselves. |
| External M07 rows with terminal dispositions | 786 | A11-A31: 127 deduplicated aliases, 103 excluded internal/composite helpers, and 554 contract- or policy-blocked rows. |
| External M07 backlog rows | 198 | Registered external M07 represented rows that still lack a terminal disposition. |
| Validation cards | 46 | Conservative validation/governance records. They are not certification evidence. |
| Source-registry seeds | 44 | Source/governance traceability seeds. |
| Validation-card-only records | 46 | Metadata records, not formula implementations. |
| Helper algorithms | 262 | Support routines not counted as executable research equations. |

A10 runtime-resolution status: `linked_to_existing_runtime=27`, `unresolved=0`, with 3 angle/unit, 14 vector-algebra, and 10 canonical-unit links in `formula-vault/resolutions/m00_runtime_links.tsv`. A11-A31 external status: `terminal_dispositions=786`, `deduplicated_aliases=127`, `excluded_internal_helpers=103`, and `contract_or_policy_blocked=554`, across twenty-one bounded `formula-vault/resolutions/m07_*.tsv` manifests. A14-A15 complete the 49-row classical two-body algebra group. A16-A25 complete all 377 source-ordered rows of orbital-geometry and conic-branch review. A26-A27 complete the governed coordinate-transform / frame-graph / time-scale policy backlog with 85 contract- or policy-blocked rows while preserving each classifier risk tier; none of these records claims M07/Scilab parity, certification, or operational readiness.

Stage 5 and immediate post-Stage-5 work have deployed several bounded, adapted slices: Chunk 0 intake/queue baseline, Session D policy/templates and taxonomy remediation, Session C1 documentation/policy adaptation, Session C2 classifier planning metadata, Session B canonical-unit scalar expansion, Orekit v3 O2a time/frame/state foundation, Orekit v3 O2b classical-elements/Kepler research foundation, Orekit v3 O2c oracle-record/tolerance-comparison helpers, Orekit v3 O2d two-line-element contract/source-policy metadata, BioSim v3 corrected B2a scenario-domain and structural-validation foundation, BioSim v3 B2b-1 process-types/validated-constructor/intent-planner foundation, BioSim v3 B2b-2 bounded compartment replay/compact-digest/atomic-event foundation, BioSim v3 B2c replay-integrity/ledger/report/example/governance, adapted Session E BioSim-plus docs/contracts, Session G friend-test material, Session A wrap2pi endpoint contract/test metadata, professional hardening slices, adapted Session F Orekit reference-oracle planning metadata, a docs/governance-only final Stage 5 closeout/status consolidation, and the bounded post-Stage-5 `m00_wrap2pi` runtime deployment.

These deployments do **not** complete deep Orekit or deep BioSim work. Orekit O2b provides bounded research/preliminary-only classical-elements, elliptic-Kepler, and deterministic smoke-example support. Orekit O2c provides validated oracle-record construction and local deterministic tolerance-comparison helpers only. Orekit O2d provides contract-only two-line-element source-policy metadata and fail-closed prerequisite evaluation only. O2c/O2d do not execute Orekit, verify external evidence hashes, import external fixtures, parse two-line-element records, implement checksums, decode epochs or orbital fields, run SGP4, perform TEME transforms, propagate orbits, or claim parity. BioSim B2a provides clean-room synthetic scenario-domain records and deterministic structural validation only. BioSim B2b-1 adds clean-room synthetic process identifiers, source/sink/transfer/transform process records, validated constructors, deterministic one-tick intent planning, and bounded planner guards only. BioSim B2b-2 adds bounded deterministic compartment replay, immutable replay events/tick summaries/final cells, explicit requested/committed/clamp semantics, fail-closed initialization, atomic per-tick commit, and compact noncryptographic Fowler-Noll-Vo 1a state digests only. BioSim B2c consumes accepted B2b-2 replay records directly and adds fail-closed replay-integrity validation, deterministic per-resource ledger accounting from committed event amounts, explicit clamp accounting, ledger self-consistency validation, deterministic path-safe report formatting, and one visibly synthetic package example. B2c does not add a flat-resource adapter, complete scenario engine, controller, biological-fidelity model, habitat-safety model, medical model, operational system, external BioSim parity, certification evidence, or regulated-use approval. Optional BioSim B2b-3 is skipped/not required for the B2c consumer path unless a future separate adapter-proof prompt reopens it. `m00_wrap2pi` is deployed as a bounded Rust research/preliminary public API for `formula_vault.m00.angle.wrap2pi`; it validates finite inputs, uses `rem_euclid(std::f64::consts::TAU)`, returns [0, TAU), canonicalizes zero to positive zero, repairs only exact `TAU` remainder-roundoff to the greatest representable value below `TAU`, rejects nonfinite inputs, does not apply epsilon or ordinary-value clamping, and makes no M07/Scilab parity claim. `app_resolve_coplanar` remains blocked for a separate least-squares/rank/tolerance policy. The M07 source artifact remains quarantined source material: it reports 1,350 represented function rows, 1,333 C2 classifier rows, and 188 source-file-level equivalence jobs as metadata, but it is not bulk-merged into public APIs.

## What AeroCodex can do now

AeroCodex currently provides source-traceable research kernels for common engineering calculations across atmosphere, thermodynamics, gas dynamics, aerodynamics, propulsion, heat transfer, structures, flight dynamics, astrodynamics, and bio-regenerative life-support scaffolding.

It also provides governance machinery:

- validation cards and source-registry seeds;
- a data/source registry for external materials and in-repo artifact hashes;
- a formula-vault intake/provenance path plus an explicit runtime-resolution manifest for M07-derived candidate records;
- an equation inventory/readiness dashboard;
- a nomenclature/acronym policy and generated terminology index;
- clean-room BioSim-RS-style resource identity, transaction, deterministic replay, ledger, and smoke/friend-test primitives;
- clean-room BioSim-plus synthetic scenario-domain records, deterministic structural validation, process records, one-tick intent-planning helpers, bounded compartment replay/digest/event helpers, and B2c replay-integrity/ledger/report helpers for research metadata only.
- a bounded `aerocodex` Beta 1 concept CLI for deterministic text/JSON execution and self-checking of the ten governed M00 canonical-unit formulas.

AeroCodex does **not** currently provide certified flight software, a complete BioSim scenario engine, an operational BLSS controller, a validated habitat-safety model, a medical model, or certified M07/Orekit parity.

## Workspace crates

| Crate | Current role |
|---|---|
| `aero-codex-core` | Shared result, error, validation, traceability, and scalar unit types. |
| `aero-codex-constants` | Phase 0.001 constants and source seeds. |
| `aero-codex-atmosphere` | Sea-level, simplified troposphere, density, pressure, temperature, and speed-of-sound helpers. |
| `aero-codex-thermo` | Perfect-gas density, speed of sound, heat-capacity, and molar-mass gas-constant helpers. |
| `aero-codex-gas-dynamics` | Isentropic, normal-shock, Mach-angle, Prandtl-Meyer, and branch-explicit oblique-shock relations. |
| `aero-codex-aerodynamics` | Dynamic pressure, lift, drag, coefficient inverses, and induced-drag helpers. |
| `aero-codex-propulsion` | Rocket equation, ideal thrust, specific impulse, and ideal choked mass-flux helpers. |
| `aero-codex-heat-transfer` | Stefan-Boltzmann radiation, Newton-law convection, and one-dimensional conduction helpers. |
| `aero-codex-structures` | Axial stress, bending stress, cantilever end-load deflection, and Euler column buckling helpers. |
| `aero-codex-flight-dynamics` | Level-turn, stall-speed, turn-rate/radius, and specific-excess-power helpers. |
| `aero-codex-astrodynamics` | Two-body orbital helpers, Hohmann transfer helpers, sphere of influence, bounded M00 angle/unit/vector helpers including `m00_wrap2pi`, bounded classical-elements and elliptic-Kepler research helpers, O2c oracle-record/tolerance-comparison metadata helpers, O2d contract-only two-line-element source-policy helpers, and runtime-linked formula-vault intake records. |
| `aero-codex-life-support` | BLSS mass-balance helpers, thin-film/MELiSSA research kernels, clean-room BioSim-style resource/tick primitives, BioSim-plus synthetic scenario-domain structural validation, bounded process/intent-planning helpers, bounded compartment replay/digest/event helpers, and B2c replay-integrity/ledger/report helpers. |
| `aero-codex-cli` | User-facing `aerocodex` Beta 1 concept binary for the ten governed M00 canonical-unit formulas, stable JSON output, exit codes, and bounded self-checks. |
| `xtask` | Dependency-free local governance, validation, data-registry, formula-vault, and inventory checks. |

## Quick start

```bash
git clone https://github.com/ConorMcGibboney/AeroCodex.git
cd AeroCodex
cargo test --workspace --all-features
python3 scripts/verify_governance.py --repo .
```

## Beta 1 concept CLI

The first testable release vertical slice exposes exactly ten governed M00 canonical-unit formulas without changing their `research_required` status or duplicating their mathematics:

```bash
cargo run -p aero-codex-cli -- version --json
cargo run -p aero-codex-cli -- formulas
cargo run -p aero-codex-cli -- run \
  formula_vault.m00.canonical.distance_to_canonical \
  distance=-42 distance_unit=7 --json
cargo run -p aero-codex-cli -- self-check --json
```

A clean self-check reports 14 passing checks and zero failures. See [`docs/beta1/release_concept.md`](docs/beta1/release_concept.md) and [`docs/beta1/cli_quickstart.md`](docs/beta1/cli_quickstart.md). The `beta1-concept` label is a software release-channel experiment; Cargo versions remain `0.0.1`, and no operational, parity, safety, or certification claim is made.

Minimal example:

```rust
use aero_codex_atmosphere::troposphere_state;
use aero_codex_aerodynamics::dynamic_pressure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let atmosphere = troposphere_state(1_000.0)?;
    let q = dynamic_pressure(atmosphere.density_kg_m3, 70.0)?;

    println!("T = {:.2} K", atmosphere.temperature_k);
    println!("rho = {:.4} kg/m^3", atmosphere.density_kg_m3);
    println!("q = {:.1} Pa", q);
    Ok(())
}
```

## Validation and governance artifacts

Key governance surfaces:

- `validation/cards/` — validation-planning cards.
- `validation/source_registry/` — conservative source-registry seed files.
- `validation/equation_inventory.tsv` — machine-readable equation inventory/readiness dashboard.
- `validation/schema/` — Codex Card schema.
- `data-governance/` — data/source policy and governed in-repo/external artifact registry.
- `formula-vault/` — quarantined formula-candidate metadata, contracts, manifests, and implementation gates.
- `nomenclature/` — acronym, symbol, terminology, and waiver policy.

Current cards and source-registry seeds intentionally remain conservative `research_required` artifacts unless exact source, test, tolerance, and validation evidence has been reviewed. A validation card or source-registry seed does not imply certification, flight readiness, mission readiness, operational approval, medical approval, or habitat-safety approval.

## Source boundaries

AeroCodex uses one canonical GitHub `main` and short-lived deployment branches. External bundles are source material, not automatic public API.

- **M07 astrodynamics bundle**: quarantined formula-vault candidate source. No bulk import, no astrodynamics crate overwrite, no public API promotion until per-slice contracts, tests, tolerances, and reference/equivalence gates pass.
- **BioSim Java and BioSim-RS bootstrap**: GPL-boundaried source/reference material. Do not mix GPL implementation code into the dual MIT/Apache AeroCodex core unless a future deliberate licensing path authorizes it.
- **Orekit**: reference oracle and architecture guide only. Do not clone the Java class hierarchy class-for-class.
- **Thin-film BLSS materials**: equation-traceable research kernels and data artifacts with cited-source boundaries; not calibrated habitat or medical designs.

## Recommended checks

Run these before merging user-visible changes:

```bash
git status --short
git diff --check
sha256sum -c checksums/SHA256SUMS
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo run -p aero-codex-cli -- version --json
cargo run -p aero-codex-cli -- run formula_vault.m00.canonical.distance_to_canonical distance=-42 distance_unit=7 --json
cargo run -p aero-codex-cli -- self-check --json
python scripts/verify_governance.py --repo .
cargo run -p xtask -- dependency-policy
python scripts/verify_thinfilm_artifact.py
python nomenclature/tooling/aerocodex_nom_lint.py --root nomenclature
python nomenclature/tooling/aerocodex_acronym_inventory.py --repo-root . --nomenclature-root nomenclature --check-new --baseline nomenclature/generated/current_repo_acronym_baseline.json
python nomenclature/tooling/aerocodex_terminology.py --root nomenclature export-jsonl --output nomenclature/generated/terminology/index.jsonl
git diff --exit-code nomenclature/generated/terminology/index.jsonl
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
```

The root `Cargo.lock` is intentionally not committed in the current workspace flow. Remove generated root lockfiles before staging unless the project deliberately changes that policy.

## Citation and reuse guidelines

When discussing or reusing an AeroCodex calculation:

- cite the original equation, dataset, standard, paper, report, or source material;
- cite the exact AeroCodex commit, crate, function, validation card, and source-registry entry;
- preserve the conservative validation status and safety caveats;
- for thin-film BLSS work, include the relevant files in `citations/`, `data/thinfilm/`, and `crates/aero-codex-life-support/src/thinfilm_provenance.rs`;
- when adding a new public calculation, add or update its source-registry entry, validation card, tests, evidence-card linkage, equation inventory row, and README-facing citation guidance.

## License

AeroCodex core repository code is licensed under `MIT OR Apache-2.0` unless a file states otherwise. External source materials retain their own licenses and source-boundary restrictions. GPL BioSim-related materials remain license-boundaried from the dual MIT/Apache core unless a future explicit licensing decision changes that.

A28-A30 complete the governed solver / numerical propagation policy backlog with 123 contract- or policy-blocked rows and leave 0 rows in that candidate pool. A31 starts the relative-motion and finite-burn scalar policy backlog with 40 contract- or policy-blocked rows and leaves 69 rows in that candidate pool.

A28-A30 complete the governed solver / numerical propagation policy backlog with 123 contract- or policy-blocked rows and leave 0 rows in that candidate pool. A31-A33 complete the relative-motion and finite-burn scalar policy backlog with 109 contract- or policy-blocked rows and leave 0 rows in that candidate pool.

### A34 external M07 attitude / inertia / quaternion policy Wave 1

A34 records 40 metadata-only terminal dispositions for the first bounded attitude representation, inertia, quaternion, and DCM policy slice. It raises the external M07 processed counter to 895 rows and leaves 428 rows in backlog. No runtime source, Scilab source, certification, or external parity claim is added.


### A35 external M07 attitude / inertia / quaternion policy Wave 2

A35 closes the remaining 19 metadata-only terminal dispositions for the attitude representation, inertia, quaternion, and DCM policy candidate pool. It raises the external M07 processed counter to 914 rows and leaves 409 rows in backlog. No runtime source, Scilab source, certification, or external parity claim is added.

### A36 external M07 attitude dynamics/control policy Wave 1

A36 records 38 metadata-only terminal dispositions for the attitude dynamics/control policy candidate pool. It raises the external M07 processed counter to 952 rows and leaves 371 rows in backlog. No runtime source, Scilab source, certification, or external parity claim is added.

### A37 external M07 J2 perturbation / numerical propagation policy Wave 1

A37 records 40 metadata-only terminal dispositions for the first bounded J2 perturbation / numerical-propagation policy slice. It raises the external M07 processed counter to 992 rows and leaves 331 rows in the external backlog. No runtime source, Scilab source, certification, or external parity claim is added.

### A38 external M07 J2 perturbation / numerical propagation policy Wave 2

A38 records 40 metadata-only terminal dispositions for the second bounded J2 perturbation / numerical-propagation policy slice. It raises the external M07 processed counter to 1032 rows and leaves 291 rows in the external backlog. No runtime source, Scilab source, certification, or external parity claim is added.
### A39 external M07 J2 perturbation / numerical propagation policy Wave 3

A39 closes the remaining 48 metadata-only terminal dispositions for the J2 perturbation / numerical-propagation policy candidate pool. It raises the external M07 processed counter to 1080 rows and leaves 243 rows in the external backlog. No runtime source, Scilab source, certification, or external parity claim is added.


### A40 external M07 SGP4 / TEME frame-time policy Wave 1

A40 records 45 metadata-only terminal dispositions for the SGP4/TEME frame-time policy and helper-exclusion candidate pool. It raises the external M07 processed counter to 1125 rows and leaves 198 rows in the external backlog. No runtime source, Scilab source, certification, or external parity claim is added.
### A41 external M07 CR3BP / external-data / input-output policy Wave 1

A41 records 45 metadata-only terminal dispositions for the CR3BP family/oracle policy, external data-table fixture governance, and input/output demonstration-row exclusion candidate pools. It raises the external M07 processed counter to 1170 rows and leaves 153 rows in the external backlog. No runtime source, Scilab source, external fixtures, certification, or external parity claim is added.
### A42 external M07 classifier-refresh / manual source-review policy Wave 1

A42 records 45 metadata-only terminal dispositions for the first bounded classifier-refresh and manual source-review policy slice. It raises the external M07 processed counter to 1215 rows, leaves 108 rows in the external backlog, and leaves 13 rows in this classifier-refresh/manual-review candidate pool. No runtime source, Scilab source, certification, or external parity claim is added.

### A43 external M07 scalar/unit helper policy Wave 1

A43 records 45 metadata-only terminal dispositions for remaining scalar-helper and unit/constants helper policy rows. It raises the external M07 processed counter to 1260 rows and leaves 63 rows in the external backlog. No runtime source, Scilab source, certification, or external parity claim is added.

### A44 external M07 residual scalar/unit/helper policy Wave 1

A44 records 45 metadata-only terminal dispositions for residual angle endpoint, helper exclusion, and unit/constants policy rows. It raises the external M07 processed counter to 1305 rows and leaves 18 rows in the external backlog. No runtime source, Scilab source, certification, or external parity claim is added.
