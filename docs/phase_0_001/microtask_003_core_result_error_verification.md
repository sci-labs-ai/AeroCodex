# Microtask 3 — Core Result, Error, and Verification Types

Status: complete in this interactive session.

## Goal

Microtask 3 establishes the shared language that AeroCodex equation crates use when calculations can fail, carry assumptions, emit model warnings, or expose verification maturity.

## Public core vocabulary reviewed/refined

| Type or alias | Location | Purpose |
|---|---|---|
| `AeroResult<T>` | `crates/aero-codex-core/src/lib.rs` | Standard result alias: `Result<T, AeroError>`. |
| `AeroError` | `crates/aero-codex-core/src/error.rs` | Specific checked-calculation error variants. |
| `EngineeringResult<T>` | `crates/aero-codex-core/src/result.rs` | Value plus assumptions, warnings, validity status, and verification record. |
| `Assumption` | `crates/aero-codex-core/src/result.rs` | Named assumption attached to a result. |
| `ModelWarning` | `crates/aero-codex-core/src/result.rs` | Non-fatal warning attached to a result. |
| `ValidityStatus` | `crates/aero-codex-core/src/result.rs` | Domain/validity assessment for a calculation result. |
| `VerificationStatus` | `crates/aero-codex-core/src/result.rs` | Verification maturity ladder. |
| `VerificationRecord` | `crates/aero-codex-core/src/result.rs` | Codex ID, status, source IDs, and notes. |
| `ParseStatusError` | `crates/aero-codex-core/src/result.rs` | Parse error for string-to-status conversion. |

## Error variants

The required Microtask 3 variants are present:

- `NonPositiveInput`
- `OutOfDomain`
- `RequiresSupersonic`
- `AmbiguousBranch`
- `NumericalFailure`
- `UnverifiedSource`

The existing `NegativeInput` variant is retained for nonnegative-domain checks such as pressure, density, mass, area, time, and other quantities where zero is allowed.

## Verification statuses

The required verification-status ladder is present and represented as canonical snake-case text through `Display`, `as_str()`, and `FromStr`:

- `ResearchRequired` → `research_required`
- `EquationTraceable` → `equation_traceable`
- `ImplementationVerified` → `implementation_verified`
- `ReferenceValidated` → `reference_validated`
- `ExperimentValidated` → `experiment_validated`

No serialization dependency such as `serde` was added. Text conversion is intentionally dependency-free for Phase 0.001.

## Validity-status behavior

`EngineeringResult<T>::new` now defaults to `ValidityStatus::NotAssessed` rather than implying that a result is inside the documented domain. Callers should explicitly set `ValidityStatus::WithinDocumentedDomain`, `BoundaryCase`, or `OutsideDocumentedDomain` after input/domain validation.

The current life-support `closure_fraction` helper was updated to explicitly mark its result as `WithinDocumentedDomain` after validating nonnegative recycled mass rate and positive required mass rate.

## API refinements made

- Added stable snake-case `AeroError::code()` values.
- Added `AeroError::parameter()` for single-parameter input/domain diagnostics.
- Added `AeroError::is_domain_error()`.
- Added `VerificationStatus::as_str()` and `ValidityStatus::as_str()`.
- Added `FromStr` implementations for `VerificationStatus` and `ValidityStatus`.
- Added `VerificationStatus::has_external_validation_evidence()`.
- Added `ValidityStatus::requires_attention()`.
- Added constructors for `Assumption`, `ModelWarning`, and `VerificationRecord::research_required`.
- Added `EngineeringResult::from_verification`, `with_assumption_record`, `with_warning_record`, `has_warnings`, and `verification_status`.

## Tests added/refined

The core crate test module now covers:

- construction of every current `AeroError` variant;
- required error display behavior and code/parameter helpers;
- construction of `EngineeringResult<T>` with assumptions, warnings, validity, and verification metadata;
- conservative default validity status of `NotAssessed`;
- display/as-str/parse round trips for all verification statuses;
- display/as-str/parse round trips for all validity statuses.

## Source-verification impact

This microtask changed shared metadata and error vocabulary only. No source-registry entry or validation card status was upgraded. All source claims remain at their prior conservative status.

## Required deployment-agent follow-up

A Rust-equipped deployment environment must still run:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```
