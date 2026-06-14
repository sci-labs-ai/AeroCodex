# Microtask 7 — Atmosphere v0.001 Equations

Status: complete in this session by static review and source edit. Rust compile/test execution remains pending in a Rust-enabled environment.

## Scope

Microtask 7 implements and hardens the first Phase 0.001 atmosphere equations in `aero-codex-atmosphere`:

- `standard_sea_level()`
- `troposphere_temperature(altitude_m)`
- `troposphere_pressure(altitude_m)`
- `troposphere_density(altitude_m)`
- `troposphere_state(altitude_m)`
- `speed_of_sound(gamma, gas_constant, temperature)`
- `verification_record(codex_id)`

## Phase 0.001 model boundary

The implemented troposphere functions use a simplified constant-lapse-rate standard-atmosphere relation over:

```text
0 m <= altitude_m <= 11000 m
```

The altitude input is documented as geometric altitude used directly as the standard-atmosphere altitude variable. Geometric/geopotential altitude conversion is explicitly deferred. This is a Phase 0.001 simplification, not a high-fidelity atmosphere model.

## Source and validation status

The atmosphere crate now exposes conservative `VerificationRecord` metadata for its Codex IDs. Every atmosphere record remains:

```text
VerificationStatus::ResearchRequired
```

A validation-planning card was added:

```text
validation/cards/atmosphere_standard_troposphere.yaml
```

The card references:

```text
source.atmosphere.us_standard_atmosphere_1976.research_required
```

No source-registry entry or validation card was upgraded from `research_required`.

## Rust changes

Updated:

```text
crates/aero-codex-atmosphere/src/lib.rs
```

Key refinements:

- documented the simplified troposphere altitude convention;
- made troposphere domain constants public;
- added `troposphere_state(altitude_m)` as a convenience state helper;
- added `verification_record(codex_id)` for dependency-free trace metadata;
- preserved `AeroResult<T>` returns for invalid altitude and invalid speed-of-sound inputs;
- changed speed-of-sound temperature validation from nonnegative to strictly positive;
- expanded tests for sea-level matching, monotonic troposphere behavior, upper-bound acceptance, invalid inputs, state aggregation, and conservative verification metadata.

## Checks run in this environment

- Parsed all Cargo manifests with Python `tomllib`.
- Confirmed `aero-codex-atmosphere` depends only on `aero-codex-core` and `aero-codex-constants`.
- Confirmed required Microtask 7 public function names are present.
- Confirmed troposphere altitude range constants and domain error paths are present.
- Confirmed `speed_of_sound` rejects invalid gamma, gas constant, and temperature inputs through shared validation helpers.
- Confirmed atmosphere trace metadata remains `ResearchRequired`.
- Confirmed the new atmosphere validation card has required fields, nonempty list sections, valid category/status, and an existing source-registry ID.
- Confirmed all validation cards and source-registry entries remain `research_required`.
- Re-ran the static forbidden native dependency token scan across Cargo manifests.
- Performed rough brace/parenthesis balance checks on changed Rust source.

## Checks not run in this environment

The following remain mandatory for the deployment agent:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```

Reason not run here: `cargo`, `rustc`, `rustfmt`, and `clippy-driver` are unavailable in this environment.

## Definition-of-done result

The atmosphere crate now has the first Phase 0.001 atmosphere equations, documented validity limits, conservative trace metadata, a validation-planning card, and expanded tests. Source and validation status remain conservative pending source review.
