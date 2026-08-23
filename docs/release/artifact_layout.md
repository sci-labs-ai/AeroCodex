# v0.1.0-alpha.1 artifact names and layouts

`release/release-manifest.toml` is the machine-readable authority for names. The tag workflow must emit exactly these top-level files:

- `aerocodex-0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz`
- `aerocodex-0.1.0-alpha.1-x86_64-pc-windows-msvc.zip`
- `aerocodex-0.1.0-alpha.1-x86_64-apple-darwin.tar.gz`
- `aerocodex-0.1.0-alpha.1-aarch64-apple-darwin.tar.gz`
- `aerocodex-0.1.0-alpha.1-source.tar.gz`
- `aerocodex-0.1.0-alpha.1-release-manifest.toml`
- `aerocodex-0.1.0-alpha.1.spdx.json`
- `aerocodex-0.1.0-alpha.1.intoto.jsonl`
- `aerocodex-0.1.0-alpha.1-SHA256SUMS`

Each platform archive contains one directory named `aerocodex-0.1.0-alpha.1-<target>/`. That directory contains the `aerocodex` executable (`aerocodex.exe` on Windows), README, license and notice files, captured smoke-test JSON, and `release/release-manifest.toml` with its raw SHA-256 sidecar.

The release workflow builds with the committed lockfile and release profile, runs `version`, the exact executable formula list, one public formula calculation, and self-check from the staged binary, then archives it. The aggregate job creates the source archive, SPDX SBOM, in-toto provenance statement, checksum file, and GitHub attestations. Fresh Linux, Windows, macOS x86_64, and macOS arm64 jobs then download the aggregate artifact, verify every top-level checksum, extract the source and matching binary archives, and run the public 16-step friend-test package across both. A draft GitHub Release is created only after those downloaded-artifact checks and the independent SBOM/provenance verifier pass. Publication is a later R7 decision.
