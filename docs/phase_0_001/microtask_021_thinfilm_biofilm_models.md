# Microtask 021 - Thin-film and biofilm BLSS Rust implementation

## Scope

This microtask converts the supplied `blss_thinfilm_report.pdf`, `blss_thinfilm_report.tex`, and `blss_thinfilm_refs.bib` into AeroCodex-compatible pure Rust kernel code inside `crates/aero-codex-life-support`.

The implementation adds four source modules:

- `brlss_backbone.rs` for the common compartment, stoichiometric, closure, and ESM equations.
- `melissa_photobioreactor.rs` for MELiSSA C4a Limnospira light transfer, reduced dynamics, carbonate chemistry, oxygen production, and simple control maps.
- `nitrifying_biofilm.rs` for MELiSSA C3 nitrification stoichiometry, radial diffusion, Fick flux, biofilm thickness, nitrifier kinetics, and tank/biofilm coupling.
- `thinfilm_algal_biofilm.rs` for attached algal thin-film growth, light attenuation, mixture/PDE local residuals, boundary conditions, ROM service vectors, and validated-domain checks.

`thinfilm_provenance.rs` preserves equation numbers, Codex IDs, function names, source-registry IDs, and BibTeX keys.

## Implementation policy

The current AeroCodex repository has a pure Rust policy. No external crates, native libraries, BLAS/LAPACK, Python, Matlab, Julia, C/C++/Fortran, generated binaries, or FFI wrappers were added. The cylindrical two-flux helper uses an internal pure-Rust modified-Bessel power-series evaluator to avoid dependency drift.

## Citation preservation

Citation information is preserved in five places:

1. Rustdoc comments on public functions.
2. `thinfilm_provenance::EQUATION_REFERENCES`.
3. `data/thinfilm/equation_manifest.csv`.
4. `citations/blss_thinfilm_refs.bib`.
5. `DATA_MANIFEST.toml` and validation/source-registry YAML.

## Verification performed in this artifact build

- Source material was copied into `source_material/new_thinfilm/`.
- All 51 report equations from the thin-film report and appendix service equations are mapped in `data/thinfilm/equation_manifest.csv`.
- Source metadata from the supplied BibTeX was cross-checked against publisher, university, or indexed records and summarized in `data/thinfilm/source_verification.csv`.
- AeroCodex validation cards and source-registry YAML files were added for the new implementation.
- A static artifact verification script is included at `scripts/verify_thinfilm_artifact.py`.

## Important limitation

This is an equation-traceable Rust implementation, not a calibrated life-support design and not a mission/flight/habitat-safety validation. Biological parameters, membrane or biofilm transport coefficients, optical coefficients, and operating domains must be calibrated against the target organism, reactor, lighting spectrum, gravity/wetting regime, and mission architecture.
