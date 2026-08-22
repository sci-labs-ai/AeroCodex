# Release build and package policy

Release builds use `rust-toolchain.toml` (`1.98.0`), the committed version-3 `Cargo.lock`, and `cargo build --locked --release`. A separate blocking Linux job uses the declared minimum supported Rust version, `1.74.0`, for locked workspace check and test commands.

The root release profile uses optimization level 3, thin LTO, one code-generation unit, disabled incremental compilation, and stripped symbols. These choices favor small, repeatable distribution binaries; they are not numeric-validation settings and do not change floating-point semantics promised by a formula contract.

The alpha Cargo package boundary contains `aero-codex-core`, `aero-codex-constants`, `aero-codex-astrodynamics`, and `aero-codex-cli`. CI uses Cargo's stabilized multi-package mode to create all four interdependent archives together with `cargo package --locked --no-verify -p <each-approved-package>`; registry publication remains a separate maintainer action because the alpha packages are not yet present in a public registry. Every other workspace package, including `xtask`, is `publish = false`.

The primary public distribution is the GitHub Release artifact set documented in `docs/release/artifact_layout.md`.
