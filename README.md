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

This README reflects the governed state after **Stage 5 professional hardening — local-gate parity and dynamics-test coverage** on current `main`. This is test/governance hardening only; no Stage 5 source-material handoff or Chunk 0 deployment is implied.

| Inventory class | Count | Meaning |
|---|---:|---|
| Executable research equations | 138 | Public Rust research/preliminary-design equation kernels inventoried by `validation/equation_inventory.tsv`. |
| Metadata-only formula-vault candidates | 27 | M00 angle/unit and vector-algebra candidates selected into the formula-vault metadata layer. |
| External M07 backlog rows | 1,323 | Registered external M07 represented rows not yet selected as formula-vault candidates. |
| Validation cards | 43 | Conservative validation/governance records. They are not certification evidence. |
| Source-registry seeds | 41 | Source/governance traceability seeds. |
| Validation-card-only records | 43 | Metadata records, not formula implementations. |
| Helper algorithms | 138 | Support routines not counted as executable research equations. |

Stage 4 Chunk 8A added a bounded M00 vector/angle expansion to `aero-codex-astrodynamics`: `m00_degrees_to_radians`, `m00_radians_to_degrees`, `m00_vector_dot`, `m00_vector_norm`, `m00_vector_cross`, `m00_unit_vector`, `m00_vector_angle`, `m00_vector_projection`, `m00_scalar_triple_product`, `m00_vector_triple_product`, `m00_vector_triple_bac_cab`, `m00_vectors_collinear`, `m00_vectors_coplanar`, `m00_tangent_from_dr_ds`, `m00_velocity_from_arc_rate`, and `m00_vector_distance`.

`wrap2pi` remains blocked for a dedicated endpoint-behavior chunk. `app_resolve_coplanar` remains blocked for a separate least-squares/rank/tolerance policy. The M07 source artifact remains quarantined source material: it reports 1,350 represented function rows and 188 Scilab equivalence jobs, but it is still release-candidate / not certified and is not bulk-merged into public APIs.

## What AeroCodex can do now

AeroCodex currently provides source-traceable research kernels for common engineering calculations across atmosphere, thermodynamics, gas dynamics, aerodynamics, propulsion, heat transfer, structures, flight dynamics, astrodynamics, and bio-regenerative life-support scaffolding.

It also provides Stage 4 governance machinery:

- validation cards and source-registry seeds;
- a data/source registry for external materials and in-repo artifact hashes;
- a formula-vault quarantine path for M07-derived candidates;
- an equation inventory/readiness dashboard;
- a nomenclature/acronym policy and generated terminology index;
- clean-room BioSim-RS-style resource identity, transaction, deterministic replay, ledger, and smoke/friend-test primitives.

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
| `aero-codex-astrodynamics` | Two-body orbital helpers, Hohmann transfer helpers, sphere of influence, and staged formula-vault candidates. |
| `aero-codex-life-support` | BLSS mass-balance helpers, thin-film/MELiSSA research kernels, and clean-room BioSim-style resource/tick primitives. |
| `xtask` | Dependency-free local governance, validation, data-registry, formula-vault, and inventory checks. |

## Quick start

```bash
git clone https://github.com/ConorMcGibboney/AeroCodex.git
cd AeroCodex
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
```

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

## Stage 4 source boundaries

Stage 4 uses one canonical GitHub `main` and short-lived deployment branches. External bundles are source material, not automatic public API.

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
cargo run -p xtask -- verify --all
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
