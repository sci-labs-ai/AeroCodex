# Toolchain baseline

AeroCodex is research/preliminary-design software. It is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

## Rust and Cargo

- Release and normal blocking CI use the exact Rust `1.98.0` toolchain declared by `rust-toolchain.toml`; nightly Rust is not required.
- A separate blocking Ubuntu job checks and tests the locked workspace with the declared MSRV, Rust `1.74.0`.
- The repository is cargo-first: use Cargo commands from the workspace root for formatting, linting, tests, documentation, and local governance checks.
- Optional `just` usage, if added later, is optional only. A contributor must be able to run the documented Cargo commands without installing `just`.
- The root `Cargo.lock` is committed, governed by repository checksums, uses lockfile format 3 for MSRV compatibility, and is required through `--locked` in release and friend-test commands.

## Platform posture

- Ubuntu and Windows are Tier 1 release platforms. macOS x86-64 and ARM64 are Tier 2 release platforms.
- Blocking pull-request CI runs on Ubuntu, Windows, and macOS; the separate declared-MSRV job runs on Ubuntu.

## Dependency posture

The alpha dependency graph contains only the fourteen workspace packages and no third-party Cargo package. The dependency policy rejects unreviewed native/runtime integration tokens, requires the dual MIT/Apache workspace license, and fails if `Cargo.lock` gains an unreviewed package. The alpha does not require:

- Python, Jupyter, web services, or API servers;
- nightly Rust;
- external native math libraries such as BLAS, LAPACK, CEA, REFPROP, CoolProp, or Cantera;
- compiled native binary blobs or generated binaries.

Existing source-intake, planning, or evidence materials may mention outside tools as references, but they are not required to build, test, document, or verify the Rust workspace in this phase.

## Generated registry posture

Generated registry artifacts are checked in only when their generation is deterministic and governed by documented inputs, stable ordering, reviewable hashes, and blocking freshness gates.
