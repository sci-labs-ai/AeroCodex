# AeroCodex

<!-- aerocodex-current-identity:start -->
Release version: `0.1.0-alpha.1`
Release tier: `research_software_alpha` (`Research Software Alpha`)
Workspace packages: `14`
Registry formulas: `152`
Blocked formulas: `140`
Publicly executable formulas: `12`
<!-- aerocodex-current-identity:end -->

<p align="center">
  <img src="assets/aerocodex_patch.png" alt="AeroCodex mission patch" width="420">
</p>

Source-traceable aerospace research software in pure Rust.

AeroCodex `0.1.0-alpha.1` provides a bounded command-line release of twelve audited M00 canonical-unit and angle conversions. The other 140 Formula Registry records remain blocked. This is Research Software Alpha for research, education, verification-oriented development, and preliminary design.

## Install

### Release archive

After the final release is published, download the archive for your platform from [GitHub Releases](https://github.com/sci-labs-ai/AeroCodex/releases), verify it against `aerocodex-0.1.0-alpha.1-SHA256SUMS`, extract it, and place `aerocodex` (`aerocodex.exe` on Windows) on your command path.

The release workflow produces Linux x86-64, Windows x86-64, macOS x86-64, and macOS ARM64 archives. Exact names and layouts are governed in [artifact layout](docs/release/artifact_layout.md).

### Build from source

Install the pinned Rust toolchain, then build or install with the committed dependency graph:

```bash
git clone https://github.com/sci-labs-ai/AeroCodex.git
cd AeroCodex
rustup show
cargo build --locked --release -p aero-codex-cli
cargo install --locked --path crates/aero-codex-cli
```

Release builds use Rust `1.98.0`; blocking CI separately checks the declared MSRV, Rust `1.74.0`. See [release build policy](docs/development/release_builds.md).

## First successful formula run

```bash
aerocodex version --json
aerocodex formula list --executable --json
aerocodex formula run m00.angle.deg_to_rad --degrees 180 --json
aerocodex self-check --json
```

The formula result is π radians. A clean self-check reports 14 passed and 0 failed after using the same public resolver, input parser, status gate, evaluator, and JSON envelope as `formula run`.

For the complete trace from ID and input through output, source record, validation card, promotion packet, tolerance, and caveat, see the [degrees-to-radians worked example](docs/examples/m00-angle-conversion.md).

## Supported alpha scope

Exactly these formula families are executable:

- ten M00 canonical-unit conversions covering canonical time, speed, gravitational parameter, distance, time, and speed scaling;
- two M00 angle conversions covering degrees to radians and radians to degrees.

Use `aerocodex formula list --executable --json` as the runtime source of truth. `release/release-manifest.toml` is the machine-readable release authority. Runtime symbols, inventory rows, or source records do not independently authorize execution.

## Status meanings

| Status | Meaning in this alpha | Execution |
| --- | --- | --- |
| `implementation_verified` | The bounded implementation and linked software evidence were reviewed. | Allowed only when the independent dispatch and release-policy gates also pass. |
| `research_required` | Additional evidence or review is still required. | Blocked. |
| `reference_validated` / `experiment_validated` | Higher evidence states defined for future governed work. | Not assigned to the twelve alpha formulas. |

`implemented`, `dispatchable`, `executable`, `validated`, and `blocked` are separate fields in `formula describe --json`. `implementation_verified` is not a claim of scientific reference validation.

## Safety boundary

AeroCodex is intended to become professional-grade, traceable aerospace research software suitable for academic, laboratory, and agency evaluation. It is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

Safety-critical, regulated, operational, crewed, habitat, medical, or mission use requires independent project-specific assurance, validation, qualification, and certification.

The twelve-formula alpha does not claim M07/Scilab parity, broader physical validation, a complete BioSim scenario engine, an operational life-support controller, or validation of the other 140 registry records. The enforced wording policy is [public wording guardrails](docs/assurance/public_wording_guardrails.md).

## Commands

```bash
aerocodex --version
aerocodex version --json
aerocodex formula list --family m00 --json
aerocodex formula list --executable --json
aerocodex formula describe m00.canonical.distance_to_canonical --json
aerocodex formula run m00.canonical.distance_to_canonical --distance 12000 --distance-unit 1000 --json
aerocodex formula status-report --json
aerocodex self-check --json
```

The JSON contract version is `aerocodex.cli.json.v1`. Success and error envelopes have golden key-set tests. See the [CLI quickstart](docs/research_alpha/cli_formula_quickstart.md) and [JSON contract](docs/research_alpha/json_contract.md).

## Evidence and release integrity

The repository governs:

- a 152-row Formula Registry and exact 12 / 140 release partition;
- formula contracts, validation cards, source records, promotion packets, and analytical vectors;
- deterministic generated registry, Rust registry, status report, and validation summary;
- cross-platform checksums with exact repository-file coverage;
- stable and MSRV CI, release-slice tests, package checks, archive smoke tests, SPDX SBOM, in-toto provenance, and GitHub attestations;
- embedded version, commit, build target, profile, and raw release-manifest SHA-256 in `aerocodex version --json`.

The live engineering record is [v0.1.0-alpha.1 release status](docs/release/v0.1.0-alpha.1-status.md). Passing software gates does not create a certification claim.

## Development

Use the committed lockfile and the same governed commands as CI:

```bash
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo run --locked -p xtask -- verify-checksums
cargo run --locked -p xtask -- verify-release-manifest
cargo run --locked -p xtask -- verify-release-identity
cargo run --locked -p xtask -- verify-generated
cargo run --locked -p xtask -- verify --all
cargo run --locked -p xtask -- dependency-policy
RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps
```

See [CONTRIBUTING](CONTRIBUTING.md) for code and formula-promotion paths, [SECURITY](SECURITY.md) for private vulnerability reporting, and the [community code of conduct](CODE_OF_CONDUCT.md).

## Documentation and citation

- [User and API documentation](https://sci-labs-ai.github.io/AeroCodex/)
- [Documentation index](docs/index.md)
- [Citation guide](docs/citation.md) and [CITATION.cff](CITATION.cff)
- [Historical documentation archive index](docs/archive/README.md)
- [Changelog](CHANGELOG.md)

When publishing a result, cite both the exact AeroCodex release or commit and the original scientific source linked by each formula record.

## License

Repository code is licensed under `MIT OR Apache-2.0` unless a file states otherwise. External source materials retain their own licenses and source-boundary restrictions. GPL BioSim-related material remains license-boundaried from the dual MIT/Apache core unless a future explicit licensing decision changes that boundary.
