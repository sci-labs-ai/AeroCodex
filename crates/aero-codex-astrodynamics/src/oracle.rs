//! AeroCodex-native records for bounded astrodynamics oracle comparisons.
//!
//! This module stores comparison context and performs deterministic scalar or
//! three-component vector tolerance checks. It does not run an external oracle,
//! read fixture files, calculate evidence hashes, perform frame transforms,
//! convert time scales, or attach accuracy/certification meaning to a result.
//!
//! The primary public comparison functions require a validated case record, so
//! units, frame, declared time scale, epoch, source label, and input/output
//! summaries cannot be skipped. Low-level component comparison remains private.

use crate::{
    frames::AstroFrame,
    time::{AstroEpoch, AstroTimeScale},
};
use core::fmt;

const SHA256_COUNT: usize = 64;
const SCALAR_COMPONENT_COUNT: usize = 1;
const VECTOR_COMPONENT_COUNT: usize = 3;

/// Absolute-plus-relative tolerance used by one oracle comparison.
///
/// The absolute term is expressed in the case's declared output units. The
/// relative term is dimensionless. The allowed error for each component is
/// `absolute + relative * abs(expected)`. Both terms may be zero, which requests
/// an exact component comparison. Overflow in the relative product or final sum
/// is rejected rather than treated as an infinite tolerance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AstrodynamicsTolerance {
    absolute: f64,
    relative: f64,
}

impl AstrodynamicsTolerance {
    /// Creates a finite nonnegative absolute/relative tolerance pair.
    pub fn new(absolute: f64, relative: f64) -> Result<Self, OracleComparisonError> {
        validate_tolerance_values(absolute, relative)?;
        Ok(Self { absolute, relative })
    }

    /// Returns `(absolute_in_output_units, relative_dimensionless)`.
    #[must_use]
    pub const fn values(self) -> (f64, f64) {
        (self.absolute, self.relative)
    }
}

/// Potentially incomplete input used to construct a validated oracle case.
///
/// `Option` is deliberate for frame/time/epoch fields so imported metadata can
/// fail with an explicit missing-context error instead of receiving a default.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AstrodynamicsOracleCaseDraft {
    /// Stable AeroCodex-native identifier. Allowed characters are ASCII letters,
    /// digits, `.`, `_`, `-`, and `:`.
    pub case_id: String,
    /// Human-readable source/oracle label; this is metadata, not executable linkage.
    pub source_oracle_label: String,
    /// Frame label attached to every compared quantity.
    pub frame: Option<AstroFrame>,
    /// Explicit declared time-scale label.
    pub time_scale: Option<AstroTimeScale>,
    /// Epoch carrying its own time-scale label.
    pub epoch: Option<AstroEpoch>,
    /// One-line definition/assumption for the selected frame label.
    pub frame_context: String,
    /// One-line identification of the caller-defined epoch reference origin.
    pub epoch_reference: String,
    /// Input-unit contract, for example `m; m/s; s`.
    pub input_units: String,
    /// Output-unit contract used by the absolute tolerance.
    pub output_units: String,
    /// Deterministic summary of the compared inputs and assumptions.
    pub input_summary: String,
    /// Deterministic summary of the expected output quantity.
    pub expected_output_summary: String,
}

/// Read-only view of validated oracle-case metadata.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AstrodynamicsOracleCaseView<'a> {
    pub case_id: &'a str,
    pub source_oracle_label: &'a str,
    pub frame: AstroFrame,
    pub time_scale: AstroTimeScale,
    pub epoch: AstroEpoch,
    pub frame_context: &'a str,
    pub epoch_reference: &'a str,
    pub input_units: &'a str,
    pub output_units: &'a str,
    pub input_summary: &'a str,
    pub expected_output_summary: &'a str,
}

/// Validated metadata required before any numerical oracle comparison.
///
/// Frame and time-scale values are labels only. This type performs no frame
/// transform and no time-scale conversion. A label-only frame or time scale may
/// be recorded as context; that does not make a transform/conversion available.
#[derive(Debug, Clone, PartialEq)]
pub struct AstrodynamicsOracleCase {
    case_id: String,
    source_oracle_label: String,
    frame: AstroFrame,
    time_scale: AstroTimeScale,
    epoch: AstroEpoch,
    frame_context: String,
    epoch_reference: String,
    input_units: String,
    output_units: String,
    input_summary: String,
    expected_output_summary: String,
}

impl AstrodynamicsOracleCase {
    /// Validates and converts a potentially incomplete draft into an immutable case.
    pub fn try_from_draft(
        draft: AstrodynamicsOracleCaseDraft,
    ) -> Result<Self, OracleComparisonError> {
        validate_case_id(&draft.case_id)?;
        validate_required_text(
            "source_oracle_label",
            &draft.source_oracle_label,
            OracleComparisonError::MissingSourceOracleLabel,
        )?;
        validate_required_text(
            "frame_context",
            &draft.frame_context,
            OracleComparisonError::MissingFrameContext,
        )?;
        validate_required_text(
            "epoch_reference",
            &draft.epoch_reference,
            OracleComparisonError::MissingEpochReference,
        )?;
        validate_required_text(
            "input_units",
            &draft.input_units,
            OracleComparisonError::MissingInputUnits,
        )?;
        validate_required_text(
            "output_units",
            &draft.output_units,
            OracleComparisonError::MissingOutputUnits,
        )?;
        validate_required_text(
            "input_summary",
            &draft.input_summary,
            OracleComparisonError::MissingInputSummary,
        )?;
        validate_required_text(
            "expected_output_summary",
            &draft.expected_output_summary,
            OracleComparisonError::MissingExpectedOutputSummary,
        )?;

        let frame = draft.frame.ok_or(OracleComparisonError::MissingFrame)?;
        let time_scale = draft
            .time_scale
            .ok_or(OracleComparisonError::MissingTimeScale)?;
        let epoch = draft.epoch.ok_or(OracleComparisonError::MissingEpoch)?;
        if epoch.time_scale() != time_scale {
            return Err(OracleComparisonError::TimeScaleMismatch {
                declared: time_scale,
                epoch: epoch.time_scale(),
            });
        }

        let case = Self {
            case_id: draft.case_id,
            source_oracle_label: draft.source_oracle_label,
            frame,
            time_scale,
            epoch,
            frame_context: draft.frame_context,
            epoch_reference: draft.epoch_reference,
            input_units: draft.input_units,
            output_units: draft.output_units,
            input_summary: draft.input_summary,
            expected_output_summary: draft.expected_output_summary,
        };
        validate_astrodynamics_oracle_case(&case)?;
        Ok(case)
    }

    /// Returns a read-only view of all validated case fields.
    #[must_use]
    pub fn view(&self) -> AstrodynamicsOracleCaseView<'_> {
        AstrodynamicsOracleCaseView {
            case_id: &self.case_id,
            source_oracle_label: &self.source_oracle_label,
            frame: self.frame,
            time_scale: self.time_scale,
            epoch: self.epoch,
            frame_context: &self.frame_context,
            epoch_reference: &self.epoch_reference,
            input_units: &self.input_units,
            output_units: &self.output_units,
            input_summary: &self.input_summary,
            expected_output_summary: &self.expected_output_summary,
        }
    }
}

impl fmt::Display for AstrodynamicsOracleCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "case_id={:?} source={:?} frame={} frame_context={:?} time_scale={} epoch_seconds={:.17e} epoch_reference={:?} input_units={:?} output_units={:?} input_summary={:?} expected_output_summary={:?}",
            self.case_id,
            self.source_oracle_label,
            self.frame.label(),
            self.frame_context,
            self.time_scale.label(),
            self.epoch.seconds_from_reference_epoch(),
            self.epoch_reference,
            self.input_units,
            self.output_units,
            self.input_summary,
            self.expected_output_summary,
        )
    }
}

/// Evidence-hash policy attached to a comparison record.
///
/// This type never calculates a digest. `SyntheticNoExternalFixture` is only for
/// caller-created synthetic values. `Pending` blocks pass/fail status.
/// `Sha256LowerHex` accepts exactly 64 lowercase hexadecimal characters and
/// rejects the all-zero placeholder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleEvidenceHash {
    kind: OracleEvidenceHashKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OracleEvidenceHashKind {
    SyntheticNoExternalFixture,
    Pending,
    Sha256LowerHex(String),
}

impl OracleEvidenceHash {
    /// Marks a comparison as using only caller-created synthetic values.
    #[must_use]
    pub const fn synthetic_no_external_fixture() -> Self {
        Self {
            kind: OracleEvidenceHashKind::SyntheticNoExternalFixture,
        }
    }

    /// Marks the external/generated evidence digest as pending.
    #[must_use]
    pub const fn pending() -> Self {
        Self {
            kind: OracleEvidenceHashKind::Pending,
        }
    }

    /// Records an already-computed SHA256 digest in canonical lowercase hex.
    ///
    /// No file is read and no digest is calculated by this function. A
    /// syntactically valid caller-supplied digest is metadata only and is not
    /// treated as verified evidence or external-oracle authority.
    pub fn sha256_lower_hex(value: String) -> Result<Self, OracleComparisonError> {
        validate_sha256_lower_hex(&value)?;
        Ok(Self {
            kind: OracleEvidenceHashKind::Sha256LowerHex(value),
        })
    }

    /// Stable evidence-state code for reports and validation records.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match &self.kind {
            OracleEvidenceHashKind::SyntheticNoExternalFixture => "synthetic_no_external_fixture",
            OracleEvidenceHashKind::Pending => "pending",
            OracleEvidenceHashKind::Sha256LowerHex(_) => "sha256_lower_hex",
        }
    }

    /// Returns the attached SHA256 lowercase-hex value when one is present.
    ///
    /// `None` means either synthetic source-free comparison data or a pending
    /// digest; callers must inspect [`Self::code`] to distinguish those states.
    #[must_use]
    pub fn sha256_value(&self) -> Option<&str> {
        match &self.kind {
            OracleEvidenceHashKind::Sha256LowerHex(value) => Some(value.as_str()),
            OracleEvidenceHashKind::SyntheticNoExternalFixture
            | OracleEvidenceHashKind::Pending => None,
        }
    }

    fn is_pending(&self) -> bool {
        matches!(&self.kind, OracleEvidenceHashKind::Pending)
    }

    fn validate(&self) -> Result<(), OracleComparisonError> {
        match &self.kind {
            OracleEvidenceHashKind::SyntheticNoExternalFixture
            | OracleEvidenceHashKind::Pending => Ok(()),
            OracleEvidenceHashKind::Sha256LowerHex(value) => validate_sha256_lower_hex(value),
        }
    }
}

/// Shape of the numerical quantity compared in one record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleComparisonShape {
    Scalar,
    Vector3,
}

impl OracleComparisonShape {
    /// Stable list used by complete enum-coverage tests.
    pub const ALLOWED: [Self; 2] = [Self::Scalar, Self::Vector3];

    /// Stable snake-case code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Vector3 => "vector3",
        }
    }

    const fn component_count(self) -> usize {
        match self {
            Self::Scalar => SCALAR_COMPONENT_COUNT,
            Self::Vector3 => VECTOR_COMPONENT_COUNT,
        }
    }
}

/// Deterministic status of a record-validated tolerance comparison.
///
/// `PassedWithinTolerance` and `FailedOutsideTolerance` mean only that local
/// deterministic arithmetic was performed against caller-provided expected
/// values and caller-selected tolerances. They do not imply external validation,
/// source verification, physical correctness, parity, certification, or
/// operational readiness. `BlockedPendingEvidenceHash` means the numerical inputs
/// were checked but evidence remains pending, so no pass/fail metrics are stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleComparisonStatus {
    /// Local deterministic arithmetic was within caller-selected tolerance.
    PassedWithinTolerance,
    /// Local deterministic arithmetic was outside caller-selected tolerance.
    FailedOutsideTolerance,
    /// Evidence hash remains pending; no pass/fail metrics are stored.
    BlockedPendingEvidenceHash,
}

impl OracleComparisonStatus {
    /// Stable list used by complete enum-coverage tests.
    pub const ALLOWED: [Self; 3] = [
        Self::PassedWithinTolerance,
        Self::FailedOutsideTolerance,
        Self::BlockedPendingEvidenceHash,
    ];

    /// Stable snake-case code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::PassedWithinTolerance => "passed_within_tolerance",
            Self::FailedOutsideTolerance => "failed_outside_tolerance",
            Self::BlockedPendingEvidenceHash => "blocked_pending_evidence_hash",
        }
    }
}

/// Numerical summary for a completed scalar or three-component comparison.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OracleComparisonMetrics {
    pub component_count: usize,
    pub first_failing_component: Option<usize>,
    pub largest_absolute_error_component: usize,
    pub largest_absolute_error: f64,
    pub allowed_error_at_largest_component: f64,
}

/// Read-only view of an oracle comparison record.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AstrodynamicsOracleRecordView<'a> {
    pub case: AstrodynamicsOracleCaseView<'a>,
    pub tolerance: AstrodynamicsTolerance,
    pub evidence_hash: &'a OracleEvidenceHash,
    pub shape: OracleComparisonShape,
    pub status: OracleComparisonStatus,
    pub metrics: Option<OracleComparisonMetrics>,
}

/// Immutable result record returned by the high-level comparison functions.
#[derive(Debug, Clone, PartialEq)]
pub struct AstrodynamicsOracleRecord {
    case: AstrodynamicsOracleCase,
    tolerance: AstrodynamicsTolerance,
    evidence_hash: OracleEvidenceHash,
    shape: OracleComparisonShape,
    status: OracleComparisonStatus,
    metrics: Option<OracleComparisonMetrics>,
}

impl AstrodynamicsOracleRecord {
    /// Returns a read-only view of all record fields.
    #[must_use]
    pub fn view(&self) -> AstrodynamicsOracleRecordView<'_> {
        AstrodynamicsOracleRecordView {
            case: self.case.view(),
            tolerance: self.tolerance,
            evidence_hash: &self.evidence_hash,
            shape: self.shape,
            status: self.status,
            metrics: self.metrics,
        }
    }
}

impl fmt::Display for AstrodynamicsOracleRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (absolute, relative) = self.tolerance.values();
        write!(
            formatter,
            "oracle_record {} tolerance_absolute={absolute:.17e} tolerance_relative={relative:.17e} evidence={} evidence_sha256={:?} shape={} status={} scope=research_required",
            self.case,
            self.evidence_hash.code(),
            self.evidence_hash.sha256_value(),
            self.shape.code(),
            self.status.code(),
        )?;

        match self.metrics {
            Some(metrics) => write!(
                formatter,
                " component_count={} first_failing_component={} largest_absolute_error_component={} largest_absolute_error={:.17e} allowed_error_at_largest_component={:.17e}",
                metrics.component_count,
                format_optional_index(metrics.first_failing_component),
                metrics.largest_absolute_error_component,
                metrics.largest_absolute_error,
                metrics.allowed_error_at_largest_component,
            ),
            None => formatter.write_str(" metrics=none"),
        }
    }
}

/// Errors from case construction, evidence-hash validation, and comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleComparisonError {
    MissingCaseId,
    InvalidCaseId,
    MissingSourceOracleLabel,
    MissingFrame,
    MissingTimeScale,
    MissingEpoch,
    MissingFrameContext,
    MissingEpochReference,
    MissingInputUnits,
    MissingOutputUnits,
    MissingInputSummary,
    MissingExpectedOutputSummary,
    InvalidText {
        field: &'static str,
    },
    TimeScaleMismatch {
        declared: AstroTimeScale,
        epoch: AstroTimeScale,
    },
    InvalidTolerance {
        component: &'static str,
    },
    NonfiniteComparisonValue {
        series: &'static str,
        component_index: usize,
    },
    AllowedToleranceOverflow {
        component_index: usize,
    },
    AbsoluteDifferenceOverflow {
        component_index: usize,
    },
    InvalidEvidenceHash {
        reason: &'static str,
    },
    InconsistentRecord {
        reason: &'static str,
    },
}

impl fmt::Display for OracleComparisonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCaseId => formatter.write_str("oracle case ID is required"),
            Self::InvalidCaseId => formatter.write_str(
                "oracle case ID must use only ASCII letters, digits, '.', '_', '-', or ':'",
            ),
            Self::MissingSourceOracleLabel => {
                formatter.write_str("source/oracle label is required")
            }
            Self::MissingFrame => formatter.write_str("frame label is required"),
            Self::MissingTimeScale => formatter.write_str("declared time-scale label is required"),
            Self::MissingEpoch => formatter.write_str("epoch is required"),
            Self::MissingFrameContext => {
                formatter.write_str("frame-context assumption is required")
            }
            Self::MissingEpochReference => {
                formatter.write_str("epoch-reference identification is required")
            }
            Self::MissingInputUnits => formatter.write_str("input-unit contract is required"),
            Self::MissingOutputUnits => formatter.write_str("output-unit contract is required"),
            Self::MissingInputSummary => formatter.write_str("input summary is required"),
            Self::MissingExpectedOutputSummary => {
                formatter.write_str("expected-output summary is required")
            }
            Self::InvalidText { field } => write!(
                formatter,
                "oracle field `{field}` must have no leading/trailing whitespace or control characters"
            ),
            Self::TimeScaleMismatch { declared, epoch } => write!(
                formatter,
                "declared time scale {} does not match epoch time scale {}",
                declared.label(),
                epoch.label(),
            ),
            Self::InvalidTolerance { component } => write!(
                formatter,
                "tolerance component `{component}` must be finite and nonnegative"
            ),
            Self::NonfiniteComparisonValue {
                series,
                component_index,
            } => write!(
                formatter,
                "{series} comparison value at component {component_index} must be finite"
            ),
            Self::AllowedToleranceOverflow { component_index } => write!(
                formatter,
                "allowed tolerance overflowed at component {component_index}"
            ),
            Self::AbsoluteDifferenceOverflow { component_index } => write!(
                formatter,
                "absolute comparison difference overflowed at component {component_index}"
            ),
            Self::InvalidEvidenceHash { reason } => {
                write!(formatter, "invalid evidence hash: {reason}")
            }
            Self::InconsistentRecord { reason } => {
                write!(formatter, "inconsistent oracle record: {reason}")
            }
        }
    }
}

impl std::error::Error for OracleComparisonError {}

/// Revalidates all metadata invariants on an oracle case.
///
/// The comparison entry points call this function before any numerical result is
/// emitted. It does not validate the authority or provenance of an oracle label.
fn validate_astrodynamics_oracle_case(
    case: &AstrodynamicsOracleCase,
) -> Result<(), OracleComparisonError> {
    validate_case_id(&case.case_id)?;
    validate_required_text(
        "source_oracle_label",
        &case.source_oracle_label,
        OracleComparisonError::MissingSourceOracleLabel,
    )?;
    validate_required_text(
        "frame_context",
        &case.frame_context,
        OracleComparisonError::MissingFrameContext,
    )?;
    validate_required_text(
        "epoch_reference",
        &case.epoch_reference,
        OracleComparisonError::MissingEpochReference,
    )?;
    validate_required_text(
        "input_units",
        &case.input_units,
        OracleComparisonError::MissingInputUnits,
    )?;
    validate_required_text(
        "output_units",
        &case.output_units,
        OracleComparisonError::MissingOutputUnits,
    )?;
    validate_required_text(
        "input_summary",
        &case.input_summary,
        OracleComparisonError::MissingInputSummary,
    )?;
    validate_required_text(
        "expected_output_summary",
        &case.expected_output_summary,
        OracleComparisonError::MissingExpectedOutputSummary,
    )?;
    if case.epoch.time_scale() != case.time_scale {
        return Err(OracleComparisonError::TimeScaleMismatch {
            declared: case.time_scale,
            epoch: case.epoch.time_scale(),
        });
    }
    Ok(())
}

/// Revalidates case, status, evidence, shape, and metric consistency.
///
/// This is intended for defensive checks before a future persistence layer uses
/// a record. It does not calculate or verify an evidence digest.
pub fn validate_astrodynamics_oracle_record(
    record: &AstrodynamicsOracleRecord,
) -> Result<(), OracleComparisonError> {
    validate_astrodynamics_oracle_case(&record.case)?;
    validate_tolerance(record.tolerance)?;
    record.evidence_hash.validate()?;

    match (
        record.evidence_hash.is_pending(),
        record.status,
        record.metrics,
    ) {
        (true, OracleComparisonStatus::BlockedPendingEvidenceHash, None) => Ok(()),
        (true, _, _) => Err(OracleComparisonError::InconsistentRecord {
            reason: "pending evidence must have blocked status and no metrics",
        }),
        (false, OracleComparisonStatus::BlockedPendingEvidenceHash, _) => {
            Err(OracleComparisonError::InconsistentRecord {
                reason: "blocked-pending status requires pending evidence",
            })
        }
        (
            false,
            status @ (OracleComparisonStatus::PassedWithinTolerance
            | OracleComparisonStatus::FailedOutsideTolerance),
            Some(metrics),
        ) => validate_metrics(record.shape, status, metrics),
        (false, _, None) => Err(OracleComparisonError::InconsistentRecord {
            reason: "completed comparison status requires metrics",
        }),
    }
}

/// Compares one scalar only after validating all case context and evidence policy.
///
/// `actual` and `expected` use the case's declared output units. A pending
/// evidence hash returns a blocked record after finite/overflow checks; it never
/// returns a pass/fail status.
pub fn compare_recorded_scalar_case(
    case: &AstrodynamicsOracleCase,
    actual: f64,
    expected: f64,
    tolerance: AstrodynamicsTolerance,
    evidence_hash: OracleEvidenceHash,
) -> Result<AstrodynamicsOracleRecord, OracleComparisonError> {
    compare_recorded_components(
        case,
        &[actual],
        &[expected],
        tolerance,
        evidence_hash,
        OracleComparisonShape::Scalar,
    )
}

/// Compares a three-component vector after validating all case context.
///
/// Components are interpreted in caller-declared output units and in the case's
/// frame/epoch context. The function performs no transform. All components are
/// checked in index order; metrics retain the first failing index and the index
/// with the largest absolute error.
pub fn compare_recorded_vector_case(
    case: &AstrodynamicsOracleCase,
    actual: [f64; VECTOR_COMPONENT_COUNT],
    expected: [f64; VECTOR_COMPONENT_COUNT],
    tolerance: AstrodynamicsTolerance,
    evidence_hash: OracleEvidenceHash,
) -> Result<AstrodynamicsOracleRecord, OracleComparisonError> {
    compare_recorded_components(
        case,
        &actual,
        &expected,
        tolerance,
        evidence_hash,
        OracleComparisonShape::Vector3,
    )
}

fn compare_recorded_components(
    case: &AstrodynamicsOracleCase,
    actual: &[f64],
    expected: &[f64],
    tolerance: AstrodynamicsTolerance,
    evidence_hash: OracleEvidenceHash,
    shape: OracleComparisonShape,
) -> Result<AstrodynamicsOracleRecord, OracleComparisonError> {
    validate_astrodynamics_oracle_case(case)?;
    validate_tolerance(tolerance)?;

    if actual.len() != shape.component_count() || expected.len() != shape.component_count() {
        return Err(OracleComparisonError::InconsistentRecord {
            reason: "comparison array length does not match shape",
        });
    }

    let metrics = compare_components(actual, expected, tolerance)?;
    let (status, stored_metrics) = if evidence_hash.is_pending() {
        (OracleComparisonStatus::BlockedPendingEvidenceHash, None)
    } else if metrics.first_failing_component.is_some() {
        (
            OracleComparisonStatus::FailedOutsideTolerance,
            Some(metrics),
        )
    } else {
        (OracleComparisonStatus::PassedWithinTolerance, Some(metrics))
    };

    let record = AstrodynamicsOracleRecord {
        case: case.clone(),
        tolerance,
        evidence_hash,
        shape,
        status,
        metrics: stored_metrics,
    };
    validate_astrodynamics_oracle_record(&record)?;
    Ok(record)
}

fn compare_components(
    actual: &[f64],
    expected: &[f64],
    tolerance: AstrodynamicsTolerance,
) -> Result<OracleComparisonMetrics, OracleComparisonError> {
    let mut first_failing_component = None;
    let mut largest_absolute_error_component = 0;
    let mut largest_absolute_error = 0.0;
    let mut allowed_error_at_largest_component = 0.0;

    for (component_index, (&actual_value, &expected_value)) in
        actual.iter().zip(expected.iter()).enumerate()
    {
        let evaluation =
            compare_component(actual_value, expected_value, tolerance, component_index)?;

        if !evaluation.within_tolerance && first_failing_component.is_none() {
            first_failing_component = Some(component_index);
        }
        if component_index == 0 || evaluation.absolute_error > largest_absolute_error {
            largest_absolute_error_component = component_index;
            largest_absolute_error = evaluation.absolute_error;
            allowed_error_at_largest_component = evaluation.allowed_error;
        }
    }

    Ok(OracleComparisonMetrics {
        component_count: actual.len(),
        first_failing_component,
        largest_absolute_error_component,
        largest_absolute_error,
        allowed_error_at_largest_component,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ComponentEvaluation {
    absolute_error: f64,
    allowed_error: f64,
    within_tolerance: bool,
}

fn compare_component(
    actual: f64,
    expected: f64,
    tolerance: AstrodynamicsTolerance,
    component_index: usize,
) -> Result<ComponentEvaluation, OracleComparisonError> {
    if !actual.is_finite() {
        return Err(OracleComparisonError::NonfiniteComparisonValue {
            series: "actual",
            component_index,
        });
    }
    if !expected.is_finite() {
        return Err(OracleComparisonError::NonfiniteComparisonValue {
            series: "expected",
            component_index,
        });
    }

    let relative_allowance = tolerance.relative * expected.abs();
    if !relative_allowance.is_finite() {
        return Err(OracleComparisonError::AllowedToleranceOverflow { component_index });
    }
    let allowed_error = tolerance.absolute + relative_allowance;
    if !allowed_error.is_finite() {
        return Err(OracleComparisonError::AllowedToleranceOverflow { component_index });
    }

    let difference = actual - expected;
    if !difference.is_finite() {
        return Err(OracleComparisonError::AbsoluteDifferenceOverflow { component_index });
    }
    let absolute_error = difference.abs();

    Ok(ComponentEvaluation {
        absolute_error,
        allowed_error,
        within_tolerance: absolute_error <= allowed_error,
    })
}

fn validate_case_id(case_id: &str) -> Result<(), OracleComparisonError> {
    if case_id.trim().is_empty() {
        return Err(OracleComparisonError::MissingCaseId);
    }
    if case_id != case_id.trim()
        || !case_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':'))
    {
        return Err(OracleComparisonError::InvalidCaseId);
    }
    Ok(())
}

fn validate_required_text(
    field: &'static str,
    value: &str,
    missing_error: OracleComparisonError,
) -> Result<(), OracleComparisonError> {
    if value.trim().is_empty() {
        return Err(missing_error);
    }
    if value != value.trim() || value.chars().any(char::is_control) {
        return Err(OracleComparisonError::InvalidText { field });
    }
    Ok(())
}

fn validate_tolerance(tolerance: AstrodynamicsTolerance) -> Result<(), OracleComparisonError> {
    validate_tolerance_values(tolerance.absolute, tolerance.relative)
}

fn validate_tolerance_values(absolute: f64, relative: f64) -> Result<(), OracleComparisonError> {
    if !absolute.is_finite() || absolute < 0.0 {
        return Err(OracleComparisonError::InvalidTolerance {
            component: "absolute",
        });
    }
    if !relative.is_finite() || relative < 0.0 {
        return Err(OracleComparisonError::InvalidTolerance {
            component: "relative",
        });
    }
    Ok(())
}

fn validate_sha256_lower_hex(value: &str) -> Result<(), OracleComparisonError> {
    if value.len() != SHA256_COUNT {
        return Err(OracleComparisonError::InvalidEvidenceHash {
            reason: "SHA256 hex must contain exactly 64 ASCII characters",
        });
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(OracleComparisonError::InvalidEvidenceHash {
            reason: "SHA256 hex must use lowercase hexadecimal characters only",
        });
    }
    if value.bytes().all(|byte| byte == b'0') {
        return Err(OracleComparisonError::InvalidEvidenceHash {
            reason: "all-zero placeholder digest is not accepted",
        });
    }
    Ok(())
}

fn validate_metrics(
    shape: OracleComparisonShape,
    status: OracleComparisonStatus,
    metrics: OracleComparisonMetrics,
) -> Result<(), OracleComparisonError> {
    let component_count = shape.component_count();
    if metrics.component_count != component_count {
        return Err(OracleComparisonError::InconsistentRecord {
            reason: "metric component count does not match comparison shape",
        });
    }
    if metrics.largest_absolute_error_component >= component_count {
        return Err(OracleComparisonError::InconsistentRecord {
            reason: "largest-error component index is outside comparison shape",
        });
    }
    if !metrics.largest_absolute_error.is_finite() || metrics.largest_absolute_error < 0.0 {
        return Err(OracleComparisonError::InconsistentRecord {
            reason: "largest absolute error must be finite and nonnegative",
        });
    }
    if !metrics.allowed_error_at_largest_component.is_finite()
        || metrics.allowed_error_at_largest_component < 0.0
    {
        return Err(OracleComparisonError::InconsistentRecord {
            reason: "allowed error must be finite and nonnegative",
        });
    }
    if metrics
        .first_failing_component
        .is_some_and(|index| index >= component_count)
    {
        return Err(OracleComparisonError::InconsistentRecord {
            reason: "first-failing component index is outside comparison shape",
        });
    }

    match (status, metrics.first_failing_component) {
        (OracleComparisonStatus::PassedWithinTolerance, None) => Ok(()),
        (OracleComparisonStatus::FailedOutsideTolerance, Some(_)) => Ok(()),
        (OracleComparisonStatus::PassedWithinTolerance, Some(_)) => {
            Err(OracleComparisonError::InconsistentRecord {
                reason: "passed status cannot contain a failing component",
            })
        }
        (OracleComparisonStatus::FailedOutsideTolerance, None) => {
            Err(OracleComparisonError::InconsistentRecord {
                reason: "failed status requires a failing component",
            })
        }
        (OracleComparisonStatus::BlockedPendingEvidenceHash, _) => {
            Err(OracleComparisonError::InconsistentRecord {
                reason: "blocked status cannot contain comparison metrics",
            })
        }
    }
}

fn format_optional_index(index: Option<usize>) -> String {
    index.map_or_else(|| "none".to_owned(), |value| value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_draft() -> AstrodynamicsOracleCaseDraft {
        AstrodynamicsOracleCaseDraft {
            case_id: "synthetic.oracle.scalar.001".to_owned(),
            source_oracle_label: "independent synthetic analytical check".to_owned(),
            frame: Some(AstroFrame::InertialEciMeanEquator),
            time_scale: Some(AstroTimeScale::Tai),
            epoch: Some(AstroEpoch::new(AstroTimeScale::Tai, 123.25).unwrap()),
            frame_context: "synthetic inertial label; no coordinate transform requested".to_owned(),
            epoch_reference: "synthetic scalar-zero reference selected by the test".to_owned(),
            input_units: "m; m/s; s".to_owned(),
            output_units: "m".to_owned(),
            input_summary: "synthetic finite state and duration".to_owned(),
            expected_output_summary: "synthetic scalar distance".to_owned(),
        }
    }

    fn complete_case() -> AstrodynamicsOracleCase {
        AstrodynamicsOracleCase::try_from_draft(complete_draft()).unwrap()
    }

    fn tolerance(absolute: f64, relative: f64) -> AstrodynamicsTolerance {
        AstrodynamicsTolerance::new(absolute, relative).unwrap()
    }

    fn valid_hash() -> OracleEvidenceHash {
        OracleEvidenceHash::sha256_lower_hex("1a".repeat(32)).unwrap()
    }

    #[test]
    fn all_status_and_shape_codes_are_stable_and_unique() {
        let status_codes: Vec<_> = OracleComparisonStatus::ALLOWED
            .iter()
            .map(|status| status.code())
            .collect();
        assert_eq!(status_codes.len(), 3);
        assert_eq!(
            status_codes,
            vec![
                "passed_within_tolerance",
                "failed_outside_tolerance",
                "blocked_pending_evidence_hash",
            ]
        );

        let shape_codes: Vec<_> = OracleComparisonShape::ALLOWED
            .iter()
            .map(|shape| shape.code())
            .collect();
        assert_eq!(shape_codes, vec!["scalar", "vector3"]);
    }

    #[test]
    fn complete_case_validates_and_exposes_read_only_context() {
        let case = complete_case();
        validate_astrodynamics_oracle_case(&case).unwrap();
        let view = case.view();
        assert_eq!(view.case_id, "synthetic.oracle.scalar.001");
        assert_eq!(view.frame, AstroFrame::InertialEciMeanEquator);
        assert_eq!(view.time_scale, AstroTimeScale::Tai);
        assert_eq!(view.epoch.time_scale(), AstroTimeScale::Tai);
        assert_eq!(
            view.frame_context,
            "synthetic inertial label; no coordinate transform requested"
        );
        assert_eq!(
            view.epoch_reference,
            "synthetic scalar-zero reference selected by the test"
        );
        assert_eq!(view.output_units, "m");
    }

    #[test]
    fn missing_case_id_is_rejected() {
        let mut draft = complete_draft();
        draft.case_id = "  ".to_owned();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(draft),
            Err(OracleComparisonError::MissingCaseId)
        );
    }

    #[test]
    fn invalid_case_id_characters_and_outer_whitespace_are_rejected() {
        for invalid in ["synthetic case", "synthetic/case", " synthetic.case"] {
            let mut draft = complete_draft();
            draft.case_id = invalid.to_owned();
            assert_eq!(
                AstrodynamicsOracleCase::try_from_draft(draft),
                Err(OracleComparisonError::InvalidCaseId)
            );
        }
    }

    #[test]
    fn missing_source_label_is_rejected() {
        let mut draft = complete_draft();
        draft.source_oracle_label = "\t".to_owned();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(draft),
            Err(OracleComparisonError::MissingSourceOracleLabel)
        );
    }

    #[test]
    fn missing_frame_time_scale_and_epoch_are_distinct_errors() {
        let mut missing_frame = complete_draft();
        missing_frame.frame = None;
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(missing_frame),
            Err(OracleComparisonError::MissingFrame)
        );

        let mut missing_scale = complete_draft();
        missing_scale.time_scale = None;
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(missing_scale),
            Err(OracleComparisonError::MissingTimeScale)
        );

        let mut missing_epoch = complete_draft();
        missing_epoch.epoch = None;
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(missing_epoch),
            Err(OracleComparisonError::MissingEpoch)
        );
    }

    #[test]
    fn declared_and_epoch_time_scales_must_match() {
        let mut draft = complete_draft();
        draft.time_scale = Some(AstroTimeScale::Tt);
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(draft),
            Err(OracleComparisonError::TimeScaleMismatch {
                declared: AstroTimeScale::Tt,
                epoch: AstroTimeScale::Tai,
            })
        );
    }

    #[test]
    fn missing_frame_context_and_epoch_reference_are_rejected() {
        let mut frame_context = complete_draft();
        frame_context.frame_context.clear();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(frame_context),
            Err(OracleComparisonError::MissingFrameContext)
        );

        let mut epoch_reference = complete_draft();
        epoch_reference.epoch_reference = "   ".to_owned();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(epoch_reference),
            Err(OracleComparisonError::MissingEpochReference)
        );
    }

    #[test]
    fn missing_or_whitespace_only_unit_contracts_are_rejected() {
        let mut input_units = complete_draft();
        input_units.input_units.clear();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(input_units),
            Err(OracleComparisonError::MissingInputUnits)
        );

        let mut output_units = complete_draft();
        output_units.output_units = "   ".to_owned();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(output_units),
            Err(OracleComparisonError::MissingOutputUnits)
        );
    }

    #[test]
    fn missing_input_and_expected_output_summaries_are_rejected() {
        let mut input_summary = complete_draft();
        input_summary.input_summary.clear();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(input_summary),
            Err(OracleComparisonError::MissingInputSummary)
        );

        let mut output_summary = complete_draft();
        output_summary.expected_output_summary = "\n".to_owned();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(output_summary),
            Err(OracleComparisonError::MissingExpectedOutputSummary)
        );
    }

    #[test]
    fn outer_whitespace_and_control_characters_are_rejected() {
        let mut outer_whitespace = complete_draft();
        outer_whitespace.input_summary = " synthetic input".to_owned();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(outer_whitespace),
            Err(OracleComparisonError::InvalidText {
                field: "input_summary",
            })
        );

        let mut control = complete_draft();
        control.expected_output_summary = "synthetic\u{7f}output".to_owned();
        assert_eq!(
            AstrodynamicsOracleCase::try_from_draft(control),
            Err(OracleComparisonError::InvalidText {
                field: "expected_output_summary",
            })
        );
    }

    #[test]
    fn zero_zero_tolerance_is_an_explicit_exact_comparison() {
        let case = complete_case();
        let exact = tolerance(0.0, 0.0);
        assert_eq!(exact.values(), (0.0, 0.0));

        let passed = compare_recorded_scalar_case(
            &case,
            8.0,
            8.0,
            exact,
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        assert_eq!(
            passed.view().status,
            OracleComparisonStatus::PassedWithinTolerance
        );

        let failed = compare_recorded_scalar_case(
            &case,
            f64::from_bits(8.0_f64.to_bits() + 1),
            8.0,
            exact,
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        assert_eq!(
            failed.view().status,
            OracleComparisonStatus::FailedOutsideTolerance
        );
    }

    #[test]
    fn invalid_tolerance_matrix_is_rejected() {
        let cases = [
            (-1.0, 0.0, "absolute"),
            (f64::NAN, 0.0, "absolute"),
            (f64::INFINITY, 0.0, "absolute"),
            (0.0, -1.0, "relative"),
            (0.0, f64::NAN, "relative"),
            (0.0, f64::INFINITY, "relative"),
        ];

        for (absolute, relative, component) in cases {
            assert_eq!(
                AstrodynamicsTolerance::new(absolute, relative),
                Err(OracleComparisonError::InvalidTolerance { component })
            );
        }
    }

    #[test]
    fn complete_scalar_case_passes_within_absolute_and_relative_tolerance() {
        let record = compare_recorded_scalar_case(
            &complete_case(),
            100.6,
            100.0,
            tolerance(0.1, 0.005),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        let view = record.view();
        assert_eq!(view.shape, OracleComparisonShape::Scalar);
        assert_eq!(view.status, OracleComparisonStatus::PassedWithinTolerance);
        let metrics = view.metrics.unwrap();
        assert_eq!(metrics.component_count, 1);
        assert_eq!(metrics.first_failing_component, None);
        assert!((metrics.allowed_error_at_largest_component - 0.6).abs() < 1.0e-12);
    }

    #[test]
    fn scalar_outside_tolerance_fails_with_deterministic_metrics() {
        let record = compare_recorded_scalar_case(
            &complete_case(),
            10.2,
            10.0,
            tolerance(0.1, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        let view = record.view();
        assert_eq!(view.status, OracleComparisonStatus::FailedOutsideTolerance);
        let metrics = view.metrics.unwrap();
        assert_eq!(metrics.first_failing_component, Some(0));
        assert_eq!(metrics.largest_absolute_error_component, 0);
    }

    #[test]
    fn negative_expected_value_uses_absolute_expected_for_relative_tolerance() {
        let record = compare_recorded_scalar_case(
            &complete_case(),
            -99.0,
            -100.0,
            tolerance(0.0, 0.01),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        let view = record.view();
        assert_eq!(view.status, OracleComparisonStatus::PassedWithinTolerance);
        let metrics = view.metrics.unwrap();
        assert_eq!(metrics.first_failing_component, None);
        assert!((metrics.allowed_error_at_largest_component - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn signed_zero_and_subnormal_values_are_deterministic_finite_inputs() {
        let signed_zero = compare_recorded_scalar_case(
            &complete_case(),
            -0.0,
            0.0,
            tolerance(0.0, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        assert_eq!(
            signed_zero.view().status,
            OracleComparisonStatus::PassedWithinTolerance
        );

        let subnormal = compare_recorded_scalar_case(
            &complete_case(),
            f64::MIN_POSITIVE / 2.0,
            0.0,
            tolerance(f64::MIN_POSITIVE, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        assert_eq!(
            subnormal.view().status,
            OracleComparisonStatus::PassedWithinTolerance
        );
    }

    #[test]
    fn vector_comparison_checks_every_component_and_tracks_two_indices() {
        let record = compare_recorded_vector_case(
            &complete_case(),
            [1.0, 2.2, 3.5],
            [1.0, 2.0, 3.0],
            tolerance(0.1, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        let metrics = record.view().metrics.unwrap();
        assert_eq!(
            record.view().status,
            OracleComparisonStatus::FailedOutsideTolerance
        );
        assert_eq!(metrics.component_count, 3);
        assert_eq!(metrics.first_failing_component, Some(1));
        assert_eq!(metrics.largest_absolute_error_component, 2);
        assert!((metrics.largest_absolute_error - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn vector_validation_does_not_hide_a_later_nonfinite_component() {
        let error = compare_recorded_vector_case(
            &complete_case(),
            [100.0, 2.0, f64::NAN],
            [0.0, 2.0, 3.0],
            tolerance(0.0, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap_err();
        assert_eq!(
            error,
            OracleComparisonError::NonfiniteComparisonValue {
                series: "actual",
                component_index: 2,
            }
        );
    }

    #[test]
    fn nonfinite_actual_and_expected_values_are_rejected() {
        assert_eq!(
            compare_recorded_scalar_case(
                &complete_case(),
                f64::INFINITY,
                1.0,
                tolerance(0.0, 0.0),
                OracleEvidenceHash::synthetic_no_external_fixture(),
            ),
            Err(OracleComparisonError::NonfiniteComparisonValue {
                series: "actual",
                component_index: 0,
            })
        );
        assert_eq!(
            compare_recorded_scalar_case(
                &complete_case(),
                1.0,
                f64::NAN,
                tolerance(0.0, 0.0),
                OracleEvidenceHash::synthetic_no_external_fixture(),
            ),
            Err(OracleComparisonError::NonfiniteComparisonValue {
                series: "expected",
                component_index: 0,
            })
        );
    }

    #[test]
    fn relative_tolerance_product_overflow_is_blocked() {
        let error = compare_recorded_scalar_case(
            &complete_case(),
            1.0,
            f64::MAX,
            tolerance(0.0, 2.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap_err();
        assert_eq!(
            error,
            OracleComparisonError::AllowedToleranceOverflow { component_index: 0 }
        );
    }

    #[test]
    fn absolute_plus_relative_tolerance_sum_overflow_is_blocked() {
        let error = compare_recorded_scalar_case(
            &complete_case(),
            f64::MAX / 2.0,
            f64::MAX / 2.0,
            tolerance(f64::MAX, 1.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap_err();
        assert_eq!(
            error,
            OracleComparisonError::AllowedToleranceOverflow { component_index: 0 }
        );
    }

    #[test]
    fn absolute_difference_overflow_is_blocked() {
        let error = compare_recorded_scalar_case(
            &complete_case(),
            f64::MAX,
            -f64::MAX,
            tolerance(0.0, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap_err();
        assert_eq!(
            error,
            OracleComparisonError::AbsoluteDifferenceOverflow { component_index: 0 }
        );
    }

    #[test]
    fn pending_evidence_hash_has_deterministic_blocked_behavior() {
        let record = compare_recorded_scalar_case(
            &complete_case(),
            1.0,
            1.0,
            tolerance(0.0, 0.0),
            OracleEvidenceHash::pending(),
        )
        .unwrap();
        let view = record.view();
        assert_eq!(
            view.status,
            OracleComparisonStatus::BlockedPendingEvidenceHash
        );
        assert_eq!(view.evidence_hash.code(), "pending");
        assert_eq!(view.metrics, None);
        validate_astrodynamics_oracle_record(&record).unwrap();
    }

    #[test]
    fn evidence_hash_format_rejects_length_case_nonhex_and_placeholder() {
        let cases = [
            "1a".repeat(31),
            "A1".repeat(32),
            "g1".repeat(32),
            "0".repeat(64),
        ];
        for value in cases {
            assert!(matches!(
                OracleEvidenceHash::sha256_lower_hex(value),
                Err(OracleComparisonError::InvalidEvidenceHash { .. })
            ));
        }
    }

    #[test]
    fn valid_sha256_metadata_allows_a_comparison_without_calculating_a_hash() {
        let record = compare_recorded_scalar_case(
            &complete_case(),
            5.0,
            5.0,
            tolerance(0.0, 0.0),
            valid_hash(),
        )
        .unwrap();
        assert_eq!(record.view().evidence_hash.code(), "sha256_lower_hex");
        assert_eq!(
            record.view().evidence_hash.sha256_value(),
            Some("1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a")
        );
        assert_eq!(
            record.view().status,
            OracleComparisonStatus::PassedWithinTolerance
        );
        assert_eq!(
            OracleEvidenceHash::synthetic_no_external_fixture().sha256_value(),
            None
        );
        assert_eq!(OracleEvidenceHash::pending().sha256_value(), None);
    }

    #[test]
    fn pending_evidence_does_not_mask_invalid_numerical_input() {
        assert_eq!(
            compare_recorded_scalar_case(
                &complete_case(),
                f64::NAN,
                1.0,
                tolerance(0.0, 0.0),
                OracleEvidenceHash::pending(),
            ),
            Err(OracleComparisonError::NonfiniteComparisonValue {
                series: "actual",
                component_index: 0,
            })
        );
    }

    #[test]
    fn label_only_frame_and_time_scale_are_context_not_transform_claims() {
        let mut draft = complete_draft();
        draft.frame = Some(AstroFrame::TemeLabelOnly);
        draft.time_scale = Some(AstroTimeScale::TdbLabelOnly);
        draft.epoch = Some(AstroEpoch::new(AstroTimeScale::TdbLabelOnly, 0.0).unwrap());
        let case = AstrodynamicsOracleCase::try_from_draft(draft).unwrap();
        let record = compare_recorded_scalar_case(
            &case,
            1.0,
            1.0,
            tolerance(0.0, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        assert_eq!(record.view().case.frame, AstroFrame::TemeLabelOnly);
        assert_eq!(record.view().case.time_scale, AstroTimeScale::TdbLabelOnly);
    }

    #[test]
    fn record_validator_rejects_inconsistent_status_metrics_and_evidence() {
        let mut record = compare_recorded_scalar_case(
            &complete_case(),
            1.0,
            1.0,
            tolerance(0.0, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        record.status = OracleComparisonStatus::FailedOutsideTolerance;
        assert_eq!(
            validate_astrodynamics_oracle_record(&record),
            Err(OracleComparisonError::InconsistentRecord {
                reason: "failed status requires a failing component",
            })
        );

        let mut pending = compare_recorded_scalar_case(
            &complete_case(),
            1.0,
            1.0,
            tolerance(0.0, 0.0),
            OracleEvidenceHash::pending(),
        )
        .unwrap();
        pending.status = OracleComparisonStatus::PassedWithinTolerance;
        assert_eq!(
            validate_astrodynamics_oracle_record(&pending),
            Err(OracleComparisonError::InconsistentRecord {
                reason: "pending evidence must have blocked status and no metrics",
            })
        );
    }

    #[test]
    fn report_is_stable_contains_context_and_avoids_prohibited_claim_terms() {
        let record = compare_recorded_vector_case(
            &complete_case(),
            [1.0, 2.0, 3.0],
            [1.0, 2.0, 3.0],
            tolerance(0.0, 0.0),
            OracleEvidenceHash::synthetic_no_external_fixture(),
        )
        .unwrap();
        let first = record.to_string();
        let second = record.to_string();
        assert_eq!(first, second);
        assert!(first.contains("case_id=\"synthetic.oracle.scalar.001\""));
        assert!(first.contains("frame=ECI-mean-equator(label)"));
        assert!(first.contains(
            "frame_context=\"synthetic inertial label; no coordinate transform requested\""
        ));
        assert!(first.contains("time_scale=TAI"));
        assert!(first
            .contains("epoch_reference=\"synthetic scalar-zero reference selected by the test\""));
        assert!(first.contains("shape=vector3"));
        assert!(first.contains("status=passed_within_tolerance"));
        assert!(first.contains("scope=research_required"));

        let lowercase = first.to_ascii_lowercase();
        for prohibited in [
            "certified",
            "certification",
            "operational approval",
            "mission readiness",
            "flight readiness",
        ] {
            assert!(!lowercase.contains(prohibited));
        }
    }
}
